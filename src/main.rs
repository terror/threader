use crate::common::*;

#[macro_use]
pub mod test_utils;

mod client;
mod common;
mod error;
mod file;
mod opt;
mod parser;
mod prefix;
mod tweet;

#[tokio::main]
async fn main() {
  dotenv().ok();
  match Opt::from_args().run().await {
    Ok(()) => println!("Thread sent!"),
    Err(e) => eprintln!("{}", e),
  }
}
