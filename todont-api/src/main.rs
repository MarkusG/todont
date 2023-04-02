use axum::{routing::get, Router};

use todont_api::endpoints::todos::get::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:id", get(get_todo));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
