use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;
use axum::async_trait;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::models::Todo;

pub type DynTodoRepository = Arc<Mutex<dyn TodoRepository + Send + Sync>>;

#[async_trait]
pub trait TodoRepository {
    async fn create(&mut self, todo: &Todo) -> Option<Todo>;
    async fn get(&self, id: Uuid) -> Option<Todo>;
    async fn get_all(&self) -> &[Todo];
    async fn update(&mut self, todo: &Todo) -> Option<Todo>;
    async fn delete(&mut self, id: Uuid) -> bool;
}

pub struct VecTodoRepository {
    todos: Vec<Todo>
}

impl VecTodoRepository {
    pub fn new() -> VecTodoRepository {
        VecTodoRepository {
            todos: Vec::<Todo>::new()
        }
    }
}

#[async_trait]
impl TodoRepository for VecTodoRepository {
    async fn create(&mut self, todo: &Todo) -> Option<Todo> {
        let existing = self.todos.iter_mut()
            .find(|t| t.id == todo.id);
        if let None = existing {
            self.todos.push((*todo).clone());
            Some(todo.clone())
        }
        else {
            None
        }
    }

    async fn get(&self, id: Uuid) -> Option<Todo> {
        let found = self.todos.iter()
            .find(|t| t.id == id);

        if let Some(todo) = found {
            Some(todo.clone())
        }
        else {
            None
        }
    }

    async fn get_all(&self) -> &[Todo] {
        self.todos.as_slice()
    }

    async fn update(&mut self, todo: &Todo) -> Option<Todo> {
        let existing = self.todos.iter_mut()
            .find(|t| t.id == todo.id);

        if let Some(found) = existing {
            found.completed_at = todo.completed_at;
            found.title = todo.title.clone();
            found.content = todo.content.clone();
            Some(found.clone())
        }
        else {
            None
        }
    }

    async fn delete(&mut self, id: Uuid) -> bool {
        let index = self.todos.iter()
            .position(|t| t.id == id);
        if let Some(idx) = index {
            self.todos.remove(idx);
            true
        }
        else {
            false
        }
    }
}

pub struct PgTodoRepository {
    
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

impl PgTodoRepository {
    pub fn new() -> PgTodoRepository {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let mut connection = PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
        connection.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|_| panic!("Error running migrations"));
        PgTodoRepository { }
    }
}
