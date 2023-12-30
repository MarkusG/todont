use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::auth::ApplicationClaims;
use crate::repository::DynTodoRepository;

pub async fn get_todo(
    Path(id): Path<Uuid>,
    claims: ApplicationClaims,
    Extension(repo_mutex): Extension<DynTodoRepository>) -> impl IntoResponse {
    let repo = repo_mutex.lock().await;
    if let Ok(todo) = repo.get(id).await {
        if todo.username == claims.username {
            return todo.into_response();
        }
    }
    return StatusCode::NOT_FOUND.into_response();
}
