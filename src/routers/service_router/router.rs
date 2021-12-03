use axum::{
  extract::{rejection::QueryRejection, Extension, Path, Query},
  http::StatusCode,
  routing::get,
  Router,
};
use axum_debug::debug_handler;
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use crate::{
  engine::rendering_engine::AppError,
  routers::{routing_common::RouteParams, routing_core::ServerContext},
};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RouteQueryParams {
  pub mode: u32,
  pub testing: Option<bool>,
}

/**
 * v2_exterior_route_handler simplifies the interface of the API.
 * We don't need community name, nor plan name. <client_name, elevation_id>
 * is sufficient to identify the exterior to render
 */
#[debug_handler]
async fn my_service_route_handler(
  Extension(ctx): Extension<Arc<ServerContext>>,
  Path(RouteParams { client_name }): Path<RouteParams>,
  query_params: Result<Query<RouteQueryParams>, QueryRejection>,
) -> Result<(StatusCode, String), AppError> {
  debug!("{:?}", client_name);
  match query_params {
    Ok(params) => {
      debug!("{:?}", params.0);
      let _pool = ctx.get_pool();
      // get a connection from the pool and do a DB access
      Ok((StatusCode::OK, "Handle API".to_string()))
    }
    Err(e) => Err(AppError {
      status: StatusCode::BAD_REQUEST,
      error_message: format!("Bad parameters: {}", e),
    }),
  }
}

pub fn service_route() -> Router {
  Router::new().route("/client/:client_name", get(my_service_route_handler))
}
