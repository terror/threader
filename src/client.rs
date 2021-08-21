use crate::common::*;

#[derive(Debug)]
pub struct Client {
  token: egg_mode::Token,
}

impl Client {
  pub async fn new() -> Result<Self> {
    let con_token =
      KeyPair::new(dotenv::var("API_KEY")?, dotenv::var("API_SECRET")?);

    let request_token = auth::request_token(&con_token, "oob").await?;

    let auth_url = auth::authorize_url(&request_token);

    println!("Authenticate here: {}", auth_url);

    let mut verifier = String::new();
    io::stdin().read_line(&mut verifier)?;

    let (token, user_id, screen_name) =
      auth::access_token(con_token, &request_token, verifier).await?;

    println!("Logged in as ({} - {})", user_id, screen_name);

    Ok(Client {
      token,
    })
  }

  pub async fn tweet(&self, tweets: Vec<Tweet>) -> Result<()> {
    let mut prev: Option<Response<egg_mode::tweet::Tweet>> = None;

    for (index, tweet) in tweets.iter().enumerate() {
      match index {
        0 => {
          let draft = DraftTweet::new(tweet.content());
          prev = Some(draft.send(&self.token).await?);
        }
        _ => {
          if let Some(prev_tweet) = prev {
            let draft = DraftTweet::new(tweet.content())
              .in_reply_to(prev_tweet.response.id);
            prev = Some(draft.send(&self.token).await?);
          }
        }
      }
    }

    Ok(())
  }
}
