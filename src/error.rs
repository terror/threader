use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(
    context(false),
    display("Could not load environment variable: {}", source)
  )]
  Env { source: dotenv::Error },

  #[snafu(
    context(false),
    display("Error interacting with egg_mode: {}", source)
  )]
  Api { source: egg_mode::error::Error },

  #[snafu(context(false), display("IO error: {}", source))]
  Io { source: std::io::Error },
}
