use crate::utils::exec::{ExecError, ExecResult};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, Notify, OwnedSemaphorePermit, Semaphore};

#[derive(Debug)]
pub struct SshSessionPool {
    connections: Mutex<Vec<Arc<SshConnection>>>,
    connection_permits: Arc<Semaphore>,
    notify: Arc<Notify>,
    max_size: usize,
    max_channels_per_session: usize,
    next_id: AtomicU64,
}

#[derive(Debug)]
pub(crate) struct SshConnection {
    id: u64,
    session: russh_extra::Session,
    channels: Arc<Semaphore>,
    _connection_permit: OwnedSemaphorePermit,
}

#[derive(Debug)]
pub(crate) enum SshSessionLease {
    Existing {
        connection: Arc<SshConnection>,
        _channel: ChannelPermit,
    },
    New {
        connection_permit: OwnedSemaphorePermit,
    },
}

#[derive(Debug)]
pub(crate) struct ChannelPermit {
    _permit: OwnedSemaphorePermit,
    notify: Arc<Notify>,
}

impl Drop for ChannelPermit {
    fn drop(&mut self) {
        self.notify.notify_one();
    }
}

impl SshConnection {
    pub(crate) fn session(&self) -> &russh_extra::Session {
        &self.session
    }
}

impl SshSessionLease {
    pub(crate) fn session(&self) -> Option<&russh_extra::Session> {
        match self {
            Self::Existing { connection, .. } => Some(connection.session()),
            Self::New { .. } => None,
        }
    }

    fn connection_id(&self) -> Option<u64> {
        match self {
            Self::Existing { connection, .. } => Some(connection.id),
            Self::New { .. } => None,
        }
    }
}

impl SshSessionPool {
    pub fn new(max_size: usize) -> Arc<Self> {
        Self::new_with_channels(max_size, 5)
    }

    pub fn new_with_channels(max_size: usize, max_channels_per_session: usize) -> Arc<Self> {
        let max_size = max_size.max(1);
        let max_channels_per_session = max_channels_per_session.max(1);
        Arc::new(Self {
            connections: Mutex::new(Vec::with_capacity(max_size)),
            connection_permits: Arc::new(Semaphore::new(max_size)),
            notify: Arc::new(Notify::new()),
            max_size,
            max_channels_per_session,
            next_id: AtomicU64::new(1),
        })
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn max_channels_per_session(&self) -> usize {
        self.max_channels_per_session
    }

    pub async fn idle_count(&self) -> usize {
        self.connections
            .lock()
            .await
            .iter()
            .filter(|connection| {
                connection.session.is_connected()
                    && connection.channels.available_permits() == self.max_channels_per_session
            })
            .count()
    }

    pub async fn connection_count(&self) -> usize {
        self.connections
            .lock()
            .await
            .iter()
            .filter(|connection| connection.session.is_connected())
            .count()
    }

    pub async fn active_channel_count(&self) -> usize {
        self.connections
            .lock()
            .await
            .iter()
            .filter(|connection| connection.session.is_connected())
            .map(|connection| {
                self.max_channels_per_session
                    .saturating_sub(connection.channels.available_permits())
            })
            .sum()
    }

    pub async fn clear(&self) {
        self.connections.lock().await.clear();
        self.notify.notify_waiters();
    }

    pub(crate) async fn acquire(&self) -> ExecResult<SshSessionLease> {
        loop {
            let notified = self.notify.notified();
            if let Some(lease) = self.try_acquire_existing().await {
                return Ok(lease);
            }
            match self.connection_permits.clone().try_acquire_owned() {
                Ok(connection_permit) => return Ok(SshSessionLease::New { connection_permit }),
                Err(tokio::sync::TryAcquireError::Closed) => {
                    return Err(ExecError::Ssh("SSH session pool is closed".into()));
                }
                Err(tokio::sync::TryAcquireError::NoPermits) => notified.await,
            }
        }
    }

    async fn try_acquire_existing(&self) -> Option<SshSessionLease> {
        let mut connections = self.connections.lock().await;
        connections.retain(|connection| connection.session.is_connected());
        for connection in connections.iter() {
            if let Ok(channel_permit) = connection.channels.clone().try_acquire_owned() {
                return Some(SshSessionLease::Existing {
                    connection: Arc::clone(connection),
                    _channel: ChannelPermit {
                        _permit: channel_permit,
                        notify: Arc::clone(&self.notify),
                    },
                });
            }
        }
        None
    }

    pub(crate) async fn attach(
        &self,
        session: russh_extra::Session,
        connection_permit: OwnedSemaphorePermit,
    ) -> ExecResult<SshSessionLease> {
        let channels = Arc::new(Semaphore::new(self.max_channels_per_session));
        let channel_permit = channels
            .clone()
            .try_acquire_owned()
            .map_err(|_| ExecError::Ssh("new SSH connection has no channel permits".into()))?;
        let connection = Arc::new(SshConnection {
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            session,
            channels,
            _connection_permit: connection_permit,
        });
        self.connections.lock().await.push(Arc::clone(&connection));
        Ok(SshSessionLease::Existing {
            connection,
            _channel: ChannelPermit {
                _permit: channel_permit,
                notify: Arc::clone(&self.notify),
            },
        })
    }

    pub(crate) async fn discard(&self, lease: &SshSessionLease) {
        if let Some(id) = lease.connection_id() {
            self.connections
                .lock()
                .await
                .retain(|connection| connection.id != id);
            self.notify.notify_waiters();
        }
    }
}
