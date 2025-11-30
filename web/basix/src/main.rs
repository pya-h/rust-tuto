use std::{ net::SocketAddr };

use axum::{ Router };
use tower_http::services::ServeDir;

mod errors;
pub use self::errors::{ AppError, AppResult };
mod web;

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
    let routes = Router::new()
        .merge(web::routes_samples::routes())
        .merge(web::routes_greet::routes())
        .merge(web::routes_login::routes())
        .fallback_service(get_static_rouites());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(routes.into_make_service()).await.unwrap();
}

fn get_static_rouites() -> Router {
    Router::new().nest_service("/", ServeDir::new("./assets"))
}
