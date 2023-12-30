use axum::{extract::Path, Extension};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::auth::ApplicationClaims;
use crate::repository::DynTodoRepository;

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    claims: ApplicationClaims,
    Extension(repo_mutex): Extension<DynTodoRepository>) -> impl IntoResponse {
    let mut repo = repo_mutex.lock().await;
    if let Ok(todo) = repo.get(id).await {
        if todo.username == claims.username {
            repo.delete(id).await;
            return StatusCode::OK;
        }
    }
    return StatusCode::NOT_FOUND;
}
