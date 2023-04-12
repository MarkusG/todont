use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest_service("/", ServeDir::new("build"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
