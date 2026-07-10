use super::{DockerCli, DockerOutput, DockerResult, ImageSummary};
use serde::de::DeserializeOwned;

impl DockerCli {
    pub async fn images(&self, all: bool, filters: &[&str]) -> DockerResult<Vec<ImageSummary>> {
        let mut args = vec!["image", "ls", "--format", "{{json .}}"];
        if all {
            args.push("--all");
        }
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.json_lines(&args).await
    }
    pub async fn image_pull(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "pull"], args).await
    }
    pub async fn image_push(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "push"], args).await
    }
    pub async fn image_build(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "build"], args).await
    }
    pub async fn image_tag(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "tag"], args).await
    }
    pub async fn image_load(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "load"], args).await
    }
    pub async fn image_save(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "save"], args).await
    }
    pub async fn image_history(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "history"], args).await
    }
    pub async fn image_remove(&self, images: &[&str], force: bool) -> DockerResult<DockerOutput> {
        let mut args = vec!["image", "rm"];
        if force {
            args.push("--force");
        }
        args.extend_from_slice(images);
        self.run(args).await
    }
    pub async fn image_inspect<T: DeserializeOwned>(
        &self,
        targets: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut args = vec!["image", "inspect"];
        args.extend_from_slice(targets);
        self.json(&args).await
    }
    pub async fn image_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("image", filters).await
    }
}
