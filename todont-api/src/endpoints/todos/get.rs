use axum::{extract::Path, Extension};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::repository::DynTodoRepository;

pub async fn get_todo(
    Path(id): Path<Uuid>,
    Extension(repo_mutex): Extension<DynTodoRepository>) -> impl IntoResponse {
    let repo = repo_mutex.lock().await;
    repo.get(id).await.into_response()
}
