use actix_web::{get, HttpResponse};

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
