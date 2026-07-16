use crate::utils::docker::{
    core::{ArgBuilder, Platform},
    query::filter::ImageFilter,
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, ImageSummary,
};
use std::path::Path;
use tokio::sync::mpsc;

// ── Handle ───────────────────────────────────────────────────────────────────

pub struct ImageHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ImageHandle<'a> {
    pub fn list(&self)                             -> ImageQuery<'_>   { ImageQuery::new(self.0) }
    pub fn build(&self, context: impl AsRef<Path>) -> BuildBuilder<'_> { BuildBuilder::new(self.0, context) }
    pub fn pull(&self, image: impl Into<String>)   -> PullBuilder<'_>  { PullBuilder::new(self.0, image) }
    pub fn prune(&self)                            -> ImagePrune<'_>   { ImagePrune::new(self.0) }

    pub fn push(&self, image: impl Into<String>)   -> ImagePushBuilder<'_> { ImagePushBuilder::new(self.0, image) }
    pub fn rm(&self, image: impl Into<String>)     -> ImageRmBuilder<'_> { ImageRmBuilder::new(self.0, image) }
    pub fn tag(&self, source: impl Into<String>, target: impl Into<String>) -> ImageTagBuilder<'_> { ImageTagBuilder::new(self.0, source, target) }
    pub fn history(&self, image: impl Into<String>) -> ImageHistoryBuilder<'_> { ImageHistoryBuilder::new(self.0, image) }
    pub fn save(&self, image: impl Into<String>)   -> ImageSaveBuilder<'_> { ImageSaveBuilder::new(self.0, image) }
    pub fn load(&self)                             -> ImageLoadBuilder<'_> { ImageLoadBuilder::new(self.0) }
    pub fn import(&self, source: impl Into<String>)-> ImageImportBuilder<'_> { ImageImportBuilder::new(self.0, source) }
    pub async fn inspect(&self, image: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["image", "inspect", image.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

// ── ImageQuery ────────────────────────────────────────────────────────────────

pub struct ImageQuery<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ImageQuery<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["image", "ls", "--format", "{{json .}}"]) }
    }
    pub fn all(mut self)                           -> Self { self.args.flag("--all"); self }
    pub fn filter(mut self, f: ImageFilter)        -> Self { self.args.filter(f); self }
    pub fn print(&self)                            -> String { self.args.preview() }
    pub async fn list(self) -> DockerResult<Vec<ImageSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> { Ok(!self.list().await?.is_empty()) }
}
crate::impl_builder_opts!(ImageQuery);

// ── BuildBuilder ──────────────────────────────────────────────────────────────

pub struct BuildBuilder<'a> {
    cli: &'a DockerCli,
    context: String,
    args: ArgBuilder,
}

impl<'a> BuildBuilder<'a> {
    fn new(cli: &'a DockerCli, context: impl AsRef<Path>) -> Self {
        Self { cli, context: context.as_ref().to_string_lossy().into_owned(), args: ArgBuilder::default() }
    }
    pub fn dockerfile(mut self, v: impl AsRef<Path>) -> Self { self.args.pair("--file", v.as_ref().to_string_lossy()); self }
    pub fn tag(mut self, t: impl Into<String>)       -> Self { self.args.pair("--tag", t.into()); self }
    pub fn target(mut self, t: impl Into<String>)    -> Self { self.args.pair("--target", t.into()); self }
    pub fn build_arg(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--build-arg", format!("{}={}", k.as_ref(), v.as_ref())); self
    }
    pub fn cache_from(mut self, v: impl Into<String>) -> Self { self.args.pair("--cache-from", v.into()); self }
    pub fn cache_to(mut self, v: impl Into<String>)   -> Self { self.args.pair("--cache-to", v.into()); self }
    pub fn secret(mut self, v: impl Into<String>)     -> Self { self.args.pair("--secret", v.into()); self }
    pub fn ssh(mut self, v: impl Into<String>)        -> Self { self.args.pair("--ssh", v.into()); self }
    pub fn no_cache(mut self)                         -> Self { self.args.flag("--no-cache"); self }
    pub fn pull(mut self)                             -> Self { self.args.flag("--pull"); self }
    pub fn platform(mut self, p: Platform)            -> Self { self.args.pair("--platform", p.to_string()); self }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.label(k, v); self }
    pub fn progress(mut self, v: impl Into<crate::utils::docker::BuildProgress>)   -> Self {
        let p: crate::utils::docker::BuildProgress = v.into();
        self.args.pair("--progress", p.as_str());
        self
    }
    pub fn arg(mut self, v: impl Into<String>)        -> Self { self.args.push(v.into()); self }

    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.clone().build());
        a.push(&self.context);
        a.preview()
    }

    pub async fn build(self) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.build());
        a.push(&self.context);
        self.cli.execute(&a).await
    }

    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.build());
        a.push(&self.context);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(BuildBuilder);

// ── PullBuilder ───────────────────────────────────────────────────────────────

pub struct PullBuilder<'a> { cli: &'a DockerCli, image: String, args: ArgBuilder }

impl<'a> PullBuilder<'a> {
    fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self { cli, image: image.into(), args: ArgBuilder::default() }
    }
    pub fn platform(mut self, p: Platform)    -> Self { self.args.pair("--platform", p.to_string()); self }
    pub fn all_tags(mut self)                 -> Self { self.args.flag("--all-tags"); self }
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.clone().build());
        a.push(&self.image);
        a.preview()
    }
    pub async fn pull(self) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.build());
        a.push(&self.image);
        self.cli.execute(&a).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.build());
        a.push(&self.image);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(PullBuilder);

// ── ImagePrune ────────────────────────────────────────────────────────────────

pub struct ImagePrune<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ImagePrune<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["image", "prune", "--force"]) }
    }
    pub fn all(mut self)                         -> Self { self.args.flag("--all"); self }
    pub fn filter(mut self, f: ImageFilter)      -> Self { self.args.filter(f); self }
    pub fn print(&self)                          -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ImagePrune);

// ── Generated Simple Builders ───────────────────────────────────────────────

pub struct ImagePushBuilder<'a> { cli: &'a DockerCli, image: String, args: ArgBuilder }
impl<'a> ImagePushBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self { Self { cli, image: image.into(), args: ArgBuilder::cmd(&["image", "push"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.image); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImagePushBuilder);

pub struct ImageRmBuilder<'a> { cli: &'a DockerCli, image: String, args: ArgBuilder }
impl<'a> ImageRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self { Self { cli, image: image.into(), args: ArgBuilder::cmd(&["image", "rm"]) } }
    pub fn force(mut self) -> Self { self.args.flag("--force"); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.image); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImageRmBuilder);

pub struct ImageTagBuilder<'a> { cli: &'a DockerCli, source: String, target: String, args: ArgBuilder }
impl<'a> ImageTagBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, source: impl Into<String>, target: impl Into<String>) -> Self { Self { cli, source: source.into(), target: target.into(), args: ArgBuilder::cmd(&["image", "tag"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.source); a.push(&self.target); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImageTagBuilder);

pub struct ImageHistoryBuilder<'a> { cli: &'a DockerCli, image: String, args: ArgBuilder }
impl<'a> ImageHistoryBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self { Self { cli, image: image.into(), args: ArgBuilder::cmd(&["image", "history"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.image); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImageHistoryBuilder);

pub struct ImageSaveBuilder<'a> { cli: &'a DockerCli, image: String, args: ArgBuilder }
impl<'a> ImageSaveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self { Self { cli, image: image.into(), args: ArgBuilder::cmd(&["image", "save"]) } }
    pub fn output(mut self, path: impl Into<String>) -> Self { self.args.pair("--output", path.into()); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.image); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImageSaveBuilder);

pub struct ImageLoadBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }
impl<'a> ImageLoadBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self { Self { cli, args: ArgBuilder::cmd(&["image", "load"]) } }
    pub fn input(mut self, path: impl Into<String>) -> Self { self.args.pair("--input", path.into()); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { self.cli.execute(&self.args).await }
}
crate::impl_builder_opts!(ImageLoadBuilder);

pub struct ImageImportBuilder<'a> { cli: &'a DockerCli, source: String, args: ArgBuilder }
impl<'a> ImageImportBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, source: impl Into<String>) -> Self { Self { cli, source: source.into(), args: ArgBuilder::cmd(&["image", "import"]) } }
    pub fn message(mut self, msg: impl Into<String>) -> Self { self.args.pair("--message", msg.into()); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.source); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ImageImportBuilder);

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::DockerCli;

    fn cli() -> DockerCli { DockerCli::new_local() }

    #[test]
    fn build_preview() {
        let tmp = cli();
        let b = BuildBuilder::new(&tmp, ".")
            .tag("api:latest")
            .target("release")
            .build_arg("ENV", "prod")
            .no_cache()
            .platform(Platform::LinuxArm64);
        let p = b.print();
        assert!(p.contains("image build"));
        assert!(p.contains("--tag api:latest"));
        assert!(p.contains("--target release"));
        assert!(p.contains("ENV=prod"));
        assert!(p.contains("--no-cache"));
        assert!(p.contains("linux/arm64"));
        assert!(p.ends_with("."));
    }

    #[test]
    fn pull_preview() {
        let p = PullBuilder::new(&cli(), "nginx:latest")
            .platform(Platform::LinuxAmd64)
            .print();
        assert!(p.contains("image pull"));
        assert!(p.contains("linux/amd64"));
        assert!(p.ends_with("nginx:latest"));
    }
}
