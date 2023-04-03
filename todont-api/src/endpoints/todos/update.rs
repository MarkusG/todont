use axum::Json;
use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::models::{Todo, UpdateTodoRequest};
use crate::repository::DynTodoRepository;

pub async fn update_todo(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTodoRequest>) -> impl IntoResponse {
    let todo = Todo::from_update(body, id);
    let mut repo = repo_mutex.lock().await;
    // TODO handle more cases, like validation or not found
    if let Some(updated) = repo.update(&todo).await {
        return (StatusCode::OK, axum::Json(updated)).into_response();
    }
    else {
        return StatusCode::BAD_REQUEST.into_response();
    }
}
