use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable, Insertable, AsChangeset};

#[derive(Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
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
            created_at: chrono::Utc::now().naive_utc(),
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
    pub completed_at: Option<NaiveDateTime>
}
