#[macro_export]
macro_rules! impl_builder_opts {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
            /// Set the maximum number of retries for this command.
            pub fn retry(mut self, max_retries: u32) -> Self {
                self.args.retry_limit = Some(max_retries);
                self
            }

            /// Attach a cancellation token to gracefully abort long-running commands.
            pub fn cancel_with(mut self, token: tokio_util::sync::CancellationToken) -> Self {
                self.args.cancel_token = Some(token);
                self
            }

            /// Escape hatch to pass a raw, custom argument directly to the Docker CLI.
            pub fn custom_arg(mut self, arg: impl Into<String>) -> Self {
                self.args.push(arg.into());
                self
            }

            /// Escape hatch to pass multiple raw, custom arguments directly to the Docker CLI.
            pub fn custom_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
                self.args.push_all(args);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_compose_opts {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
            /// Specify an alternate compose file
            pub fn file(mut self, f: impl Into<String>) -> Self {
                self.args.insert_pair(1, "--file", f.into());
                self
            }

            /// Specify an alternate environment file
            pub fn env_file(mut self, f: impl Into<String>) -> Self {
                self.args.insert_pair(1, "--env-file", f.into());
                self
            }

            /// Specify an alternate project name
            pub fn project(mut self, p: impl Into<String>) -> Self {
                self.args.insert_pair(1, "--project-name", p.into());
                self
            }
            
            /// Specify a profile to enable
            pub fn profile(mut self, p: impl Into<String>) -> Self {
                self.args.insert_pair(1, "--profile", p.into());
                self
            }
        }
    };
}
