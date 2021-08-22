// std
pub use std::{
  env, fmt, fs,
  io::{self, prelude::*},
  path::{Path, PathBuf},
};

// dependencies
pub use dotenv::dotenv;
pub use egg_mode::{self, auth, tweet::DraftTweet, KeyPair, Response, Token};
pub use pulldown_cmark::{Event, Parser as MarkdownParser, Tag};
pub use snafu::{ResultExt, Snafu};
pub use structopt::StructOpt;
pub use tokio;

// test dependencies
#[cfg(test)]
pub use {
  rstest::*, std::fs::File as TestFile, tempfile::TempDir, textwrap::dedent,
};

// modules used in tests
#[cfg(test)]
pub use crate::test_utils::*;

// struct and enums
pub use crate::{
  client::Client,
  error::{Error, Result},
  file::File,
  opt::Opt,
  parser::Parser,
  prefix::Prefix,
  tweet::Tweet,
};
