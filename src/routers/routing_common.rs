use serde::Deserialize;

/*
 * This is the simpler route params used in v2
 */
#[derive(Deserialize, Debug)]
pub struct RouteParams {
  pub client_name: String,
}
