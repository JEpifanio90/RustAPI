use axum::{
    routing::get,
    Router
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello World"
}
