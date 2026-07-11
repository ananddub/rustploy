use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, BuildStrategy, SourceSpec},
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
                self.docker
                    .image_build_cancelled(
                        &[
                            "--build-arg",
                            format!(
                                "BUILDKIT_SYNTAX=ghcr.io/railwayapp/railpack-frontend:v{version}"
                            )
                            .as_str(),
                            "--file",
                            plan.as_str(),
                            "--tag",
                            spec.image.as_str(),
                            spec.work_directory.as_str(),
                        ],
                        cancel,
                    )
                    .await?;
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
            format!("{}/{dockerfile}", spec.work_directory),
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

        args.push(format!(
            "{}/{}",
            spec.work_directory,
            context.trim_start_matches('/')
        ));
        let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
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
        self.docker
            .image_build_cancelled(
                &[
                    "--tag",
                    spec.image.as_str(),
                    "--file",
                    format!("{}/Dockerfile.rustploy", spec.work_directory).as_str(),
                    spec.work_directory.as_str(),
                ],
                cancel,
            )
            .await
            .map(|_| ())
    }
}

const SPA_NGINX: &str = r#"events { worker_connections 1024; }
http { include mime.types; server { listen 80; root /usr/share/nginx/html; index index.html; location / { try_files $uri $uri/ /index.html; } } }
"#;
