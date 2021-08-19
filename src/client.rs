use crate::common::*;

#[derive(Debug)]
pub struct Client {
  token: egg_mode::Token,
}

impl Client {
  pub async fn new() -> Result<Self> {
    // let auth_url = auth::authorize_url(
    //   &auth::request_token(
    //     &KeyPair::new(dotenv::var("API_KEY")?, dotenv::var("API_SECRET")?),
    //     "oob",
    //   )
    //   .await?,
    // );
    Ok(Client {
      token: egg_mode::auth::bearer_token(&egg_mode::KeyPair::new(
        dotenv::var("API_KEY")?,
        dotenv::var("API_SECRET")?,
      ))
      .await?,
    })
  }

  pub async fn tweet(&self, thread: Thread) -> Result<()> {
    let tweets = thread.tweets();

    for i in 0..thread.length() {
      match i {
        0 => {
          DraftTweet::new(
            tweets[i as usize]
              .clone()
              .add_title(thread.title())
              .to_string(),
          )
          .send(&self.token)
          .await?;
        }
        _ => {
          DraftTweet::new(tweets[i as usize].clone().to_string())
            .send(&self.token)
            .await?;
        }
      }
    }

    Ok(())
  }
}
