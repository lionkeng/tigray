use crate::engine::rendering_engine::AppError;

use axum::{
  handler::Handler,
  http::{header::HeaderName, header::HeaderValue, HeaderMap, StatusCode},
  response::IntoResponse,
  routing::get,
  Router,
};
use axum_debug::debug_handler;
use sqlx::PgPool;

use super::service_router::router::service_route;

pub struct ServerContext {
  pub pool: i32, // what we want it to be ---> PgPool,
}

impl ServerContext {
  pub fn new(pool: i32) -> Self {
    ServerContext { pool }
  }
  pub fn get_pool(&self) -> &i32 {
    &self.pool
  }
}

pub fn api_routes() -> Router {
  let routes = Router::new()
    .route("/me", get(handler))
    .nest("/my_service", service_route());

  let routes = routes.fallback(handler_404.into_service());
  return routes;
}

/// This is an example handler
/// use of debug_handler is very handy in getting more information from the compiler when
/// compilation goes awry
///
#[debug_handler]
async fn handler() -> Result<(StatusCode, HeaderMap, String), AppError> {
  let mut headers = HeaderMap::new();
  headers.insert(
    HeaderName::from_static("content-type"), // is case-sensitive
    HeaderValue::from_static("image/png"),
  );
  // let data = generate_test_image().await.unwrap();
  Err(AppError {
    status: StatusCode::OK,
    error_message: "Not Implemented".to_string(),
  })
}

pub async fn default_handler() -> impl IntoResponse {
  (StatusCode::OK, "Anewgo Rendering Engine")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Nothing to see here")
}
