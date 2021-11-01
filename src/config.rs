use crate::common::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
  pub access_token_key:    String,
  pub access_token_secret: String,
  pub consumer_key:        String,
  pub consumer_secret:     String,
}

impl Config {
  fn filename() -> &'static str {
    CONFIG_FILE_NAME
  }

  fn path() -> Result<Option<PathBuf>> {
    Ok(
      xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap())?
        .find_config_file(Self::filename()),
    )
  }

  pub fn load() -> Result<Self> {
    if let Some(path) = &Self::path()? {
      let contents = fs::read_to_string(path).context(error::LoadConfig {
        path,
      })?;
      return toml::from_str(&contents).context(error::DeserializeConfig {
        path,
      });
    }
    Err(Error::ConfigFileNotFound {
      filename: CONFIG_FILE_NAME.to_string(),
    })
  }
}
