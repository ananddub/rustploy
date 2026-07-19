use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::{IntoCommand, shell_single_quote};

pub struct OsCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> OsCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub fn port(&self) -> port::PortCli<'a> { port::PortCli { executor: self.executor } }
    pub fn lock(&self) -> lock::LockCli<'a> { lock::LockCli { executor: self.executor } }
    pub fn http(&self) -> http::HttpCli<'a> { http::HttpCli { executor: self.executor } }
    pub fn system(&self) -> system::SystemCli<'a> { system::SystemCli { executor: self.executor } }
    pub fn process_api(&self) -> process::ProcessCli<'a> { process::ProcessCli { executor: self.executor } }
    pub fn service_api(&self) -> service::ServiceCli<'a> { service::ServiceCli { executor: self.executor } }
    pub fn package_api(&self) -> package::PackageCli<'a> { package::PackageCli { executor: self.executor } }
    pub fn dir_api(&self) -> dir::DirCli<'a> { dir::DirCli { executor: self.executor } }
    pub fn network(&self) -> network::NetworkCli<'a> { network::NetworkCli { executor: self.executor } }
    pub fn env(&self) -> env::EnvCli<'a> { env::EnvCli { executor: self.executor } }
    pub fn mount_api(&self) -> mount::MountCli<'a> { mount::MountCli { executor: self.executor } }
    pub fn disk(&self) -> disk::DiskCli<'a> { disk::DiskCli { executor: self.executor } }
    pub fn firewall(&self) -> firewall::FirewallCli<'a> { firewall::FirewallCli { executor: self.executor } }
    pub fn resource(&self) -> resource::ResourceCli<'a> { resource::ResourceCli { executor: self.executor } }
    pub fn file_api(&self) -> file::FileCli<'a> { file::FileCli { executor: self.executor } }
    pub fn symlink_api(&self) -> symlink::SymlinkCli<'a> { symlink::SymlinkCli { executor: self.executor } }

    // Direct methods for zero-boilerplate usage
    pub fn file(&self, path: impl IntoCommand) -> file::FileBuilder<'a> {
        file::FileBuilder::new(self.executor, path)
    }
    pub fn dir(&self, path: impl IntoCommand) -> dir::DirBuilder<'a> {
        dir::DirBuilder::new(self.executor, path)
    }
    pub fn package(&self, name: impl IntoCommand) -> package::PackageBuilder<'a> {
        package::PackageBuilder::new(self.executor, name)
    }
    pub fn service(&self, name: impl IntoCommand) -> service::ServiceBuilder<'a> {
        service::ServiceBuilder::new(self.executor, name)
    }
    pub fn process(&self, pid_or_name: impl IntoCommand) -> process::ProcessBuilder<'a> {
        process::ProcessBuilder::new(self.executor, pid_or_name)
    }
    pub fn mount(&self, source: impl IntoCommand, target: impl IntoCommand) -> mount::MountBuilder<'a> {
        mount::MountBuilder::new(self.executor, Some(source), target)
    }
    pub fn mount_ref(&self, target: impl IntoCommand) -> mount::MountBuilder<'a> {
        mount::MountBuilder::new(self.executor, None::<&str>, target)
    }
    pub fn symlink(&self, target: impl IntoCommand, link: impl IntoCommand) -> symlink::SymlinkBuilder<'a> {
        symlink::SymlinkBuilder::new(self.executor, Some(target.build_str()), link.build_str())
    }
    pub fn symlink_ref(&self, link: impl IntoCommand) -> symlink::SymlinkBuilder<'a> {
        symlink::SymlinkBuilder::new(self.executor, None, link.build_str())
    }
}

pub(crate) fn escape_arg(c: impl AsRef<str>) -> String {
    let s = c.as_ref();
    if s.starts_with('$') {
        format!("\"{}\"", s)
    } else {
        shell_single_quote(s)
    }
}

// Submodules
pub mod port;
pub mod lock;
pub mod http;
pub mod system;
pub mod process;
pub mod service;
pub mod package;
pub mod dir;
pub mod network;
pub mod env;
pub mod mount;
pub mod disk;
pub mod firewall;
pub mod resource;
pub mod file;
pub mod symlink;
