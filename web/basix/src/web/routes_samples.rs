use axum::{Router, response::{Html, IntoResponse}, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Html("Root <strong>Route!?</strong>") })
        )
        .route("/another", get(routes_another_handler))
}

async fn routes_another_handler() -> impl IntoResponse {
    println!("->> {:<12} Route called!", "/another");
    Html("<h1>Another 1</h1>")
}
