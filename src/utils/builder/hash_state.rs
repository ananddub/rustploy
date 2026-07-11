use crate::utils::builder::custom_type::{AppDeploy, DeployState, IdType};
use auto_di::singleton;
use dashmap::DashMap;
use tokio::sync::{broadcast, watch};
use tokio_util::sync::CancellationToken;

pub struct ApplicationState {
    pub hashmap: DashMap<IdType, AppDeploy>,
}
#[singleton]
impl ApplicationState {
    pub fn new() -> Self {
        Self {
            hashmap: DashMap::new(),
        }
    }
    pub fn set_state(&self, app_id: IdType, state: AppDeploy) {
        self.hashmap.insert(app_id, state);
    }

    pub fn add_default(&self, app_id: IdType, env_id: i64, project_id: i64) {
        let app_deploy = AppDeploy {
            app_id: app_id.clone(),
            project_id,
            env_id,
            state: watch::channel(DeployState::Queue).0,
            broadcast: broadcast::channel(100).0,
            cancellation_token: CancellationToken::new(),
        };
        self.hashmap.insert(app_id, app_deploy);
    }

    pub fn remove_state(&self, app_id: IdType) {
        self.hashmap.remove(&app_id);
    }

    pub fn stop(&self, app_id: AppDeploy) -> Result<(), String> {
        self.hashmap
            .get(&app_id.app_id)
            .map(|entry| {
                entry.cancellation_token.cancel();
            })
            .ok_or_else(|| "AppDeploy not found".to_string())
    }

    pub fn state_recv(&self, app_id: IdType) -> Option<watch::Receiver<DeployState>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.state.subscribe().clone())
    }

    pub async fn broadcast_recv(&self, app_id: IdType) -> Option<broadcast::Receiver<String>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.broadcast.subscribe())
    }

    pub fn get_broadcast_send(&self, app_id: IdType) -> Option<broadcast::Sender<String>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.broadcast.clone())
    }
    pub fn get_state_send(&self, app_id: IdType) -> Option<watch::Sender<DeployState>> {
        self.hashmap.get(&app_id).map(|entry| entry.state.clone())
    }

    pub fn send_state(&self, app_id: IdType, state: DeployState) {
        self.hashmap.get(&app_id).map(|entry| {
            let _ = entry.state.send(state);
        });
    }

    pub fn send_broadcast(&self, app_id: IdType, event: String) {
        self.hashmap.get(&app_id).map(|entry| {
            let _ = entry.broadcast.send(event.clone());
        });
    }
}
