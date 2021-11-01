use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(
    context(false),
    display(
      "Error interacting with Twitter's API through egg_mode: {}",
      source
    )
  )]
  Client { source: egg_mode::error::Error },

  #[snafu(
    context(false),
    display("Unable to fetch base directories: {}", source)
  )]
  BaseDirectories { source: xdg::BaseDirectoriesError },

  #[snafu(display(
    "Tweet with content {} is over Twitter's character limit by {}",
    content,
    over_by
  ))]
  CharacterLimit { content: String, over_by: usize },

  #[snafu(display(
    "Failed to construct a file instance given `path` {}. \
    Please make sure the path given is a path to a markdown file.",
    path.display()
  ))]
  ConstructFile { path: PathBuf },

  #[snafu(display(
    "Configuration file {} not found in any XDG base directory.",
    filename
  ))]
  ConfigFileNotFound { filename: String },

  #[snafu(display(
    "Failed to Deserialize TOML configuration located at `{}`. \
    Please check your toml syntax and ensure that all configuration options are set.",
    path.display()
  ))]
  DeserializeConfig {
    source: toml::de::Error,
    path:   PathBuf,
  },

  #[snafu(context(false), display("IO error: {}", source))]
  Io { source: std::io::Error },

  #[snafu(display("Unable to load configuration from {}: {}", path.display(), source))]
  LoadConfig { source: io::Error, path: PathBuf },
}
