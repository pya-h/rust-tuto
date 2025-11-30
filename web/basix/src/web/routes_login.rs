use serde::Deserialize;
use axum::{ Json, Router, routing::post };
use serde_json::{ Value, json };
use crate::{ AppError, AppResult };

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login_endpoint_handler))
}

async fn login_endpoint_handler(body: Json<LoginPayload>) -> AppResult<Json<Value>> {
    if body.username != "root" || body.password != "toor" {
        return Err(AppError::LoginFailure);
    }

    // TODO: Set cookie

    let response = Json(
        json!({"result": {
        "success": "true",
        "username": body.username,
    }})
    );

    Ok(response)
}
