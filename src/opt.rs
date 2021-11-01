use crate::common::*;

#[derive(Debug, StructOpt)]
pub struct Opt {
  #[structopt(short, long)]
  /// Path to the twitter thread in markdown.
  file: PathBuf,
}

impl Opt {
  pub async fn run(&self) -> Result<()> {
    Client::new(Config::load()?)
      .await
      .tweet(File::new(&self.file)?.parse()?)
      .await
  }
}
