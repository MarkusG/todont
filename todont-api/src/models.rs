use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable, Insertable, AsChangeset};
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};

#[derive(Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>
}

impl Todo {
    pub fn from_create(request: CreateTodoRequest) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now().naive_utc(),
            completed_at: None,
            title: request.title,
            content: request.content
        }
    }

    pub fn from_update(request: UpdateTodoRequest, id: Uuid) -> Todo {
        Todo {
            id,
            created_at: request.created_at.naive_utc(),
            completed_at: if let Some(ts) = request.completed_at { Some(ts.naive_utc()) } else { None },
            title: request.title,
            content: request.content
        }
    }

    pub fn into_response(self) -> TodoResponse {
        TodoResponse {
            id: self.id,
            title: self.title,
            content: self.content,
            created_at: DateTime::<Utc>::from_utc(self.created_at, Utc),
            completed_at: if let Some(ts) = self.completed_at {
                Some(DateTime::<Utc>::from_utc(ts, Utc))
            } else { None }
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub content: String
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub messages: Vec<String>
}

pub enum ApplicationError {
    TodoNotFound(Uuid),
    ValidationFailed(Vec<String>),
    Unhandled
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        use crate::models::ApplicationError::*;
        match self {
            TodoNotFound(id) =>
                (StatusCode::NOT_FOUND,
                 axum::Json(ErrorResponse { 
                     message: format!("Could not find todo with id {}", id)
                 }))
                .into_response(),
            ValidationFailed(messages) =>
                (StatusCode::BAD_REQUEST,
                 axum::Json(ValidationErrorResponse {
                     messages
                 }))
                .into_response(),
            Unhandled =>
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String
}
