use axum::Extension;
use axum::response::IntoResponse;

use crate::repository::DynTodoRepository;

pub async fn get_todos(
    Extension(repo): Extension<DynTodoRepository>) -> impl IntoResponse {
    axum::Json(repo.get_all().await.to_vec())
}
