use std::sync::Arc;

#[derive(Debug)]
pub struct SshSessionPool {
    max_size: usize,
    max_channels_per_session: usize,
}

impl SshSessionPool {
    pub fn new(max_size: usize) -> Arc<Self> {
        Self::new_with_channels(max_size, 5)
    }

    pub fn new_with_channels(max_size: usize, max_channels_per_session: usize) -> Arc<Self> {
        Arc::new(Self {
            max_size,
            max_channels_per_session,
        })
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn max_channels_per_session(&self) -> usize {
        self.max_channels_per_session
    }

    pub async fn idle_count(&self) -> usize {
        0
    }

    pub async fn connection_count(&self) -> usize {
        0
    }

    pub async fn active_channel_count(&self) -> usize {
        0
    }

    pub async fn clear(&self) {}
}
