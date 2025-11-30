use std::{net::SocketAddr};

use axum::{ Router, extract::{Path, Query}, response::{ Html, IntoResponse }, routing::get };
use serde::Deserialize;

#[allow(unused)]

#[derive(Debug, Deserialize)]
struct GreetingParam {
    name: Option<String>,
}



fn get_routes() -> Router {
    Router::new()
        .route("/", get(|| async { Html("Root <strong>Route!?</strong>") }))
        .route("/another", get(routes_another_handler))
        .route("/greet", get(routes_greet_handler))
        .route("/greet/:times", get(routes_greet_n_times_handler))
}


#[tokio::main]
async fn main() {
    // let routes = Router::new()
    //     .route(
    //         "/",
    //         get(|| async { Html("Root <strong>Route!?</strong>") })
    //     )
    //     .route("/another", get(routes_another_handler))
    //     .route("/greet", get(routes_greet_handler))
    //     .route("/greet/:times", get(routes_greet_n_times_handler));
    let routes = Router::new().merge(get_routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(routes.into_make_service()).await.unwrap();
}

async fn routes_another_handler() -> impl IntoResponse {
    println!("->> {:<12} Route called!", "/another");
    Html("<h1>Another 1</h1>")
}

async fn routes_greet_handler(Query(queries): Query<GreetingParam>) -> impl IntoResponse {
    println!("->> {:<12} Route called with the query: {:?}", "/greet", queries);
    let name = queries.name.as_deref().unwrap_or("Whatever!");

    Html(format!("<h2>Hello <u>{name}</u></h2>"))
}

async fn routes_greet_n_times_handler(Path(times): Path<u64>, Query(queries): Query<GreetingParam>) -> impl IntoResponse {
    println!("->> {:<12} Route called with the query: {:?}, params: {:?}", "/greet", queries, times);
    let name = queries.name.as_deref().unwrap_or("Whatever");
    let mut response = String::new();
    for _ in 0..times {
        response.push_str(format!("<h2>Hello <u>{name}</u></h2>").as_str());
    }
    Html(response)
}