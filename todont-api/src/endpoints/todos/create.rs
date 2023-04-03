use axum::Json;
use axum::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::models::{Todo, CreateTodoRequest};
use crate::repository::DynTodoRepository;

pub async fn create_todo(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    Json(body): Json<CreateTodoRequest>) -> impl IntoResponse {
    let todo = Todo::from_create(body);
    let mut repo = repo_mutex.lock().await;
    if let Some(created) = repo.create(&todo).await {
        return (StatusCode::OK, axum::Json(created)).into_response();
    }
    else {
        return StatusCode::BAD_REQUEST.into_response();
    }
}
