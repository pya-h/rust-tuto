use axum::{ Router, extract::{ Path, Query }, response::{ Html, IntoResponse }, routing::get};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GreetingParam {
    name: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/greet", get(routes_greet_handler))
        .route("/greet/:times", get(routes_greet_n_times_handler))
}

async fn routes_greet_handler(Query(queries): Query<GreetingParam>) -> impl IntoResponse {
    println!("->> {:<12} Route called with the query: {:?}", "/greet", queries);
    let name = queries.name.as_deref().unwrap_or("Whatever!");

    Html(format!("<h2>Hello <u>{name}</u></h2>"))
}

async fn routes_greet_n_times_handler(
    Path(times): Path<u64>,
    Query(queries): Query<GreetingParam>
) -> impl IntoResponse {
    println!(
        "->> {:<12} Route called with the query: {:?}, params: {:?}",
        "/greet",
        queries,
        times
    );
    let name = queries.name.as_deref().unwrap_or("Whatever");
    let mut response = String::new();
    for _ in 0..times {
        response.push_str(format!("<h2>Hello <u>{name}</u></h2>").as_str());
    }
    Html(response)
}
