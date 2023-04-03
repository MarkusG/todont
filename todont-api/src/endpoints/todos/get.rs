use axum::{extract::Path, Extension};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::repository::DynTodoRepository;

pub async fn get_todo(
    Path(id): Path<Uuid>,
    Extension(repo): Extension<DynTodoRepository>) -> impl IntoResponse {
    let found = repo.get(id).await;
    if let Some(todo) = found {
        return (StatusCode::OK, axum::Json(todo)).into_response();
    }
    else {
        return StatusCode::NOT_FOUND.into_response();
    }
}
