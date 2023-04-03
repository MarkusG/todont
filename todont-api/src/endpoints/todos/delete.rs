use axum::{extract::Path, Extension};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::repository::DynTodoRepository;

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    Extension(repo_mutex): Extension<DynTodoRepository>) -> impl IntoResponse {
    let mut repo = repo_mutex.lock().await;
    let deleted = repo.delete(id).await;
    if deleted { StatusCode::OK } else { StatusCode::NOT_FOUND }
}
