use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Todo {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub title: String,
    pub content: String
}

impl Todo {
    pub fn from_create(request: CreateTodoRequest) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            created_at: chrono::offset::Utc::now(),
            completed_at: None,
            title: request.title,
            content: request.content
        }
    }

    pub fn from_update(request: UpdateTodoRequest, id: Uuid) -> Todo {
        Todo {
            id,
            created_at: chrono::offset::Utc::now(),
            completed_at: request.completed_at,
            title: request.title,
            content: request.content
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub completed_at: Option<DateTime<Utc>>
}
