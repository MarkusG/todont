use std::env;
use std::sync::Arc;

use tokio::sync::Mutex;

use uuid::Uuid;

use axum::async_trait;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dotenvy::dotenv;

use crate::models::Todo;

pub type DynTodoRepository = Arc<Mutex<dyn TodoRepository + Send + Sync>>;

#[async_trait]
pub trait TodoRepository {
    async fn create(&mut self, todo: &Todo) -> Option<Todo>;
    async fn get(&self, id: Uuid) -> Option<Todo>;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&mut self, todo: &Todo) -> Option<Todo>;
    async fn delete(&mut self, id: Uuid) -> bool;
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

fn establish_connection() -> PgConnection {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

#[async_trait]
impl TodoRepository for PgTodoRepository {
    async fn create(&mut self, todo: &Todo) -> Option<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        Some(diesel::insert_into(todos)
            .values(todo)
            .returning(Todo::as_returning())
            .get_result(connection)
            .expect("Error inserting new todo"))
    }

    async fn get(&self, todo_id: Uuid) -> Option<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        let result = todos.filter(id.eq(todo_id))
            .select(Todo::as_select())
            .first(connection);

        if let Err(diesel::NotFound) = result {
            None
        } else {
            Some(result.expect("Error getting todo"))
        }
    }

    async fn get_all(&self) -> Vec<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        todos.get_results(connection)
            .expect("Error getting todos")
    }

    async fn update(&mut self, todo: &Todo) -> Option<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        let result = diesel::update(todos)
            .filter(id.eq(todo.id))
            .set(todo)
            .get_result(connection);

        if let Ok(t) = result {
            Some(t)
        } else {
            None
        }
    }

    async fn delete(&mut self, todo_id: Uuid) -> bool {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        if diesel::delete(todos.filter(id.eq(todo_id)))
            .execute(connection)
            .expect("Error deleting todo") > 0 { true } else { false }
    }
}
