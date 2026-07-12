use std::{convert::Infallible, pin::Pin, sync::Arc};

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::Stream;
use serde_json::json;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{Duration, MissedTickBehavior};

use crate::{
    api::dto::deployment::{
        ActiveDeploymentDto, ComposeLogQuery, DeploymentListQuery, DeploymentResponseDto,
        DockerLogQuery, DockerStatsQuery,
    },
    services::deployment::{CancelDeploymentResult, DeploymentListFilter, DeploymentService},
    utils::{
        builder::custom_type::{DeployEvent, DeployState, DeploySubscription, IdType},
        docker::DockerStreamEvent,
        jwt::claim::Claims,
    },
};

type ApiError = (StatusCode, String);
type DeploymentEventStream = Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;
type DeploymentSse = Sse<DeploymentEventStream>;

pub struct DeploymentController {
    service: Arc<DeploymentService>,
}

#[controller("/deployments")]
impl DeploymentController {
    fn new(service: Arc<DeploymentService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        _claims: Claims,
        Query(query): Query<DeploymentListQuery>,
    ) -> Result<Json<Vec<DeploymentResponseDto>>, ApiError> {
        self.service
            .list(DeploymentListFilter {
                status: query.status,
                state: query.state,
                application_id: query.application_id,
                compose_id: query.compose_id,
                server_id: query.server_id,
                limit: query.limit.unwrap_or(50),
                offset: query.offset.unwrap_or(0),
            })
            .await
            .map(|items| items.into_iter().map(DeploymentResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/active")]
    async fn active(&self, _claims: Claims) -> Result<Json<Vec<ActiveDeploymentDto>>, ApiError> {
        self.service
            .list_active_components()
            .await
            .map(|items| items.into_iter().map(ActiveDeploymentDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/running")]
    async fn running(
        &self,
        _claims: Claims,
        Query(query): Query<DeploymentListQuery>,
    ) -> Result<Json<Vec<DeploymentResponseDto>>, ApiError> {
        self.service
            .list_running(query.limit.unwrap_or(50), query.offset.unwrap_or(0))
            .await
            .map(|items| items.into_iter().map(DeploymentResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<DeploymentResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(DeploymentResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{id}/events")]
    async fn application_events(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        self.events(IdType::AppId(id)).await
    }

    #[get("/compose/{id}/events")]
    async fn compose_events(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        self.events(IdType::ComposeId(id)).await
    }

    #[get("/application/{id}/stats")]
    async fn application_stats(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_application_stats(id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/compose/{id}/stats")]
    async fn compose_stats(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_compose_stats(id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/container/{target}/logs")]
    async fn docker_container_logs(
        &self,
        _claims: Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_container_logs(
                query.server_id,
                target,
                docker_log_options(query.tail, query.timestamps, query.follow),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[get("/docker/stats")]
    async fn docker_global_stats(
        &self,
        _claims: Claims,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_global_stats(query.server_id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/container/{target}/stats")]
    async fn docker_container_stats(
        &self,
        _claims: Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_container_stats(query.server_id, target, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/service/{target}/logs")]
    async fn docker_service_logs(
        &self,
        _claims: Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_service_logs(
                query.server_id,
                target,
                docker_log_options(query.tail, query.timestamps, query.follow),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[get("/docker/compose/logs")]
    async fn docker_compose_logs(
        &self,
        _claims: Claims,
        Query(query): Query<ComposeLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let server_id = query.server_id;
        let args = compose_log_args(query);
        let receiver = self
            .service
            .stream_docker_compose_logs(server_id, args)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[post("/{id}/cancel")]
    async fn cancel(&self, _claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        match self.service.cancel(id).await {
            Ok(CancelDeploymentResult::CancelRequested) => Ok(StatusCode::ACCEPTED),
            Ok(CancelDeploymentResult::NotRunning) => Err((
                StatusCode::CONFLICT,
                "deployment is not running, so it cannot be cancelled".into(),
            )),
            Ok(CancelDeploymentResult::NotCancellable) => Err((
                StatusCode::BAD_REQUEST,
                "deployment is not attached to an application or compose project".into(),
            )),
            Ok(CancelDeploymentResult::NotActiveInThisProcess) => Err((
                StatusCode::CONFLICT,
                "deployment is not active in this process; it may already be finished or recovered after restart".into(),
            )),
            Err(error) => Err(map_sqlx_error(error)),
        }
    }

    async fn events(&self, id: IdType) -> Result<DeploymentSse, ApiError> {
        let Some(subscription) = self
            .service
            .subscribe_component(id)
            .await
            .map_err(map_sqlx_error)?
        else {
            return Err((
                StatusCode::NOT_FOUND,
                "active deployment stream not found".into(),
            ));
        };

        Ok(Sse::new(deployment_event_stream(subscription)))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "deployment not found".into()),
        other => {
            tracing::error!(error = %other, "deployment operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "deployment operation failed".into(),
            )
        }
    }
}

fn deployment_event_stream(subscription: DeploySubscription) -> DeploymentEventStream {
    let initial_state = subscription.state.borrow().clone();
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (subscription, Some(initial_state), keep_alive),
        |(mut subscription, initial_state, mut keep_alive)| async move {
            if let Some(state) = initial_state {
                return Some((Ok(state_event(state)), (subscription, None, keep_alive)));
            }

            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (subscription, None, keep_alive)));
                    }
                    changed = subscription.state.changed() => {
                        if changed.is_err() {
                            return None;
                        }

                        let state = subscription.state.borrow().clone();
                        return Some((Ok(state_event(state)), (subscription, None, keep_alive)));
                    }
                    received = subscription.events.recv() => {
                        match received {
                            Ok(event) => {
                                return Some((Ok(deploy_event(event)), (subscription, None, keep_alive)));
                            }
                            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                                return Some((Ok(lagged_event(skipped)), (subscription, None, keep_alive)));
                            }
                            Err(broadcast::error::RecvError::Closed) => {
                                return None;
                            }
                        }
                    }
                }
            }
        },
    ))
}

fn keep_alive_event() -> Event {
    Event::default()
        .event("keep-alive")
        .data(json_payload(json!({
            "type": "keep-alive",
        })))
}

fn state_event(state: DeployState) -> Event {
    Event::default().event("state").data(json_payload(json!({
        "type": "state",
        "state": format!("{state:?}"),
    })))
}

fn deploy_event(event: DeployEvent) -> Event {
    match event {
        DeployEvent::StateChanged(state) => state_event(state),
        DeployEvent::Log(line) => Event::default().event("log").data(json_payload(json!({
            "type": "log",
            "line": line,
        }))),
        DeployEvent::Message(message) => {
            Event::default().event("message").data(json_payload(json!({
                "type": "message",
                "message": message,
            })))
        }
    }
}

fn lagged_event(skipped: u64) -> Event {
    Event::default().event("lagged").data(json_payload(json!({
        "type": "lagged",
        "skipped": skipped,
    })))
}

fn json_payload(value: serde_json::Value) -> String {
    serde_json::to_string(&value).unwrap_or_else(|_| "{}".into())
}

fn docker_stream(receiver: mpsc::Receiver<DockerStreamEvent>) -> DeploymentEventStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        let event = match received {
                            Some(DockerStreamEvent::Stdout(bytes)) => docker_log_event("stdout", bytes),
                            Some(DockerStreamEvent::Stderr(bytes)) => docker_log_event("stderr", bytes),
                            None => return None,
                        };
                        return Some((Ok(event), (receiver, keep_alive)));
                    }
                }
            }
        },
    ))
}

fn docker_log_event(stream: &str, bytes: Vec<u8>) -> Event {
    Event::default().event(stream).data(json_payload(json!({
        "type": stream,
        "line": String::from_utf8_lossy(&bytes),
    })))
}

fn docker_stats_stream(receiver: mpsc::Receiver<DockerStreamEvent>) -> DeploymentEventStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        let event = match received {
                            Some(DockerStreamEvent::Stdout(bytes)) => docker_stats_event(bytes),
                            Some(DockerStreamEvent::Stderr(bytes)) => docker_log_event("stderr", bytes),
                            None => return None,
                        };
                        return Some((Ok(event), (receiver, keep_alive)));
                    }
                }
            }
        },
    ))
}

fn docker_stats_event(bytes: Vec<u8>) -> Event {
    let line = String::from_utf8_lossy(&bytes);
    let value = serde_json::from_str::<serde_json::Value>(line.trim()).unwrap_or_else(|_| {
        json!({
            "raw": line,
        })
    });

    Event::default().event("stats").data(json_payload(json!({
        "type": "stats",
        "stats": value,
    })))
}

fn docker_log_options(
    tail: Option<usize>,
    timestamps: Option<bool>,
    follow: Option<bool>,
) -> Vec<String> {
    let mut args = Vec::new();
    if follow.unwrap_or(true) {
        args.push("--follow".into());
    }
    if timestamps.unwrap_or(false) {
        args.push("--timestamps".into());
    }
    let tail = tail.unwrap_or(200).to_string();
    args.extend(["--tail".into(), tail]);
    args
}

fn compose_log_args(query: ComposeLogQuery) -> Vec<String> {
    let mut args = vec!["compose".into()];

    if let Some(file) = query.file {
        args.extend(["--file".into(), file]);
    }
    if let Some(project_dir) = query.project_dir {
        args.extend(["--project-directory".into(), project_dir]);
    }
    if let Some(project_name) = query.project_name {
        args.extend(["--project-name".into(), project_name]);
    }

    args.push("logs".into());
    args.extend(docker_log_options(
        query.tail,
        query.timestamps,
        query.follow,
    ));

    if let Some(service) = query.service {
        args.push(service);
    }

    args
}
