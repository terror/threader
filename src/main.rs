use crate::common::*;

#[macro_use]
#[cfg(test)]
pub mod test_utils;

mod client;
mod common;
mod config;
mod error;
mod file;
mod opt;
mod parser;
mod prefix;
mod tweet;

#[tokio::main]
async fn main() {
  match Opt::from_args().run().await {
    Ok(()) => println!("Thread sent!"),
    Err(e) => eprintln!("{}", e),
  }
}
