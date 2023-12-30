use axum::Json;
use axum::http::StatusCode;
use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::auth::ApplicationClaims;
use crate::models::{Todo, UpdateTodoRequest};
use crate::repository::DynTodoRepository;

pub async fn update_todo(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    claims: ApplicationClaims,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTodoRequest>) -> impl IntoResponse {
    let todo = Todo::from_update(body, id, claims.username);
    let mut repo = repo_mutex.lock().await;
    if let Ok(existing) = repo.get(id).await {
        if existing.username == todo.username {
            repo.update(&todo).await.unwrap();
            return StatusCode::OK;
        }
    }
    return StatusCode::NOT_FOUND;
}
