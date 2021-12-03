use crate::routers::routing_core::{default_handler, handler_404};
use axum::AddExtensionLayer;
use axum::{handler::Handler, routing::get, Router};
use routers::routing_core::{api_routes, ServerContext};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Once};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub mod engine;
pub mod routers;

pub async fn tigray_app() -> Result<Router, anyhow::Error> {
  // shared state
  /*
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let pool = PgPoolOptions::new()
    .max_connections(16)
    .connect(&db_url)
    .await?;
  */
  let pool = 1;
  let ctx = Arc::new(ServerContext::new(pool));

  let app = Router::new()
    .route("/", get(default_handler))
    .nest("/api", api_routes())
    .layer(AddExtensionLayer::new(ctx))
    .layer(CompressionLayer::new())
    .layer(TraceLayer::new_for_http());

  Ok(app.fallback(handler_404.into_service()))
}

static INIT: Once = Once::new();
pub fn setup() {
  dotenv::dotenv().ok(); // read in the .env file
  INIT.call_once(|| {
    env_logger::init();
  });
}
