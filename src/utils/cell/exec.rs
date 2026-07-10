pub mod Exec {
    use std::io::Read;
    use std::process::Stdio;
    use tokio::io::AsyncReadExt;
    use tokio::process::Command;

    pub async fn exec(cmd: &str, args: &[&str]) -> Result<(), String> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        let mut stdout_task = child.stdout.take().unwrap();
        let mut stderr_task = child.stderr.take().unwrap();
        let t1 = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let n = stdout_task.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                dbg!("{}", String::from_utf8_lossy(&buf[..n]));
            }
        });
        let t2 = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let n = stderr_task.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                dbg!("{}", String::from_utf8_lossy(&buf[..n]));
            }
        });
        tokio::try_join!(t1, t2).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_exec() {
        let result = super::Exec::exec("ping", &["google.com"]).await;
        assert!(result.is_ok());
    }
}
