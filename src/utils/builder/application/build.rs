use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, BuildStrategy, BuilderEvent, SourceSpec},
    exec::{ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn build_image(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if matches!(spec.source, SourceSpec::Docker { .. }) {
            return Ok(());
        }

        let Some(strategy) = &spec.build else {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "build strategy is required for non-Docker source".into(),
            });
        };

        match strategy {
            BuildStrategy::Dockerfile {
                dockerfile,
                context,
                target,
                no_cache,
            } => {
                self.build_dockerfile(spec, dockerfile, context, target, *no_cache, cancel)
                    .await?
            }
            BuildStrategy::Nixpacks => {
                self.emit(BuilderEvent::Message(format!(
                    "building image {} with nixpacks from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                self.executor
                    .run_cancelled(
                        "nixpacks",
                        [
                            "build",
                            spec.work_directory.as_str(),
                            "--name",
                            spec.image.as_str(),
                        ],
                        cancel,
                    )
                    .await?;
            }
            BuildStrategy::Paketo => {
                self.emit(BuilderEvent::Message(format!(
                    "building image {} with Paketo from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                self.executor
                    .run_cancelled(
                        "pack",
                        [
                            "build",
                            spec.image.as_str(),
                            "--path",
                            spec.work_directory.as_str(),
                            "--builder",
                            "paketobuildpacks/builder-jammy-full",
                        ],
                        cancel,
                    )
                    .await?;
            }
            BuildStrategy::Railpack { version } => {
                self.emit(BuilderEvent::Message(format!(
                    "building image {} with railpack {version} from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                let plan = format!("{}/railpack-plan.json", spec.work_directory);
                self.executor
                    .run_cancelled(
                        "railpack",
                        [
                            "prepare",
                            spec.work_directory.as_str(),
                            "--plan-out",
                            plan.as_str(),
                        ],
                        cancel,
                    )
                    .await?;
                let args = railpack_build_args(spec, version, &plan)?;
                let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
                self.docker.image_build_cancelled(&refs, cancel).await?;
            }
            BuildStrategy::Static {
                publish_directory,
                spa,
            } => {
                self.build_static(spec, publish_directory, *spa, cancel)
                    .await?
            }
        }
        Ok(())
    }

    async fn build_dockerfile(
        &self,
        spec: &ApplicationSpec,
        dockerfile: &str,
        context: &str,
        target: &Option<String>,
        no_cache: bool,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let mut args = vec![
            "build".to_owned(),
            "--tag".into(),
            spec.image.clone(),
            "--file".into(),
            join_path(&spec.work_directory, default_if_empty(dockerfile, "Dockerfile")),
        ];
        if let Some(target) = target {
            args.extend(["--target".into(), target.clone()]);
        }
        if no_cache {
            args.push("--no-cache".into());
        }
        for (key, value) in &spec.build_args {
            args.extend(["--build-arg".into(), format!("{key}={value}")]);
        }

        let secret_dir = format!("/tmp/rustploy-secrets-{}", spec.app_name);
        if !spec.build_secrets.is_empty() {
            self.executor
                .run("mkdir", ["-p", secret_dir.as_str()])
                .await?;
        }
        for (key, value) in &spec.build_secrets {
            let path = format!("{secret_dir}/{key}");
            self.write_file_cancelled(&path, value.as_bytes(), cancel)
                .await?;
            args.extend(["--secret".into(), format!("id={key},src={path}")]);
        }

        args.push(join_path(&spec.work_directory, default_if_empty(context, ".")));
        let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        validate_build_context(refs.last().copied())?;
        self.emit(BuilderEvent::Message(format!(
            "docker build image {} using dockerfile {} and context {}",
            spec.image,
            join_path(&spec.work_directory, default_if_empty(dockerfile, "Dockerfile")),
            refs.last().copied().unwrap_or("")
        )))
        .await;
        tracing::info!(image = %spec.image, args = ?args, "running docker image build");
        let result = self.docker.image_build_cancelled(&refs, cancel).await;
        let _ = self.executor.run("rm", ["-rf", secret_dir.as_str()]).await;
        result.map(|_| ())
    }

    async fn build_static(
        &self,
        spec: &ApplicationSpec,
        publish_directory: &str,
        spa: bool,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let dockerfile = format!(
            "FROM nginx:alpine\nWORKDIR /usr/share/nginx/html\n{}COPY {} .\nCMD [\"nginx\",\"-g\",\"daemon off;\"]\n",
            if spa {
                "COPY nginx.conf /etc/nginx/nginx.conf\n"
            } else {
                ""
            },
            publish_directory
        );
        self.write_file_cancelled(
            &format!("{}/Dockerfile.rustploy", spec.work_directory),
            dockerfile.as_bytes(),
            cancel,
        )
        .await?;
        if spa {
            self.write_file_cancelled(
                &format!("{}/nginx.conf", spec.work_directory),
                SPA_NGINX.as_bytes(),
                cancel,
            )
            .await?;
        }
        let args = static_build_args(spec)?;
        let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        self.docker
            .image_build_cancelled(&refs, cancel)
            .await
            .map(|_| ())
    }
}

fn static_build_args(spec: &ApplicationSpec) -> ExecResult<Vec<String>> {
    let args = vec![
        "--tag".into(),
        spec.image.clone(),
        "--file".into(),
        format!("{}/Dockerfile.rustploy", spec.work_directory),
        spec.work_directory.clone(),
    ];
    validate_build_context(args.last().map(String::as_str))?;
    tracing::info!(image = %spec.image, args = ?args, "running docker static image build");
    Ok(args)
}

fn railpack_build_args(
    spec: &ApplicationSpec,
    version: &str,
    plan: &str,
) -> ExecResult<Vec<String>> {
    let args = vec![
        "--build-arg".into(),
        format!("BUILDKIT_SYNTAX=ghcr.io/railwayapp/railpack-frontend:v{version}"),
        "--file".into(),
        plan.into(),
        "--tag".into(),
        spec.image.clone(),
        spec.work_directory.clone(),
    ];
    validate_build_context(args.last().map(String::as_str))?;
    tracing::info!(image = %spec.image, args = ?args, "running docker railpack image build");
    Ok(args)
}

fn default_if_empty<'a>(value: &'a str, default: &'a str) -> &'a str {
    let value = value.trim();
    if value.is_empty() { default } else { value }
}

fn join_path(base: &str, child: &str) -> String {
    let child = child.trim();
    if child == "." {
        return base.trim_end_matches('/').into();
    }
    if child.starts_with('/') {
        child.into()
    } else {
        format!("{}/{}", base.trim_end_matches('/'), child)
    }
}

fn validate_build_context(value: Option<&str>) -> ExecResult<()> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: "docker build context path resolved empty".into(),
        });
    };
    if value == "-" {
        return Ok(());
    }
    Ok(())
}

const SPA_NGINX: &str = r#"events { worker_connections 1024; }
http { include mime.types; server { listen 80; root /usr/share/nginx/html; index index.html; location / { try_files $uri $uri/ /index.html; } } }
"#;
