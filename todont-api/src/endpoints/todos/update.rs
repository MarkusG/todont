use axum::Json;
use axum::Extension;
use axum::extract::Path;
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
    repo.update(&todo).await.into_response()
}
