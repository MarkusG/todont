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
    pub completed_at: Option<NaiveDateTime>,
    pub username: String
}

#[derive(Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_sha512: Vec<u8>
}

#[derive(Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub name: String
}

#[derive(Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Permission {
    pub name: String,
    pub flag: i64
}

impl Todo {
    pub fn from_create(request: CreateTodoRequest, username: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now().naive_utc(),
            completed_at: None,
            title: request.title,
            content: request.content,
            username
        }
    }

    pub fn from_update(request: UpdateTodoRequest, id: Uuid, username: String) -> Todo {
        Todo {
            id,
            created_at: request.created_at.naive_utc(),
            completed_at: if let Some(ts) = request.completed_at { Some(ts.naive_utc()) } else { None },
            title: request.title,
            content: request.content,
            username
        }
    }

    pub fn into_response_struct(self) -> TodoResponse {
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

impl IntoResponse for Todo {
    fn into_response(self) -> Response {
        (StatusCode::OK,
         axum::Json(self.into_response_struct())).into_response()
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

#[derive(Debug)]
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

pub mod permissions {
    pub type Permissions = i64;

    pub const CREATE_TODO: i64 = 0x01;
    pub const READ_TODOS: i64 = 0x02;
    pub const UPDATE_TODO: i64 = 0x04;
    pub const DELETE_TODO: i64 = 0x08;
}
