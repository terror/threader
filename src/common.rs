// std
pub use std::{
  fmt::{self, Display, Formatter},
  fs,
  io::{self, prelude::*},
  path::{Path, PathBuf},
};

// dependencies
pub use {
  dirs,
  egg_mode::{self, auth, tweet::DraftTweet, KeyPair, Response, Token},
  pulldown_cmark::{Event, Parser as MarkdownParser, Tag},
  serde::{Deserialize, Serialize},
  snafu::{ResultExt, Snafu},
  structopt::StructOpt,
  tokio, toml, xdg,
};

// test dependencies
#[cfg(test)]
pub use {indoc::indoc, std::env, tempfile::TempDir};

// test modules
#[cfg(test)]
pub use crate::test_utils::*;

// modules
pub(crate) use crate::error;

// struct and enums
pub use crate::{
  client::Client, config::Config, error::Error, file::File, opt::Opt,
  parser::Parser, prefix::Prefix, tweet::Tweet,
};

// type aliases
pub type Result<T, E = Error> = std::result::Result<T, E>;

// contstants
pub const MARKDOWN: &str = "md";
pub const CHARACTER_LIMIT: usize = 280;
pub const CONFIG_FILE_NAME: &str = ".threader.toml";
