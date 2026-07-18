use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::rclone::RcloneBuilder;
use crate::utils::exec::ArgBuilder;
use tokio_util::sync::CancellationToken;

/// Trait for types that can be compiled into a shell command string.
pub trait IntoCommand {
    fn build_str(&self) -> String;

    #[allow(async_fn_in_trait)]
    async fn execute(&self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let cmd_str = self.build_str();
        executor.run("sh", &["-c", &cmd_str]).await
    }

    #[allow(async_fn_in_trait)]
    async fn execute_cancelled(
        &self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let cmd_str = self.build_str();
        executor.run_cancelled("sh", &["-c", &cmd_str], cancel).await
    }
}

impl IntoCommand for String {
    fn build_str(&self) -> String {
        self.clone()
    }
}

impl IntoCommand for &str {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for &String {
    fn build_str(&self) -> String {
        (*self).clone()
    }
}

#[derive(Clone, Debug)]
pub enum Condition {
    DirExists(String),
    FileExists(String),
    CmdSucceeds(String),
    EnvSet(String),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    pub fn dir_exists(path: impl Into<String>) -> Self {
        Condition::DirExists(path.into())
    }

    pub fn file_exists(path: impl Into<String>) -> Self {
        Condition::FileExists(path.into())
    }

    pub fn cmd_succeeds(cmd: impl IntoCommand) -> Self {
        Condition::CmdSucceeds(cmd.build_str())
    }

    pub fn env_set(key: impl Into<String>) -> Self {
        Condition::EnvSet(key.into())
    }

    pub fn not(self) -> Self {
        Condition::Not(Box::new(self))
    }

    pub fn and(self, other: Condition) -> Self {
        Condition::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: Condition) -> Self {
        Condition::Or(Box::new(self), Box::new(other))
    }

    fn to_bash_inner(&self, needs_parens: bool) -> String {
        let s = match self {
            Condition::DirExists(p) => format!("[ -d {} ]", shell_single_quote(p)),
            Condition::FileExists(p) => format!("[ -f {} ]", shell_single_quote(p)),
            Condition::CmdSucceeds(c) => c.clone(),
            Condition::EnvSet(k) => format!("[ -n \"${{{}}}\" ]", k),
            Condition::Not(c) => format!("! {}", c.to_bash_inner(true)),
            Condition::And(a, b) => format!(
                "{} && {}", 
                a.to_bash_inner(true), 
                b.to_bash_inner(true)
            ),
            Condition::Or(a, b) => format!(
                "{} || {}", 
                a.to_bash_inner(true), 
                b.to_bash_inner(true)
            ),
        };
        if needs_parens && matches!(self, Condition::And(..) | Condition::Or(..)) {
            format!("({})", s)
        } else {
            s
        }
    }

    pub fn to_bash(&self) -> String {
        self.to_bash_inner(false)
    }
}

impl IntoCommand for Condition {
    fn build_str(&self) -> String {
        self.to_bash()
    }
}

impl IntoCommand for ArgBuilder {
    fn build_str(&self) -> String {
        self.preview()
    }
}

impl IntoCommand for RcloneBuilder {
    fn build_str(&self) -> String {
        self.clone().to_command_string()
    }
}

/// Represents a single step in the execution pipeline.
#[derive(Clone, Debug)]
pub enum PipelineStep {
    /// A standalone command or builder that runs sequentially (on a new line).
    Sequential(String),
    /// Runs only if the previous command succeeds (&&).
    And(String),
    /// Runs only if the previous command fails (||).
    Or(String),
    /// Pipes stdout of the previous command to stdin of this command (|).
    Pipe(String),
    /// A conditional if-else block.
    IfElse {
        condition: String,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    },
}

/// A pipeline for executing multiple commands in a single shell invocation.
#[derive(Default, Clone, Debug)]
pub struct ScriptPipeline {
    working_dir: Option<String>,
    env_vars: Vec<(String, String)>,
    steps: Vec<PipelineStep>,
    trace: bool,
    verbose_headers: bool,
}

impl ScriptPipeline {
    /// Creates a new empty `ScriptPipeline`.
    pub fn new() -> Self {
        Self {
            working_dir: None,
            env_vars: Vec::new(),
            steps: Vec::new(),
            trace: false,
            verbose_headers: false,
        }
    }

    /// Sets the pipeline-wide working directory.
    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Adds a pipeline-wide environment variable.
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), val.into()));
        self
    }

    /// Enables or disables command execution tracing (`set -x`).
    pub fn trace(mut self, enabled: bool) -> Self {
        self.trace = enabled;
        self
    }

    /// Enables or disables verbose headers printed before sequential steps.
    pub fn verbose_headers(mut self, enabled: bool) -> Self {
        self.verbose_headers = enabled;
        self
    }

    /// Adds a command or builder that runs sequentially (on a new line).
    pub fn cmd(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Sequential(cmd.build_str()));
        self
    }

    /// Appends multiple sequential commands or builders to the pipeline.
    pub fn extend_cmds(mut self, cmds: impl IntoIterator<Item = impl IntoCommand>) -> Self {
        for cmd in cmds {
            self = self.cmd(cmd);
        }
        self
    }

    /// Runs this command only if the previous step succeeded (&&).
    pub fn and(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::And(cmd.build_str()));
        self
    }

    /// Runs this command only if the previous step failed (||).
    pub fn or(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Or(cmd.build_str()));
        self
    }

    /// Pipes stdout of the previous command to stdin of this command (|).
    pub fn pipe(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Pipe(cmd.build_str()));
        self
    }

    /// Adds a conditional if/else block to the pipeline.
    pub fn if_else(
        mut self,
        condition: impl IntoCommand,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        self.steps.push(PipelineStep::IfElse {
            condition: condition.build_str(),
            then_branch,
            else_branch,
        });
        self
    }

    /// Runs then_branch if the given path is a directory.
    pub fn if_dir_exists(
        self,
        path: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -d {} ]", shell_single_quote(path.as_ref()));
        self.if_else(cond, then_branch, else_branch)
    }

    /// Runs then_branch if the given path is a regular file.
    pub fn if_file_exists(
        self,
        path: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -f {} ]", shell_single_quote(path.as_ref()));
        self.if_else(cond, then_branch, else_branch)
    }

    /// Runs then_branch if the given command exits 0.
    pub fn if_cmd_succeeds(
        self,
        cmd: impl IntoCommand,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        self.if_else(cmd.build_str(), then_branch, else_branch)
    }

    /// Runs then_branch if the given env var is set (non-empty).
    pub fn if_env_set(
        self,
        key: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -n \"${{{}}}\" ]", key.as_ref());
        self.if_else(cond, then_branch, else_branch)
    }

    /// Compatibility alias to add an `RcloneBuilder`.
    pub fn rclone(self, builder: RcloneBuilder) -> Self {
        self.cmd(builder)
    }

    /// Adds a conditional block using fluent chaining.
    pub fn if_condition(self, cond: Condition) -> IfBuilder {
        IfBuilder { condition: cond, parent: self }
    }
}

pub struct IfBuilder {
    condition: Condition,
    parent: ScriptPipeline,
}

impl IfBuilder {
    pub fn then<F>(self, build: F) -> IfThenBuilder
    where
        F: FnOnce(ScriptPipeline) -> ScriptPipeline,
    {
        let then_branch = build(ScriptPipeline::new());
        IfThenBuilder {
            condition: self.condition,
            then_branch,
            parent: self.parent,
        }
    }
}

pub struct IfThenBuilder {
    condition: Condition,
    then_branch: ScriptPipeline,
    parent: ScriptPipeline,
}

impl IfThenBuilder {
    pub fn otherwise<F>(self, build: F) -> ScriptPipeline
    where
        F: FnOnce(ScriptPipeline) -> ScriptPipeline,
    {
        let else_branch = build(ScriptPipeline::new());
        self.parent.if_else(self.condition, self.then_branch, Some(else_branch))
    }

    pub fn end_if(self) -> ScriptPipeline {
        self.parent.if_else(self.condition, self.then_branch, None)
    }
}

impl ScriptPipeline {
    /// Compiles all commands and settings in the pipeline into a single POSIX-compliant shell script.
    pub fn compile(&self) -> String {
        let mut script = String::new();
        script.push_str("set -e\n");
        if self.trace {
            script.push_str("set -x\n");
        }
        script.push_str(&self.compile_inner());
        script.push('\n');
        script
    }

    /// Helper method to recursively compile inner steps.
    pub fn compile_inner(&self) -> String {
        let mut script = String::new();
        for (k, v) in &self.env_vars {
            script.push_str(&format!("export {}={}\n", k, shell_single_quote(v)));
        }
        if let Some(ref dir) = self.working_dir {
            script.push_str(&format!("cd {}\n", shell_single_quote(dir)));
        }

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                PipelineStep::Sequential(cmd) => {
                    if i > 0 {
                        script.push('\n');
                    }
                    if self.verbose_headers {
                        script.push_str(&format!("echo '=== [Step {}] ==='\n", i + 1));
                    }
                    script.push_str(cmd);
                }
                PipelineStep::And(cmd) => {
                    script.push_str(" && ");
                    script.push_str(cmd);
                }
                PipelineStep::Or(cmd) => {
                    script.push_str(" || ");
                    script.push_str(cmd);
                }
                PipelineStep::Pipe(cmd) => {
                    script.push_str(" | ");
                    script.push_str(cmd);
                }
                PipelineStep::IfElse { condition, then_branch, else_branch } => {
                    if i > 0 {
                        script.push('\n');
                    }
                    script.push_str(&format!("if {}; then\n", condition));
                    let then_compiled = then_branch.compile_inner();
                    if then_compiled.trim().is_empty() {
                        script.push_str("    :\n");
                    } else {
                        for line in then_compiled.lines() {
                            script.push_str(&format!("    {}\n", line));
                        }
                    }
                    if let Some(eb) = else_branch {
                        script.push_str("else\n");
                        let else_compiled = eb.compile_inner();
                        if else_compiled.trim().is_empty() {
                            script.push_str("    :\n");
                        } else {
                            for line in else_compiled.lines() {
                                script.push_str(&format!("    {}\n", line));
                            }
                        }
                    }
                    script.push_str("fi");
                }
            }
        }
        script
    }

    /// Executes the entire pipeline using the provided executor by piping the compiled script into `sh`.
    pub async fn execute(&self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let script = self.compile();
        executor.run_with_stdin("sh", &[] as &[&str], script).await
    }

    /// Executes the entire pipeline with cancellation support.
    pub async fn execute_cancelled(
        &self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let script = self.compile();
        executor.run_with_stdin_cancelled("sh", &[] as &[&str], script, cancel).await
    }
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::exec::LocalExecutor;
    use crate::utils::rclone::{RcloneCommand, RcloneTarget};

    #[tokio::test]
    async fn test_pipeline_execution() {
        let executor = CommandExecutor::Local(LocalExecutor::new());

        let pipeline = ScriptPipeline::new()
            .cmd("echo 'line 1'")
            .cmd("echo 'line 2'");

        let output = pipeline.execute(&executor).await.unwrap();
        assert!(output.status.success());
        assert!(output.stdout.contains("line 1"));
        assert!(output.stdout.contains("line 2"));
    }

    #[tokio::test]
    async fn test_pipeline_with_rclone_compilation() {
        let src = RcloneTarget::Local {
            path: "/tmp/src.txt".to_string(),
        };
        let dest = RcloneTarget::Local {
            path: "/tmp/dest.txt".to_string(),
        };
        let rclone = RcloneBuilder::new(RcloneCommand::Copyto)
            .source(src)
            .destination(dest);

        let pipeline = ScriptPipeline::new()
            .cmd("echo 'starting rclone'")
            .cmd(rclone)
            .cmd("echo 'finished rclone'");

        let script = pipeline.compile();
        assert!(script.contains("set -e"));
        assert!(script.contains("starting rclone"));
        assert!(script.contains("rclone copyto /tmp/src.txt /tmp/dest.txt"));
        assert!(script.contains("finished rclone"));
    }

    #[tokio::test]
    async fn test_pipeline_with_various_builders() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cli = crate::utils::docker::DockerCli::new_local();

        // Bind the handles to local variables to extend their lifetimes
        let containers_handle = cli.containers();
        let images_handle = cli.images();
        let compose_handle = cli.compose();
        let services_handle = cli.services();
        let stacks_handle = cli.stacks();

        // 1. ContainerCreate builder
        let container = containers_handle.create("alpine:latest")
            .name("test-pipeline-container")
            .tty(true);

        // 2. ContainerStartBuilder
        let start = containers_handle.start("test-pipeline-container");

        // 3. NixpacksBuildBuilder
        let nixpacks_cli = crate::utils::builder::packs::nixpacks::NixpacksCli::new(&executor);
        let nixpacks = nixpacks_cli.build("/path/to/project").name("my-app");

        // 4. Image BuildBuilder
        let img_build = images_handle.build("/path/to/ctx").tag("my-image:latest");

        // 5. Compose UpBuilder
        let compose_up = compose_handle.up().detach().build();

        // 6. Service CreateBuilder
        let svc_create = services_handle.create("nginx:latest").name("my-service");

        // 7. Stack DeployBuilder
        let stack_deploy = stacks_handle.deploy("my-stack").compose_file("docker-compose.yml");

        // Pipeline combining all
        let pipeline = ScriptPipeline::new()
            .cmd(container)
            .cmd(start)
            .cmd(nixpacks)
            .cmd(img_build)
            .cmd(compose_up)
            .cmd(svc_create)
            .cmd(stack_deploy);

        let script = pipeline.compile();
        assert!(script.contains("docker container run --name test-pipeline-container --tty alpine:latest"));
        assert!(script.contains("docker container start test-pipeline-container"));
        assert!(script.contains("nixpacks build /path/to/project --name my-app"));
        assert!(script.contains("docker image build --tag my-image:latest /path/to/ctx"));
        assert!(script.contains("docker compose up --detach --build"));
        assert!(script.contains("docker service create --name my-service nginx:latest"));
        assert!(script.contains("docker stack deploy --compose-file docker-compose.yml my-stack"));
    }

    #[tokio::test]
    async fn test_pipeline_advanced_features() {
        let pipeline = ScriptPipeline::new()
            .working_dir("/var/www/app")
            .env("ENV_KEY", "env_val")
            .trace(true)
            .verbose_headers(true)
            .cmd("echo 'Step 1'")
            .and("echo 'Runs only if Step 1 succeeded'")
            .or("echo 'Runs only if Step 1 failed'")
            .pipe("grep -i step");

        let script = pipeline.compile();
        assert!(script.contains("set -x"));
        assert!(script.contains("export ENV_KEY='env_val'"));
        assert!(script.contains("cd '/var/www/app'"));
        assert!(script.contains("echo '=== [Step 1] ==='"));
        assert!(script.contains("echo 'Step 1' && echo 'Runs only if Step 1 succeeded' || echo 'Runs only if Step 1 failed' | grep -i step"));
    }

    #[tokio::test]
    async fn test_pipeline_if_else() {
        let then_branch = ScriptPipeline::new()
            .cmd("echo 'directory exists'")
            .cmd("ls -la");

        let else_branch = ScriptPipeline::new()
            .cmd("echo 'directory does not exist'")
            .cmd("mkdir -p /tmp/test-dir");

        let pipeline = ScriptPipeline::new()
            .if_else("[ -d /tmp/test-dir ]", then_branch, Some(else_branch));

        let script = pipeline.compile();
        assert!(script.contains("if [ -d /tmp/test-dir ]; then"));
        assert!(script.contains("    echo 'directory exists'"));
        assert!(script.contains("    ls -la"));
        assert!(script.contains("else"));
        assert!(script.contains("    echo 'directory does not exist'"));
        assert!(script.contains("    mkdir -p /tmp/test-dir"));
        assert!(script.contains("fi"));
    }

    #[tokio::test]
    async fn test_pipeline_git_builders() {
        let git_cli = crate::utils::git::client::GitCli::new_local()
            .with_repository("/tmp/repo");

        let clone = git_cli.clone("https://github.com/test/repo.git").destination("/tmp/repo");
        let fetch = git_cli.fetch().all();
        let reset = git_cli.reset().hard().commit("origin/main");

        let pipeline = ScriptPipeline::new()
            .cmd(clone)
            .cmd(fetch)
            .cmd(reset);

        let script = pipeline.compile();
        assert!(script.contains("git clone https://github.com/test/repo.git /tmp/repo"));
        assert!(script.contains("git -c safe.directory=/tmp/repo -C /tmp/repo fetch --all"));
        assert!(script.contains("git -c safe.directory=/tmp/repo -C /tmp/repo reset --hard origin/main"));
    }

    #[tokio::test]
    async fn test_pipeline_convenience_conditions() {
        let then_branch = ScriptPipeline::new().cmd("echo 'then'");
        let else_branch = ScriptPipeline::new().cmd("echo 'else'");

        let pipeline = ScriptPipeline::new()
            .if_dir_exists("/tmp/test-dir", then_branch.clone(), Some(else_branch.clone()))
            .if_file_exists("/tmp/test-file", then_branch.clone(), Some(else_branch.clone()))
            .if_cmd_succeeds("git status", then_branch.clone(), Some(else_branch.clone()))
            .if_env_set("MY_VAR", then_branch, Some(else_branch));

        let script = pipeline.compile();
        assert!(script.contains("if [ -d '/tmp/test-dir' ]; then"));
        assert!(script.contains("if [ -f '/tmp/test-file' ]; then"));
        assert!(script.contains("if git status; then"));
        assert!(script.contains("if [ -n \"${MY_VAR}\" ]; then"));
    }

    #[tokio::test]
    async fn test_pipeline_condition_enum_and_fluent_api() {
        let cond = Condition::dir_exists("/tmp/repo")
            .and(Condition::file_exists("/tmp/repo/config.json"))
            .and(Condition::cmd_succeeds("git status"))
            .or(Condition::env_set("FORCE_DEPLOY").not());

        let pipeline = ScriptPipeline::new()
            .if_condition(cond)
                .then(|p|
                    p.cmd("echo 'deploying'").cmd("cargo build")
                )
                .otherwise(|p|
                    p.cmd("echo 'skipping'")
                )
            .cmd("echo 'done'");

        let script = pipeline.compile();
        assert!(script.contains("if (([ -d '/tmp/repo' ] && [ -f '/tmp/repo/config.json' ]) && git status) || ! [ -n \"${FORCE_DEPLOY}\" ]; then"));
        assert!(script.contains("    echo 'deploying'"));
        assert!(script.contains("    cargo build"));
        assert!(script.contains("else"));
        assert!(script.contains("    echo 'skipping'"));
        assert!(script.contains("fi"));
        assert!(script.contains("echo 'done'"));
    }

    #[tokio::test]
    async fn test_direct_builder_execution() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cmd = "echo 'direct execution works'";
        
        let out = cmd.execute(&executor).await.unwrap();
        assert!(out.status.success());
        assert!(out.stdout.contains("direct execution works"));

        let cancel = CancellationToken::new();
        let out_cancelled = cmd.execute_cancelled(&executor, &cancel).await.unwrap();
        assert!(out_cancelled.status.success());
        assert!(out_cancelled.stdout.contains("direct execution works"));
    }
}
