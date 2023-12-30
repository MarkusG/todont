use axum::Json;
use axum::Extension;
use axum::response::IntoResponse;

use crate::models::{Todo, CreateTodoRequest};
use crate::repository::DynTodoRepository;
use crate::auth::ApplicationClaims;

pub async fn create_todo(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    claims: ApplicationClaims,
    Json(body): Json<CreateTodoRequest>) -> impl IntoResponse {
    let todo = Todo::from_create(body, claims.username);
    let mut repo = repo_mutex.lock().await;
    repo.create(&todo).await.into_response()
}
