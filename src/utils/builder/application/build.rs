use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, BuildStrategy, BuilderEvent, SourceSpec},
    exec::{ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;
use crate::utils::builder::packs::{nixpacks::NixpacksCli, paketo::{PackCli, PaketoBuilderImage}, railpack::RailpackCli, heroku::{HerokuCli, HerokuBuilderImage}};

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
                self.ctx.emit(BuilderEvent::Message(format!(
                    "building image {} with nixpacks from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                let cli = NixpacksCli::new(&self.ctx.executor);
                let mut builder = cli
                    .build(spec.work_directory.as_str())
                    .name(spec.image.as_str());
                for (k, v) in &spec.build_args {
                    builder = builder.env(k, v);
                }
                builder.run(cancel).await?;
            }
            BuildStrategy::Paketo => {
                self.ctx.emit(BuilderEvent::Message(format!(
                    "building image {} with Paketo from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                let cli = PackCli::new(&self.ctx.executor);
                let mut builder = cli
                    .build(spec.image.as_str())
                    .path(spec.work_directory.as_str())
                    .builder(PaketoBuilderImage::JammyFull);
                for (k, v) in &spec.build_args {
                    builder = builder.env(k, v);
                }
                builder.run(cancel).await?;
            }
            BuildStrategy::Heroku => {
                self.ctx.emit(BuilderEvent::Message(format!(
                    "building image {} with Heroku from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                let cli = HerokuCli::new(&self.ctx.executor);
                let mut builder = cli
                    .build(spec.image.as_str())
                    .path(spec.work_directory.as_str())
                    .builder(HerokuBuilderImage::Builder22);
                for (k, v) in &spec.build_args {
                    builder = builder.env(k, v);
                }
                builder.run(cancel).await?;
            }
            BuildStrategy::Railpack { version } => {
                self.ctx.emit(BuilderEvent::Message(format!(
                    "building image {} with railpack {version} from {}",
                    spec.image, spec.work_directory
                )))
                .await;
                let plan = format!("{}/railpack-plan.json", spec.work_directory);
                let cli = RailpackCli::new(&self.ctx.executor);
                let mut builder = cli
                    .prepare(spec.work_directory.as_str())
                    .plan_out(plan.as_str());
                for (k, v) in &spec.build_args {
                    builder = builder.env(k, v);
                }
                builder.run(cancel).await?;
                let _ = self.ctx.docker.images().build(&spec.work_directory)
                    .tag(spec.image.clone())
                    .dockerfile(&plan)
                    .build_arg("BUILDKIT_SYNTAX", format!("ghcr.io/railwayapp/railpack-frontend:v{version}"))
                    .cancel_with(cancel.clone())
                    .build()
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
        let context_path = join_path(&spec.work_directory, default_if_empty(context, "."));
        let dockerfile_path = join_path(&spec.work_directory, default_if_empty(dockerfile, "Dockerfile"));
        
        validate_build_context(Some(&context_path))?;
        
        let images = self.ctx.docker.images();
        let mut builder = images.build(context_path.clone())
            .tag(spec.image.clone())
            .dockerfile(dockerfile_path.clone());

        if let Some(t) = target {
            builder = builder.target(t.clone());
        }
        if no_cache {
            builder = builder.no_cache();
        }
        for (key, value) in &spec.build_args {
            builder = builder.build_arg(key.clone(), value.clone());
        }

        let secret_dir = format!("/tmp/rustploy-secrets-{}", spec.app_name);
        if !spec.build_secrets.is_empty() {
            self.ctx.executor
                .run("mkdir", ["-p", secret_dir.as_str()])
                .await?;
        }
        for (key, value) in &spec.build_secrets {
            let path = format!("{secret_dir}/{key}");
            self.ctx.write_file_cancelled(&path, value.as_bytes(), cancel)
                .await?;
            builder = builder.secret(format!("id={key},src={path}"));
        }

        self.ctx.emit(BuilderEvent::Message(format!(
            "docker build image {} using dockerfile {} and context {}",
            spec.image,
            dockerfile_path,
            context_path
        )))
        .await;
        
        let print_args = builder.print();
        tracing::info!(image = %spec.image, command = %print_args, "running docker image build");
        
        let result = builder
            .cancel_with(cancel.clone())
            .build()
            .await;
            
        let _ = self.ctx.executor.run("rm", ["-rf", secret_dir.as_str()]).await;
        
        result.map(|_| ()).map_err(|e| ExecError::CommandFailed {
            code: None,
            stderr: e.to_string(),
        })
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
        self.ctx.write_file_cancelled(
            &format!("{}/Dockerfile.rustploy", spec.work_directory),
            dockerfile.as_bytes(),
            cancel,
        )
        .await?;
        if spa {
            self.ctx.write_file_cancelled(
                &format!("{}/nginx.conf", spec.work_directory),
                SPA_NGINX.as_bytes(),
                cancel,
            )
            .await?;
        }
        self.ctx.docker.images().build(spec.work_directory.clone())
            .tag(spec.image.clone())
            .dockerfile(format!("{}/Dockerfile.rustploy", spec.work_directory))
            .cancel_with(cancel.clone())
            .build()
            .await
            .map(|_| ())
    }
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
