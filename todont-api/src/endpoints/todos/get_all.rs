use axum::Extension;
use axum::response::IntoResponse;

use crate::auth::ApplicationClaims;
use crate::repository::DynTodoRepository;

pub async fn get_todos(
    Extension(repo_mutex): Extension<DynTodoRepository>,
    claims: ApplicationClaims) -> impl IntoResponse {
    let repo = repo_mutex.lock().await;
    axum::Json(
        repo.get_all(claims.username).await
            .to_vec()
            .into_iter()
            .map(|t| t.into_response_struct()).collect::<Vec<_>>()
    )
}
