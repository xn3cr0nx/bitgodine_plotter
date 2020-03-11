use crate::api_error::ApiError;
// use crate::
use actix_web::{get, web, HttpResponse};
use serde_json::json;
// use uuid::Uuid;

#[get("/test")]
async fn test() -> Result<HttpResponse, ApiError> {
  Ok(HttpResponse::Ok().json(json!({ "message": "is working asshole".to_string() })))
}

// #[get("/failtest")]
pub async fn failtest() -> Result<HttpResponse, ApiError> {
  Ok(HttpResponse::BadRequest().json(json!({ "message": "successfully failing".to_string() })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(test);
  // cfg.service(failtest);
}
