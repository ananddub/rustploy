use crate::utils::exec::{ExecError, ExecResult};
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};

#[derive(Debug)]
pub struct SshSessionPool {
    idle: Mutex<Vec<russh_extra::Session>>,
    permits: Arc<Semaphore>,
    max_size: usize,
}

impl SshSessionPool {
    pub fn new(max_size: usize) -> Arc<Self> {
        let max_size = max_size.max(1);
        Arc::new(Self {
            idle: Mutex::new(Vec::with_capacity(max_size)),
            permits: Arc::new(Semaphore::new(max_size)),
            max_size,
        })
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }
    pub async fn idle_count(&self) -> usize {
        self.idle.lock().await.len()
    }
    pub async fn clear(&self) {
        self.idle.lock().await.clear();
    }

    pub(crate) async fn acquire(
        &self,
    ) -> ExecResult<(Option<russh_extra::Session>, OwnedSemaphorePermit)> {
        let permit = self
            .permits
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| ExecError::Ssh("SSH session pool is closed".into()))?;
        let session = self.idle.lock().await.pop();
        Ok((session, permit))
    }

    pub(crate) async fn release(&self, session: russh_extra::Session) {
        let mut idle = self.idle.lock().await;
        if idle.len() < self.max_size && session.is_connected() {
            idle.push(session);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn limits_concurrent_session_borrowers() {
        let pool = SshSessionPool::new(1);
        let (_, permit) = pool.acquire().await.unwrap();
        let waiting = pool.clone();
        assert!(
            tokio::time::timeout(Duration::from_millis(30), waiting.acquire())
                .await
                .is_err()
        );
        drop(permit);
        let (_, second) = tokio::time::timeout(Duration::from_secs(1), pool.acquire())
            .await
            .unwrap()
            .unwrap();
        drop(second);
        assert_eq!(pool.max_size(), 1);
        assert_eq!(pool.idle_count().await, 0);
    }
}
