use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Simple API for Rust Actix Web is running.";
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "message": MESSAGE,
        }))
}
