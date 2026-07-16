#[derive(Debug, Clone)]
pub struct S3Destination {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub provider: Option<String>,
    pub additional_flags: Option<String>,
}

impl S3Destination {
    pub fn rclone_flags(&self) -> Vec<String> {
        let mut flags = vec![
            format!("--s3-access-key-id=\"{}\"",     self.access_key),
            format!("--s3-secret-access-key=\"{}\"", self.secret_key),
            format!("--s3-region=\"{}\"",             self.region),
            format!("--s3-endpoint=\"{}\"",           self.endpoint),
            "--s3-no-check-bucket".to_string(),
            "--s3-force-path-style".to_string(),
        ];
        if let Some(provider) = &self.provider {
            flags.insert(0, format!("--s3-provider=\"{provider}\""));
        }
        if let Some(extra) = &self.additional_flags {
            for flag in extra.split_whitespace() {
                flags.push(flag.to_string());
            }
        }
        flags
    }

    pub fn rclone_upload_args(&self, object_path: &str) -> Vec<String> {
        let mut args = vec!["rcat".to_string()];
        args.extend(self.rclone_flags());
        args.push(format!(":s3:{}/{}", self.bucket, object_path));
        args
    }

    pub fn rclone_list_args(&self, prefix: &str) -> Vec<String> {
        let mut args = vec!["lsf".to_string()];
        args.extend(self.rclone_flags());
        args.push(format!(":s3:{}/{}", self.bucket, prefix));
        args
    }

    pub fn rclone_delete_args(&self, object_path: &str) -> Vec<String> {
        let mut args = vec!["delete".to_string()];
        args.extend(self.rclone_flags());
        args.push(format!(":s3:{}/{}", self.bucket, object_path));
        args
    }
}
