use crate::api_error::ApiError;
use actix_web::{web, HttpResponse};
use serde_json::json;
// use uuid::Uuid;

async fn test() -> Result<HttpResponse, ApiError> {
  Ok(HttpResponse::Ok().json(json!({ "message": "is working asshole".to_string() })))
}

pub async fn failtest() -> Result<HttpResponse, ApiError> {
  Ok(HttpResponse::BadRequest().json(json!({ "message": "successfully failing".to_string() })))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/tests")
      .route("/test", web::get().to(test))
      .route("/failtest", web::get().to(failtest)));
}
