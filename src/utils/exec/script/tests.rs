use crate::utils::exec::ScriptPipeline;
use crate::utils::exec::Condition;
use crate::utils::exec::script::shell_single_quote;

use super::*;
use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::rclone::{RcloneBuilder, RcloneCommand, RcloneTarget};
use tokio_util::sync::CancellationToken;
use crate::pipeline;
use crate::utils::builder::spec::BuildStrategy::Nixpacks;
use crate::utils::git::GitCli;

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
        & Condition::file_exists("/tmp/repo/config.json")
        & Condition::cmd_succeeds("git status")
        | Condition::env_set("FORCE_DEPLOY").not();

    let pipeline = ScriptPipeline::new()
        .if_condition(cond)
            .then(|p| p.cmd("echo 'deploying'").cmd("cargo build"))
            .otherwise(|p| p.cmd("echo 'skipping'"))
        .cmd("echo 'done'");
    let script = pipeline.compile();
    assert!(script.contains("if ([ -d '/tmp/repo' ] && [ -f '/tmp/repo/config.json' ] && git status) || ! [ -n \"${FORCE_DEPLOY}\" ]; then"));
    assert!(script.contains("    echo 'deploying'"));
    assert!(script.contains("    cargo build"));
    assert!(script.contains("else"));
    assert!(script.contains("    echo 'skipping'"));
    assert!(script.contains("fi"));
    assert!(script.contains("echo 'done'"));
}
#[tokio::test]
async fn test_pipeline_condition_enum_and_fluent_macros() {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    let nixpacks_cli = crate::utils::builder::packs::nixpacks::NixpacksCli::new(&executor);

    let pipeline = pipeline! {
        if (dir("/tmp/repo") && file("/tmp/repo/config.json")&& cmd("git status")|| !env("FORCE_DEPLOY")) {
            echo("deploying");
            cargo("build");
        } else {
            nixpacks_cli.build("/path/to/project").name("my-app");
            echo("skipping");
        }
        echo("done");
    };

    let script = pipeline.compile();
    assert!(script.contains("if ([ -d '/tmp/repo' ] && [ -f '/tmp/repo/config.json' ] && git status) || ! [ -n \"${FORCE_DEPLOY}\" ]; then"));
    assert!(script.contains("    echo 'deploying'"));
    assert!(script.contains("    cargo 'build'"));
    assert!(script.contains("else"));
    assert!(script.contains("    nixpacks build /path/to/project --name my-app"));
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

#[tokio::test]
async fn test_pipeline_array_and_vector_commands() {
    let pipeline = pipeline! {
        cmd(&["echo", "hello", "world"]);
        cmd(vec!["echo", "foo", "bar"]);
    };

    let script = pipeline.compile();
    assert!(script.contains("'echo' 'hello' 'world'"));
    assert!(script.contains("'echo' 'foo' 'bar'"));
}
