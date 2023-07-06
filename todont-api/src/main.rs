use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, Extension};
use axum::routing::{post, get, put, delete};

use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use tower::ServiceBuilder;

use todont_api::endpoints::todos::create::*;
use todont_api::endpoints::todos::get::*;
use todont_api::endpoints::todos::get_all::*;
use todont_api::endpoints::todos::update::*;
use todont_api::endpoints::todos::delete::*;
use todont_api::repository::{DynTodoRepository, PgTodoRepository};

#[tokio::main]
async fn main() {
    let repo = Arc::new(Mutex::new(PgTodoRepository::new())) as DynTodoRepository;

    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "https://localhost:3001".parse().unwrap(),
    ];


    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(origins);

    let app = Router::new()
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos", get(get_todos))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .layer(
            ServiceBuilder::new()
            .layer(cors)
        )
        .layer(Extension(repo));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
