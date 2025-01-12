use actix_web::{get, http::header, middleware::Logger, App, HttpServer};
#[get("/api/healthchecker")]

pub fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build simple API for Rust Actix Web";
    HttpResponse::Ok().json(json!({
        "message": MESSAGE,
    }));
}
