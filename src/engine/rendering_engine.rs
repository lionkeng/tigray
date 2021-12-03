use axum::{
  body::{Bytes, Full},
  http::{Response, StatusCode},
  response::IntoResponse,
  Json,
};
use serde_json::json;

use std::convert::Infallible;

pub struct AppError {
  pub status: StatusCode,
  pub error_message: String,
}

impl IntoResponse for AppError {
  type Body = Full<Bytes>;
  type BodyError = Infallible;

  fn into_response(self) -> Response<Self::Body> {
    let body = Json(json!({
        "error": self.error_message,
    }));
    (self.status, body).into_response()
  }
}
