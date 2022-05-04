#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde_derive;
extern crate serde_json;

mod api_error;
mod application;
mod cmd;
mod domain;
mod infrastructure;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  env_logger::init();
  //initialize the command line interface.
  cmd::execute().await?;
  Ok(())
}
