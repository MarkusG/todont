use std::sync::Arc;

use axum::{Router, Extension};
use axum::{routing::get};

use todont_api::endpoints::todos::get::*;
use todont_api::repository::{DynTodoRepository, VecTodoRepository};

#[tokio::main]
async fn main() {
    let repo = Arc::new(VecTodoRepository::new()) as DynTodoRepository;

    let app = Router::new()
        .route("/todos/:id", get(get_todo))
        .layer(Extension(repo));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
