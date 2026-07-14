use super::{DockerCli, DockerOutput, DockerResult, NetworkSummary, VolumeSummary};
use serde::de::DeserializeOwned;

impl DockerCli {
    /// Raw network list — prefer [`DockerCli::networks()`] handle instead.
    pub async fn networks_raw(&self, filters: &[&str]) -> DockerResult<Vec<NetworkSummary>> {
        self.list("network", filters).await
    }
    /// Raw volume list — prefer [`DockerCli::volumes()`] handle instead.
    pub async fn volumes_raw(&self, filters: &[&str]) -> DockerResult<Vec<VolumeSummary>> {
        self.list("volume", filters).await
    }
    pub async fn network_create(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "create"], args).await
    }
    pub async fn network_remove(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "rm"], args).await
    }
    pub async fn network_connect(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "connect"], args).await
    }
    pub async fn network_disconnect(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "disconnect"], args).await
    }
    pub async fn volume_create(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["volume", "create"], args).await
    }
    pub async fn volume_remove(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["volume", "rm"], args).await
    }
    pub async fn network_inspect<T: DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut cmd = vec!["network", "inspect"];
        cmd.extend_from_slice(args);
        self.json(&cmd).await
    }
    pub async fn volume_inspect<T: DeserializeOwned>(&self, args: &[&str]) -> DockerResult<Vec<T>> {
        let mut cmd = vec!["volume", "inspect"];
        cmd.extend_from_slice(args);
        self.json(&cmd).await
    }
    pub async fn network_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("network", filters).await
    }
    pub async fn volume_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("volume", filters).await
    }
}



// #[test]
pub mod TestingDockerBuilder {
    use crate::utils::docker::query::{ContainerFilter, ContainerStatus, HealthStatus, ImageFilter, RestartPolicy};
    use super::*;

    #[test]
    fn docker_test() {
        let docker = DockerCli::new_local();
        let v =   docker.images()
            .build(".")
            .dockerfile("Dockerfile")
            .tag("my-alpine:latest")
            .print();
        println!("docker build command: {}", v);
    }

    #[test]
    fn docker_filter(){
        let docker = DockerCli::new_local();
        let v  = docker
            .images()
            .list()
            .filter(ImageFilter::Dangling(false))
            .filter(ImageFilter::Reference("rust:*".to_string()))
            .filter(ImageFilter::Before("ubuntu:24.04".to_string()))
            .filter(ImageFilter::Since("alpine:3.20".to_string()))
            .print();
        println!("docker filter command: {}", v);
    }
    #[test]
    fn docker_container_filter(){
        // let docker = DockerCli::new_local();
        //
        // let cmd =  docker
        //     .containers()
        //     .create("ghcr.io/example/backend:latest")
        //     .name("backend")
        //     .hostname("api")
        //     .network("frontend")
        //     .ip("172.20.0.20")
        //     .memory("1g")
        //     .cpus(2)
        //     .restart(RestartPolicy::UnlessStopped)
        //     .env("RUST_LOG","debug")
        //     .env("DATABASE_URL","mysql://db")
        //     .volume("app-data","/data")
        //     .publish(8080,8080)
        //     .health_cmd("curl -f http://localhost:8080/health || exit 1")
        //     .health_interval("30s")
        //     .label("app","backend")
        //     .print();
        // println!("docker container filter command: {}", cmd);
    }

}