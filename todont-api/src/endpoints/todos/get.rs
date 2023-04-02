use axum::Json;
use axum::extract::Path;
use uuid::Uuid;

use crate::models::Todo;

pub async fn get_todo(Path(id): Path<Uuid>) -> Json<Todo> {
    axum::Json(Todo {
        id,
        created_at: chrono::offset::Utc::now(),
        completed_at: None,
        title: "Hello world todo".to_string(),
        content: "This is a longer string representing details about the todo".to_string()
    })
}
