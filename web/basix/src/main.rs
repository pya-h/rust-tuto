use std::net::SocketAddr;

use axum::{ Router, response::{ Html, IntoResponse }, routing::get };

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route(
            "/",
            get(|| async { Html("Root <strong>Route!?</strong>") })
        )
        .route("/another", get(routes_another_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(routes.into_make_service()).await.unwrap();
}

async fn routes_another_handler() -> impl IntoResponse {
    Html("<h1>Another 1</h1>")
}
