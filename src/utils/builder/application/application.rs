use super::{spec::*, traefik};
use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
    git::GitCli,
};
use serde::Serialize;
use std::collections::BTreeMap;
use tokio::{
    sync::mpsc,
    time::{Duration, Instant},
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct ApplicationBuilder {
    executor: CommandExecutor,
    docker: DockerCli,
    events: Option<mpsc::Sender<BuilderEvent>>,
    health_timeout: Duration,
}

impl ApplicationBuilder {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            docker: DockerCli::from_executor(executor.clone()),
            executor,
            events: None,
            health_timeout: Duration::from_secs(120),
        }
    }
    pub fn with_events(mut self, events: mpsc::Sender<BuilderEvent>) -> Self {
        self.events = Some(events);
        self
    }
    pub fn with_health_timeout(mut self, timeout: Duration) -> Self {
        self.health_timeout = timeout;
        self
    }

    pub async fn deploy(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<DeploymentResult> {
        validate_spec(spec)?;
        self.emit(BuilderEvent::Preparing).await;
        self.cancelled(cancel)?;
        self.prepare_source(spec).await?;
        self.emit(BuilderEvent::SourceReady).await;
        self.cancelled(cancel)?;
        self.emit(BuilderEvent::Building).await;
        self.build_image(spec).await?;
        self.emit(BuilderEvent::ImageReady).await;
        self.cancelled(cancel)?;
        let app_dir = format!("/etc/rustploy/applications/{}", spec.app_name);
        self.executor.run("mkdir", ["-p", app_dir.as_str()]).await?;
        self.prepare_file_mounts(spec).await?;
        let stack_file = format!("{app_dir}/stack.yml");
        let stack_yaml = serde_yaml::to_string(&stack_spec(spec))
            .map_err(|e| ExecError::Json(serde_json::Error::io(std::io::Error::other(e))))?;
        self.write_file(&stack_file, stack_yaml.as_bytes()).await?;
        self.emit(BuilderEvent::Deploying).await;
        self.docker
            .stack_deploy(&[
                "--compose-file",
                stack_file.as_str(),
                "--with-registry-auth",
                spec.stack_name.as_str(),
            ])
            .await?;
        self.cancelled(cancel)?;
        let traefik_file = format!("/etc/rustploy/traefik/dynamic/{}.json", spec.app_name);
        let routing = serde_json::to_vec_pretty(&traefik::application_config(spec))?;
        self.write_file(&traefik_file, &routing).await?;
        self.emit(BuilderEvent::Routing).await;
        self.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            let _ = self.executor.run("rm", ["-f", traefik_file.as_str()]).await;
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }
        self.emit(BuilderEvent::Deployed).await;
        Ok(DeploymentResult {
            app_name: spec.app_name.clone(),
            image: spec.image.clone(),
            service_name: spec.service_name(),
            stack_file,
            traefik_file,
        })
    }

    async fn prepare_source(&self, spec: &ApplicationSpec) -> ExecResult<()> {
        match &spec.source {
            SourceSpec::Docker { image, registry } => {
                if let Some(auth) = registry {
                    self.docker
                        .image_pull_authenticated(
                            &auth.registry,
                            &auth.username,
                            &auth.password,
                            image,
                            &[],
                        )
                        .await?;
                } else {
                    self.docker.image_pull(&[image]).await?;
                }
            }
            SourceSpec::Git {
                url,
                branch,
                submodules,
            } => {
                let git = GitCli::from_executor(self.executor.clone())
                    .with_repository(spec.work_directory.clone());
                let git_dir = format!("{}/.git", spec.work_directory);
                if self
                    .executor
                    .run("test", ["-d", git_dir.as_str()])
                    .await
                    .is_ok()
                {
                    git.fetch(&["--prune", "origin", branch]).await?;
                    git.reset(&["--hard", "FETCH_HEAD"]).await?;
                } else {
                    if let Some(parent) = std::path::Path::new(&spec.work_directory).parent() {
                        self.executor
                            .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                            .await?;
                    }
                    GitCli::from_executor(self.executor.clone())
                        .clone_repository(
                            url,
                            Some(&spec.work_directory),
                            &["--branch", branch, "--single-branch"],
                        )
                        .await?;
                }
                if *submodules {
                    git.submodule(&["update", "--init", "--recursive"]).await?;
                }
            }
        }
        Ok(())
    }

    async fn build_image(&self, spec: &ApplicationSpec) -> ExecResult<()> {
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
                if *no_cache {
                    args.push("--no-cache".into());
                }
                for (k, v) in &spec.build_args {
                    args.extend(["--build-arg".into(), format!("{k}={v}")]);
                }
                let secret_dir = format!("/tmp/rustploy-secrets-{}", spec.app_name);
                if !spec.build_secrets.is_empty() {
                    self.executor
                        .run("mkdir", ["-p", secret_dir.as_str()])
                        .await?;
                }
                for (k, v) in &spec.build_secrets {
                    let path = format!("{secret_dir}/{k}");
                    self.write_file(&path, v.as_bytes()).await?;
                    args.extend(["--secret".into(), format!("id={k},src={path}")]);
                }
                args.push(format!(
                    "{}/{}",
                    spec.work_directory,
                    context.trim_start_matches('/')
                ));
                let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
                let result = self.docker.image_build(&refs).await;
                let _ = self.executor.run("rm", ["-rf", secret_dir.as_str()]).await;
                result?;
            }
            BuildStrategy::Nixpacks => {
                self.executor
                    .run(
                        "nixpacks",
                        [
                            "build",
                            spec.work_directory.as_str(),
                            "--name",
                            spec.image.as_str(),
                        ],
                    )
                    .await?;
            }
            BuildStrategy::Paketo => {
                self.executor
                    .run(
                        "pack",
                        [
                            "build",
                            spec.image.as_str(),
                            "--path",
                            spec.work_directory.as_str(),
                            "--builder",
                            "paketobuildpacks/builder-jammy-full",
                        ],
                    )
                    .await?;
            }
            BuildStrategy::Railpack { version } => {
                let plan = format!("{}/railpack-plan.json", spec.work_directory);
                self.executor
                    .run(
                        "railpack",
                        [
                            "prepare",
                            spec.work_directory.as_str(),
                            "--plan-out",
                            plan.as_str(),
                        ],
                    )
                    .await?;
                self.docker
                    .image_build(&[
                        "--build-arg",
                        format!("BUILDKIT_SYNTAX=ghcr.io/railwayapp/railpack-frontend:v{version}")
                            .as_str(),
                        "--file",
                        plan.as_str(),
                        "--tag",
                        spec.image.as_str(),
                        spec.work_directory.as_str(),
                    ])
                    .await?;
            }
            BuildStrategy::Static {
                publish_directory,
                spa,
            } => {
                let dockerfile = format!(
                    "FROM nginx:alpine\nWORKDIR /usr/share/nginx/html\n{}COPY {} .\nCMD [\"nginx\",\"-g\",\"daemon off;\"]\n",
                    if *spa {
                        "COPY nginx.conf /etc/nginx/nginx.conf\n"
                    } else {
                        ""
                    },
                    publish_directory
                );
                self.write_file(
                    &format!("{}/Dockerfile.rustploy", spec.work_directory),
                    dockerfile.as_bytes(),
                )
                .await?;
                if *spa {
                    self.write_file(
                        &format!("{}/nginx.conf", spec.work_directory),
                        SPA_NGINX.as_bytes(),
                    )
                    .await?;
                }
                self.docker
                    .image_build(&[
                        "--tag",
                        spec.image.as_str(),
                        "--file",
                        format!("{}/Dockerfile.rustploy", spec.work_directory).as_str(),
                        spec.work_directory.as_str(),
                    ])
                    .await?;
            }
        }
        Ok(())
    }

    async fn wait_healthy(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let deadline = Instant::now() + self.health_timeout;
        loop {
            self.cancelled(cancel)?;
            let output = self
                .docker
                .run([
                    "service",
                    "ps",
                    "--filter",
                    "desired-state=running",
                    "--format",
                    "{{json .}}",
                    spec.service_name().as_str(),
                ])
                .await?;
            let rows = output
                .stdout
                .lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .collect::<Vec<_>>();
            if rows.iter().any(|row| {
                row.get("CurrentState")
                    .and_then(|v| v.as_str())
                    .is_some_and(|v| v.starts_with("Running"))
            }) {
                return Ok(());
            }
            if let Some(error) = rows
                .iter()
                .filter_map(|row| row.get("Error").and_then(|v| v.as_str()))
                .find(|e| !e.is_empty())
            {
                return Err(ExecError::CommandFailed {
                    code: None,
                    stderr: error.into(),
                });
            }
            if Instant::now() >= deadline {
                return Err(ExecError::Timeout {
                    seconds: self.health_timeout.as_secs(),
                });
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
    async fn write_file(&self, path: &str, content: &[u8]) -> ExecResult<()> {
        self.executor
            .run_with_stdin(
                "sh",
                ["-c", "umask 077; cat > \"$1\"", "rustploy-write", path],
                content,
            )
            .await?;
        Ok(())
    }
    async fn prepare_file_mounts(&self, spec: &ApplicationSpec) -> ExecResult<()> {
        for mount in &spec.mounts {
            if matches!(mount.kind, MountKind::File) {
                let parent = std::path::Path::new(&mount.source)
                    .parent()
                    .ok_or_else(|| ExecError::CommandFailed {
                        code: None,
                        stderr: "invalid file mount source".into(),
                    })?;
                self.executor
                    .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                    .await?;
                let content = mount
                    .content
                    .as_deref()
                    .ok_or_else(|| ExecError::CommandFailed {
                        code: None,
                        stderr: format!("file mount {} has no content", mount.target),
                    })?;
                self.write_file(&mount.source, content.as_bytes()).await?;
            }
        }
        Ok(())
    }
    fn cancelled(&self, token: &CancellationToken) -> ExecResult<()> {
        if token.is_cancelled() {
            Err(ExecError::StreamCancelled)
        } else {
            Ok(())
        }
    }
    async fn emit(&self, event: BuilderEvent) {
        if let Some(sender) = &self.events {
            let _ = sender.send(event).await;
        }
    }
}

fn validate_spec(spec: &ApplicationSpec) -> ExecResult<()> {
    for (label, value) in [
        ("application", spec.app_name.as_str()),
        ("stack", spec.stack_name.as_str()),
        ("network", spec.network.as_str()),
    ] {
        if value.is_empty()
            || !value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!("invalid {label} name: {value}"),
            });
        }
    }
    for domain in &spec.domains {
        if domain.host.is_empty()
            || !domain
                .host
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '*'))
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!("invalid domain host: {}", domain.host),
            });
        }
        if !domain.path.starts_with('/') || !domain.internal_path.starts_with('/') {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "domain paths must start with /".into(),
            });
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct StackFile {
    version: &'static str,
    services: BTreeMap<String, StackService>,
    networks: BTreeMap<String, ExternalNetwork>,
}
#[derive(Serialize)]
struct StackService {
    image: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    environment: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    volumes: Vec<StackMount>,
    networks: Vec<String>,
    deploy: DeploySpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<HealthSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_grace_period: Option<String>,
}
#[derive(Serialize)]
struct DeploySpec {
    replicas: u32,
    resources: DeployResources,
    restart_policy: RestartPolicy,
    update_config: UpdateConfig,
    rollback_config: UpdateConfig,
    #[serde(skip_serializing_if = "Placement::is_empty")]
    placement: Placement,
}
#[derive(Serialize)]
struct DeployResources {
    #[serde(skip_serializing_if = "Limits::is_empty")]
    limits: Limits,
    #[serde(skip_serializing_if = "Limits::is_empty")]
    reservations: Limits,
}
#[derive(Serialize, Default)]
struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<String>,
}
impl Limits {
    fn is_empty(&self) -> bool {
        self.cpus.is_none() && self.memory.is_none()
    }
}
#[derive(Serialize)]
struct RestartPolicy {
    condition: &'static str,
    delay: &'static str,
    max_attempts: u8,
    window: &'static str,
}
#[derive(Serialize)]
struct UpdateConfig {
    parallelism: u8,
    delay: &'static str,
    order: &'static str,
    failure_action: &'static str,
}
#[derive(Serialize, Default)]
struct Placement {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    constraints: Vec<String>,
}
impl Placement {
    fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}
#[derive(Serialize)]
struct ExternalNetwork {
    external: bool,
    name: String,
}
#[derive(Serialize)]
struct StackMount {
    #[serde(rename = "type")]
    kind: &'static str,
    source: String,
    target: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    read_only: bool,
}
fn stack_spec(app: &ApplicationSpec) -> StackFile {
    let mut services = BTreeMap::new();
    services.insert(
        app.app_name.clone(),
        StackService {
            image: app.image.clone(),
            environment: app.environment.clone(),
            command: app.command.clone(),
            args: app.args.clone(),
            volumes: app
                .mounts
                .iter()
                .map(|mount| StackMount {
                    kind: match mount.kind {
                        MountKind::Volume => "volume",
                        MountKind::Bind | MountKind::File => "bind",
                    },
                    source: mount.source.clone(),
                    target: mount.target.clone(),
                    read_only: mount.read_only || matches!(mount.kind, MountKind::File),
                })
                .collect(),
            networks: vec![app.network.clone()],
            deploy: DeploySpec {
                replicas: app.replicas,
                resources: DeployResources {
                    limits: Limits {
                        cpus: app.resources.cpu_limit.clone(),
                        memory: app.resources.memory_limit.clone(),
                    },
                    reservations: Limits {
                        cpus: app.resources.cpu_reservation.clone(),
                        memory: app.resources.memory_reservation.clone(),
                    },
                },
                restart_policy: RestartPolicy {
                    condition: "on-failure",
                    delay: "5s",
                    max_attempts: 3,
                    window: "120s",
                },
                update_config: UpdateConfig {
                    parallelism: 1,
                    delay: "5s",
                    order: "start-first",
                    failure_action: "rollback",
                },
                rollback_config: UpdateConfig {
                    parallelism: 1,
                    delay: "5s",
                    order: "stop-first",
                    failure_action: "pause",
                },
                placement: Placement {
                    constraints: app.placement_constraints.clone(),
                },
            },
            healthcheck: app.healthcheck.clone(),
            stop_grace_period: app.stop_grace_period.clone(),
        },
    );
    let mut networks = BTreeMap::new();
    networks.insert(
        app.network.clone(),
        ExternalNetwork {
            external: true,
            name: app.network.clone(),
        },
    );
    StackFile {
        version: "3.9",
        services,
        networks,
    }
}
const SPA_NGINX: &str = r#"events { worker_connections 1024; }
http { include mime.types; server { listen 80; root /usr/share/nginx/html; index index.html; location / { try_files $uri $uri/ /index.html; } } }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    fn spec() -> ApplicationSpec {
        ApplicationSpec {
            app_name: "api".into(),
            stack_name: "prod".into(),
            source: SourceSpec::Docker {
                image: "api:1".into(),
                registry: None,
            },
            build: None,
            work_directory: "/tmp/api".into(),
            image: "api:1".into(),
            environment: BTreeMap::from([("PORT".into(), "3000".into())]),
            build_args: BTreeMap::new(),
            build_secrets: BTreeMap::new(),
            command: None,
            args: vec![],
            replicas: 2,
            network: "rustploy-network".into(),
            mounts: vec![MountSpec {
                kind: MountKind::Volume,
                source: "api-data".into(),
                target: "/data".into(),
                read_only: false,
                content: None,
            }],
            domains: vec![],
            resources: ResourceSpec {
                memory_limit: Some("512M".into()),
                ..Default::default()
            },
            healthcheck: None,
            placement_constraints: vec!["node.role==worker".into()],
            stop_grace_period: Some("15s".into()),
        }
    }
    #[test]
    fn stack_yaml_contains_reconciled_deployment_config() {
        let yaml = serde_yaml::to_string(&stack_spec(&spec())).unwrap();
        assert!(yaml.contains("replicas: 2"));
        assert!(yaml.contains("rustploy-network"));
        assert!(yaml.contains("node.role==worker"));
        assert!(yaml.contains("failure_action: rollback"));
        assert!(yaml.contains("source: api-data"));
    }
    #[test]
    fn unsafe_app_name_is_rejected() {
        let mut value = spec();
        value.app_name = "../../root".into();
        assert!(validate_spec(&value).is_err());
    }
}
