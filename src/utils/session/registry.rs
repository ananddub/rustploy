use crate::utils::exec::RemoteExecutor;
use dashmap::DashMap;
use std::sync::OnceLock;

/// Process-wide remote executor cache. Each executor owns a bounded SSH session
/// pool, so API checks and deployments reuse connections to the same server.
pub struct RemoteExecutorRegistry {
    entries: DashMap<i64, (u64, RemoteExecutor)>,
}

impl RemoteExecutorRegistry {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<RemoteExecutorRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            entries: DashMap::new(),
        })
    }

    pub fn get(&self, server_id: i64, version: u64) -> Option<RemoteExecutor> {
        self.entries
            .get(&server_id)
            .filter(|entry| entry.value().0 == version)
            .map(|entry| entry.value().1.clone())
    }

    pub fn current(&self, server_id: i64) -> Option<RemoteExecutor> {
        self.entries
            .get(&server_id)
            .map(|entry| entry.value().1.clone())
    }

    pub fn insert(&self, server_id: i64, version: u64, executor: RemoteExecutor) {
        self.entries.insert(server_id, (version, executor));
    }

    pub fn remove(&self, server_id: i64) -> Option<RemoteExecutor> {
        self.entries.remove(&server_id).map(|(_, (_, value))| value)
    }
}
