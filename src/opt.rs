use crate::common::*;

#[derive(Debug, StructOpt)]
pub struct Opt {
  #[structopt(short, long)]
  file: PathBuf,
}

impl Opt {
  pub async fn run(&self) -> Result<()> {
    Client::new()
      .await?
      .tweet(File::new(&self.file).parse()?)
      .await
  }
}
