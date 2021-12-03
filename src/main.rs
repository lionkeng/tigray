use std::net::{Ipv4Addr, SocketAddr};
use tigray::{setup, tigray_app};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // read in .env file
  setup();

  // Set the RUST_LOG, if it hasn't been explicitly defined
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "tigray=debug,tower_http=debug")
  }
  if std::env::var_os("TIGRAY_PORT").is_none() {
    std::env::set_var("TIGRAY_PORT", "5051");
  }
  if std::env::var_os("TIGRAY_HOST").is_none() {
    std::env::set_var("TIGRAY_HOST", "127.0.0.1");
  }

  tracing_subscriber::fmt::init();

  let port_str = std::env::var("TIGRAY_PORT").expect("tigray_PORT is not set");
  let port_num = port_str.parse().expect("TIGRAY_PORT is not a valid number");
  let host_str = std::env::var("TIGRAY_HOST").expect("tigray_HOST is not configured");
  let host_addr: Ipv4Addr = host_str
    .parse()
    .expect("required a dot-notated IPv4 address");
  let addr = SocketAddr::from((host_addr, port_num));

  let app = tigray_app().await?;

  // run it
  debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
}
