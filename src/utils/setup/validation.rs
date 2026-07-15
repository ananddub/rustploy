use crate::utils::exec::{CommandExecutor, ExecResult};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolState {
    pub installed: bool,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PortAvailability {
    pub port: u16,
    pub available: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ServerAudit {
    pub os_id: String,
    pub architecture: String,
    pub docker: ToolState,
    pub git: ToolState,
    pub rclone: ToolState,
    pub nixpacks: ToolState,
    pub railpack: ToolState,
    pub buildpacks: ToolState,
    pub swarm_active: bool,
    pub network_exists: bool,
    pub base_directory_exists: bool,
    pub docker_group_member: bool,
    pub ports: Vec<PortAvailability>,
}

pub(crate) async fn audit(
    executor: &CommandExecutor,
    base: &str,
    network: &str,
    ports: &[u16],
) -> ExecResult<ServerAudit> {
    let script = "tool(){ if command -v \"$1\" >/dev/null 2>&1; then printf 'true\\t'; \"$1\" --version 2>/dev/null | head -n1; else printf 'false\\t\\n'; fi; }\nprintf '%s\\n' \"$(. /etc/os-release; printf '%s' \"$ID\")\" \"$(uname -m)\"\ntool docker; tool git; tool rclone; tool nixpacks; tool railpack; tool pack\ndocker info --format '{{.Swarm.LocalNodeState}}' 2>/dev/null || true\ngroups 2>/dev/null | tr ' ' '\\n' | grep -qx docker && echo true || echo false";
    let mut lines = executor
        .run("sh", ["-c", script])
        .await?
        .stdout
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    while lines.len() < 10 {
        lines.push(String::new());
    }
    let parse_tool = |line: &str| {
        let mut parts = line.splitn(2, '\t');
        ToolState {
            installed: parts.next() == Some("true"),
            version: parts
                .next()
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.trim().to_owned()),
        }
    };
    let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
    let network_exists = docker.networks().inspect(network).await.is_ok();
    let base_directory_exists = executor.run("test", ["-d", base]).await.is_ok();
    let mut checked_ports = Vec::new();
    for port in ports {
        let expression = format!("! ss -H -ltnu 'sport = :{port}' 2>/dev/null | grep -q .");
        checked_ports.push(PortAvailability {
            port: *port,
            available: executor
                .run("sh", ["-c", expression.as_str()])
                .await
                .is_ok(),
        });
    }
    Ok(ServerAudit {
        os_id: lines[0].clone(),
        architecture: lines[1].clone(),
        docker: parse_tool(&lines[2]),
        git: parse_tool(&lines[3]),
        rclone: parse_tool(&lines[4]),
        nixpacks: parse_tool(&lines[5]),
        railpack: parse_tool(&lines[6]),
        buildpacks: parse_tool(&lines[7]),
        swarm_active: lines[8].trim() == "active",
        docker_group_member: lines[9].trim() == "true",
        network_exists,
        base_directory_exists,
        ports: checked_ports,
    })
}
