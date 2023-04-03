use axum::Extension;
use axum::response::IntoResponse;

use crate::repository::DynTodoRepository;

pub async fn get_todos(
    Extension(repo_mutex): Extension<DynTodoRepository>) -> impl IntoResponse {
    let repo = repo_mutex.lock().await;
    axum::Json(repo.get_all().await.to_vec())
}
