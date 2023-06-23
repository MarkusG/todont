use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;
use axum::async_trait;

use crate::models::Todo;

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

pub type DynTodoRepository = Arc<Mutex<dyn TodoRepository + Send + Sync>>;

#[async_trait]
pub trait TodoRepository {
    async fn create(&mut self, todo: &Todo) -> Option<Todo>;
    async fn get(&self, id: Uuid) -> Option<Todo>;
    async fn get_all(&self) -> &[Todo];
    async fn update(&mut self, todo: &Todo) -> Option<Todo>;
    async fn delete(&mut self, id: Uuid) -> bool;
}
