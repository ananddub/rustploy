use std::collections::HashMap;
use super::command::RcloneCommand;
use super::target::RcloneTarget;

pub struct RcloneBuilder {
    command: RcloneCommand,
    source: Option<RcloneTarget>,
    destination: Option<RcloneTarget>,

    // Performance & Optimization
    transfers: Option<u32>,
    checkers: Option<u32>,
    buffer_size: Option<String>,
    bwlimit: Option<String>,
    fast_list: bool,
    use_mmap: bool,

    // Timeouts & Retries
    retries: Option<u32>,
    low_level_retries: Option<u32>,
    timeout: Option<String>,
    connect_timeout: Option<String>,
    retry_delay: Option<String>,

    // Safety & Flags
    dry_run: bool,
    check_first: bool,
    ignore_errors: bool,
    ignore_existing: bool,
    update: bool,
    inplace: bool,

    // Logging
    log_file: Option<String>,
    log_level: Option<String>,
    stats: Option<String>,

    // Extensibility
    additional_flags: Vec<String>,
}

impl RcloneBuilder {
    pub fn new(command: RcloneCommand) -> Self {
        Self {
            command,
            source: None,
            destination: None,
            transfers: None,
            checkers: None,
            buffer_size: None,
            bwlimit: None,
            fast_list: false,
            use_mmap: false,
            retries: None,
            low_level_retries: None,
            timeout: None,
            connect_timeout: None,
            retry_delay: None,
            dry_run: false,
            check_first: false,
            ignore_errors: false,
            ignore_existing: false,
            update: false,
            inplace: false,
            log_file: None,
            log_level: None,
            stats: None,
            additional_flags: Vec::new(),
        }
    }

    pub fn source(mut self, target: RcloneTarget) -> Self {
        self.source = Some(target);
        self
    }

    pub fn destination(mut self, target: RcloneTarget) -> Self {
        self.destination = Some(target);
        self
    }

    pub fn transfers(mut self, transfers: u32) -> Self {
        self.transfers = Some(transfers);
        self
    }

    pub fn checkers(mut self, checkers: u32) -> Self {
        self.checkers = Some(checkers);
        self
    }

    pub fn buffer_size(mut self, buffer_size: impl Into<String>) -> Self {
        self.buffer_size = Some(buffer_size.into());
        self
    }

    pub fn bwlimit(mut self, bwlimit: impl Into<String>) -> Self {
        self.bwlimit = Some(bwlimit.into());
        self
    }

    pub fn fast_list(mut self) -> Self {
        self.fast_list = true;
        self
    }

    pub fn use_mmap(mut self) -> Self {
        self.use_mmap = true;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn low_level_retries(mut self, low_level_retries: u32) -> Self {
        self.low_level_retries = Some(low_level_retries);
        self
    }

    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: impl Into<String>) -> Self {
        self.connect_timeout = Some(connect_timeout.into());
        self
    }

    pub fn retry_delay(mut self, retry_delay: impl Into<String>) -> Self {
        self.retry_delay = Some(retry_delay.into());
        self
    }

    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    pub fn check_first(mut self) -> Self {
        self.check_first = true;
        self
    }

    pub fn ignore_errors(mut self) -> Self {
        self.ignore_errors = true;
        self
    }

    pub fn ignore_existing(mut self) -> Self {
        self.ignore_existing = true;
        self
    }

    pub fn update(mut self) -> Self {
        self.update = true;
        self
    }

    pub fn inplace(mut self) -> Self {
        self.inplace = true;
        self
    }

    pub fn log_file(mut self, log_file: impl Into<String>) -> Self {
        self.log_file = Some(log_file.into());
        self
    }

    pub fn log_level(mut self, log_level: impl Into<String>) -> Self {
        self.log_level = Some(log_level.into());
        self
    }

    pub fn stats(mut self, stats: impl Into<String>) -> Self {
        self.stats = Some(stats.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.additional_flags.push(arg.into());
        self
    }

    pub fn build(self) -> (Vec<String>, HashMap<String, String>) {
        let mut args = Vec::new();
        let mut envs = HashMap::new();

        args.push(self.command.as_str().to_string());

        if let Some(t) = self.transfers {
            args.push(format!("--transfers={}", t));
        }
        if let Some(c) = self.checkers {
            args.push(format!("--checkers={}", c));
        }
        if let Some(ref b) = self.buffer_size {
            args.push(format!("--buffer-size={}", b));
        }
        if let Some(ref bw) = self.bwlimit {
            args.push(format!("--bwlimit={}", bw));
        }
        if self.fast_list {
            args.push("--fast-list".to_string());
        }
        if self.use_mmap {
            args.push("--use-mmap".to_string());
        }

        if let Some(r) = self.retries {
            args.push(format!("--retries={}", r));
        }
        if let Some(lr) = self.low_level_retries {
            args.push(format!("--low-level-retries={}", lr));
        }
        if let Some(ref t) = self.timeout {
            args.push(format!("--timeout={}", t));
        }
        if let Some(ref ct) = self.connect_timeout {
            args.push(format!("--contimeout={}", ct));
        }
        if let Some(ref rd) = self.retry_delay {
            args.push(format!("--retry-delay={}", rd));
        }

        if self.dry_run {
            args.push("--dry-run".to_string());
        }
        if self.check_first {
            args.push("--check-first".to_string());
        }
        if self.ignore_errors {
            args.push("--ignore-errors".to_string());
        }
        if self.ignore_existing {
            args.push("--ignore-existing".to_string());
        }
        if self.update {
            args.push("--update".to_string());
        }
        if self.inplace {
            args.push("--inplace".to_string());
        }

        if let Some(ref lf) = self.log_file {
            args.push(format!("--log-file={}", lf));
        }
        if let Some(ref ll) = self.log_level {
            args.push(format!("--log-level={}", ll));
        }
        if let Some(ref st) = self.stats {
            args.push(format!("--stats={}", st));
        }

        args.extend(self.additional_flags);

        if let Some(ref src) = self.source {
            let (target_path, target_envs) = src.compile("src");
            args.push(target_path);
            envs.extend(target_envs);
        }

        if let Some(ref dest) = self.destination {
            let (target_path, target_envs) = dest.compile("dest");
            args.push(target_path);
            envs.extend(target_envs);
        }

        (args, envs)
    }
}
