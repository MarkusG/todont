use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, Extension};
use axum::routing::{post, get, delete};

use todont_api::endpoints::todos::create::*;
use todont_api::endpoints::todos::get::*;
use todont_api::endpoints::todos::get_all::*;
use todont_api::endpoints::todos::delete::*;
use todont_api::repository::{DynTodoRepository, VecTodoRepository};

#[tokio::main]
async fn main() {
    let repo = Arc::new(Mutex::new(VecTodoRepository::new())) as DynTodoRepository;
    // let repo = Arc::new(VecTodoRepository::new()) as DynTodoRepository;

    let app = Router::new()
        .route("/todos/:id", get(get_todo))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", delete(delete_todo))
        .layer(Extension(repo));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
