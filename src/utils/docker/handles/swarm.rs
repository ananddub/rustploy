use crate::utils::{
    docker::{
        core::{types::SwarmRole, ArgBuilder},
        client::DockerCli,
        DockerOutput, DockerResult,
    },
    exec::ExecOutput,
};

// ── SwarmHandle ─────────────────────────────────────────────────────────────

pub struct SwarmHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> SwarmHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn init(&self) -> SwarmInitBuilder<'a> {
        SwarmInitBuilder::new(self.cli)
    }

    pub fn join(&self) -> SwarmJoinBuilder<'a> {
        SwarmJoinBuilder::new(self.cli)
    }

    pub fn leave(&self) -> SwarmLeaveBuilder<'a> {
        SwarmLeaveBuilder::new(self.cli)
    }

    pub fn update(&self) -> SwarmUpdateBuilder<'a> {
        SwarmUpdateBuilder::new(self.cli)
    }

    pub fn unlock_key(&self) -> SwarmUnlockKeyBuilder<'a> {
        SwarmUnlockKeyBuilder::new(self.cli)
    }

    pub fn join_token(&self) -> SwarmJoinTokenBuilder<'a> {
        SwarmJoinTokenBuilder::new(self.cli)
    }

    pub fn unlock(&self, key: impl Into<String>) -> SwarmUnlockBuilder<'a> {
        SwarmUnlockBuilder::new(self.cli, key)
    }

    pub fn ca(&self) -> SwarmCaBuilder<'a> {
        SwarmCaBuilder::new(self.cli)
    }

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::SwarmInfo> {
        let output = self.cli.run(["info", "--format", "{{json .Swarm}}"]).await?;
        let json = serde_json::from_str(&output.stdout)?;
        Ok(json)
    }
}

// ── SwarmInitBuilder ────────────────────────────────────────────────────────

pub struct SwarmInitBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmInitBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "init"]) }
    }

    pub fn advertise_addr(mut self, addr: impl AsRef<str>) -> Self { self.args.pair("--advertise-addr", addr); self }
    pub fn listen_addr(mut self, addr: impl AsRef<str>) -> Self { self.args.pair("--listen-addr", addr); self }
    pub fn force_new_cluster(mut self) -> Self { self.args.flag("--force-new-cluster"); self }
    pub fn data_path_port(mut self, port: u16) -> Self { self.args.pair("--data-path-port", port.to_string()); self }
    pub fn default_addr_pool(mut self, pool: impl AsRef<str>) -> Self { self.args.pair("--default-addr-pool", pool); self }
    pub fn default_addr_pool_mask_length(mut self, len: u8) -> Self { self.args.pair("--default-addr-pool-mask-length", len.to_string()); self }
    pub fn arg(mut self, v: impl Into<String>) -> Self { self.args.push(v.into()); self }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmInitBuilder);

// ── SwarmJoinBuilder ────────────────────────────────────────────────────────

pub struct SwarmJoinBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    remote: Option<String>,
}

impl<'a> SwarmJoinBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "join"]), remote: None }
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self { self.remote = Some(remote.into()); self }
    pub fn token(mut self, token: impl AsRef<str>) -> Self { self.args.pair("--token", token); self }
    pub fn advertise_addr(mut self, addr: impl AsRef<str>) -> Self { self.args.pair("--advertise-addr", addr); self }
    pub fn listen_addr(mut self, addr: impl AsRef<str>) -> Self { self.args.pair("--listen-addr", addr); self }
    pub fn data_path_port(mut self, port: u16) -> Self { self.args.pair("--data-path-port", port.to_string()); self }
    pub fn arg(mut self, v: impl Into<String>) -> Self { self.args.push(v.into()); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        if let Some(r) = self.remote.take() {
            self.args.push(r);
        }
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmJoinBuilder);

// ── SwarmLeaveBuilder ───────────────────────────────────────────────────────

pub struct SwarmLeaveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmLeaveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "leave"]) }
    }

    pub fn force(mut self) -> Self { self.args.flag("--force"); self }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmLeaveBuilder);

// ── SwarmUpdateBuilder ──────────────────────────────────────────────────────

pub struct SwarmUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "update"]) }
    }

    pub fn autolock(mut self, lock: bool) -> Self { self.args.pair("--autolock", lock.to_string()); self }
    pub fn task_history_limit(mut self, limit: u32) -> Self { self.args.pair("--task-history-limit", limit.to_string()); self }
    pub fn snapshot_interval(mut self, interval: u32) -> Self { self.args.pair("--snapshot-interval", interval.to_string()); self }
    pub fn dispatcher_heartbeat(mut self, duration: impl AsRef<str>) -> Self { self.args.pair("--dispatcher-heartbeat", duration); self }
    pub fn arg(mut self, v: impl Into<String>) -> Self { self.args.push(v.into()); self }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmUpdateBuilder);

// ── SwarmUnlockKeyBuilder ───────────────────────────────────────────────────

pub struct SwarmUnlockKeyBuilder<'a> {
    cli: &'a DockerCli,
}

impl<'a> SwarmUnlockKeyBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub async fn get(&self) -> DockerResult<String> {
        let out = self.cli.run(["swarm", "unlock-key", "--quiet"]).await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self) -> DockerResult<String> {
        let out = self.cli.run(["swarm", "unlock-key", "--rotate", "--quiet"]).await?;
        Ok(out.stdout.trim().to_string())
    }
}

// ── SwarmJoinTokenBuilder ───────────────────────────────────────────────────

pub struct SwarmJoinTokenBuilder<'a> {
    cli: &'a DockerCli,
}

impl<'a> SwarmJoinTokenBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub async fn get(&self, role: SwarmRole) -> DockerResult<String> {
        let out = self.cli.run(["swarm", "join-token", "--quiet", &role.to_string()]).await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self, role: SwarmRole) -> DockerResult<String> {
        let out = self.cli.run(["swarm", "join-token", "--rotate", "--quiet", &role.to_string()]).await?;
        Ok(out.stdout.trim().to_string())
    }
}

// ── SwarmUnlockBuilder ──────────────────────────────────────────────────────

pub struct SwarmUnlockBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    key: String,
}

impl<'a> SwarmUnlockBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, key: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "unlock"]), key: key.into() }
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        // Securely pass the key via stdin
        self.cli.run_with_stdin(refs, self.key.as_bytes()).await
    }
}

// ── SwarmCaBuilder ──────────────────────────────────────────────────────────

pub struct SwarmCaBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmCaBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["swarm", "ca"]) }
    }

    pub fn rotate(mut self) -> Self { self.args.flag("--rotate"); self }
    pub fn external_ca(mut self, ca: impl AsRef<str>) -> Self { self.args.pair("--external-ca", ca); self }
    pub fn ca_cert(mut self, cert: impl AsRef<str>) -> Self { self.args.pair("--ca-cert", cert); self }
    pub fn ca_key(mut self, key: impl AsRef<str>) -> Self { self.args.pair("--ca-key", key); self }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmCaBuilder);
