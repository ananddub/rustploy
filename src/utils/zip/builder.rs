use std::path::PathBuf;
use crate::utils::exec::{CommandExecutor, ExecError};


#[derive(Debug, thiserror::Error)]
pub enum ZipError {
    #[error("source path not set")]
    MissingSource,
    #[error("destination path not set")]
    MissingDestination,
    #[error("execution failed: {0}")]
    Exec(#[from] ExecError),
    #[error("command failed: {0}")]
    Failed(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}


#[derive(Debug, Clone, Copy, Default)]
pub enum CompressionLevel {
    Stored,   // -0  no compression, fastest
    Fast,     // -1
    #[default]
    Default,  // -6  zip default
    Best,     // -9  max compression
}

impl CompressionLevel {
    fn flag(&self) -> &'static str {
        match self {
            Self::Stored  => "-0",
            Self::Fast    => "-1",
            Self::Default => "-6",
            Self::Best    => "-9",
        }
    }
}


pub struct ZipBuilder<'e> {
    executor: &'e CommandExecutor,

    source:      Option<PathBuf>,
    destination: Option<PathBuf>,

    // zip options
    recurse:      bool,
    junk_paths:   bool,
    compression:  CompressionLevel,
    excludes:     Vec<String>,

    // unzip options
    overwrite:    bool,
    list_only:    bool,
    only_files:   Vec<String>,

    extra_args:   Vec<String>,
}

impl<'e> ZipBuilder<'e> {
    pub fn new(executor: &'e CommandExecutor) -> Self {
        Self {
            executor,
            source:      None,
            destination: None,
            recurse:     false,
            junk_paths:  false,
            compression: CompressionLevel::Default,
            excludes:    Vec::new(),
            overwrite:   false,
            list_only:   false,
            only_files:  Vec::new(),
            extra_args:  Vec::new(),
        }
    }


    pub fn source(mut self, path: impl Into<PathBuf>) -> Self {
        self.source = Some(path.into());
        self
    }

    pub fn destination(mut self, path: impl Into<PathBuf>) -> Self {
        self.destination = Some(path.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.extra_args.push(arg.into());
        self
    }


    /// `-r` recurse into directories.
    pub fn recurse(mut self) -> Self {
        self.recurse = true;
        self
    }

    /// `-j` strip directory paths — store filenames only.
    pub fn junk_paths(mut self) -> Self {
        self.junk_paths = true;
        self
    }

    /// Compression level (default: `CompressionLevel::Default`).
    pub fn compression(mut self, level: CompressionLevel) -> Self {
        self.compression = level;
        self
    }

    /// `-x <pattern>` exclude matching entries (works for both zip and unzip).
    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        self.excludes.push(pattern.into());
        self
    }


    /// `-o` overwrite existing files without prompting.
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// `-l` list archive contents, don't extract.
    pub fn list_only(mut self) -> Self {
        self.list_only = true;
        self
    }

    /// Extract only these specific files from the archive.
    pub fn only(mut self, files: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.only_files.extend(files.into_iter().map(|f| f.into()));
        self
    }


    pub async fn zip(self) -> Result<(), ZipError> {
        let source = self.source.as_ref().ok_or(ZipError::MissingSource)?;
        let dest   = self.destination.as_ref().ok_or(ZipError::MissingDestination)?;

        let mut args: Vec<String> = vec!["zip".into()];
        if self.recurse    { args.push("-r".into()); }
        if self.junk_paths { args.push("-j".into()); }
        args.push(self.compression.flag().into());
        for pat in &self.excludes {
            args.push("-x".into());
            args.push(pat.clone());
        }
        args.extend(self.extra_args.iter().cloned());
        args.push(dest.to_string_lossy().into_owned());
        args.push(source.to_string_lossy().into_owned());

        let out = self.executor
            .run("zip", &args[1..])
            .await?;

        if !out.status.success() {
            return Err(ZipError::Failed(out.stderr));
        }
        Ok(())
    }


    pub async fn unzip(self) -> Result<(), ZipError> {
        let source = self.source.as_ref().ok_or(ZipError::MissingSource)?;

        let mut args: Vec<String> = vec!["unzip".into()];
        if self.overwrite  { args.push("-o".into()); }
        if self.list_only  { args.push("-l".into()); }
        args.extend(self.extra_args.iter().cloned());
        args.push(source.to_string_lossy().into_owned());
        args.extend(self.only_files.iter().cloned());
        if let Some(ref dest) = self.destination {
            args.push("-d".into());
            args.push(dest.to_string_lossy().into_owned());
        }

        let out = self.executor
            .run("unzip", &args[1..])
            .await?;

        if !out.status.success() {
            return Err(ZipError::Failed(out.stderr));
        }
        Ok(())
    }
}
