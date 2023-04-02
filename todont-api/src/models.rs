use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub title: String,
    pub content: String
}
