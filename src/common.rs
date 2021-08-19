// std
pub use std::{
  env, fmt, fs,
  io::{self, prelude::*},
  path::PathBuf,
};

// dependencies
pub use dotenv::dotenv;
pub use egg_mode::{self, auth, tweet::DraftTweet, KeyPair, Response};
pub use pulldown_cmark::{Event, Parser, Tag};
pub use snafu::{ResultExt, Snafu};
pub use structopt::StructOpt;
pub use tokio;

// modules
pub(crate) use crate::error;

// struct and enums
pub use crate::{
  client::Client,
  error::{Error, Result},
  file::File,
  opt::Opt,
  thread::Thread,
  tweet::Tweet,
  util::*,
};
