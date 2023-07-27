use axum::Json;
use axum::Extension;
use axum::response::IntoResponse;

use crate::models::{Todo, CreateTodoRequest};
use crate::repository::DynTodoRepository;

pub async fn create_todo(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    Json(body): Json<CreateTodoRequest>) -> impl IntoResponse {
    let todo = Todo::from_create(body, "mark".to_string());
    let mut repo = repo_mutex.lock().await;
    repo.create(&todo).await.into_response()
}
