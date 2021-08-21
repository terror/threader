use crate::common::*;

#[derive(Debug)]
pub struct Client {
  token: egg_mode::Token,
}

impl Client {
  pub async fn new() -> Result<Self> {
    Ok(Client {
      token: Token::Access {
        consumer: KeyPair::new(
          dotenv::var("CONSUMER_KEY")?,
          dotenv::var("CONSUMER_SECRET")?,
        ),
        access:   KeyPair::new(
          dotenv::var("ACCESS_TOKEN_KEY")?,
          dotenv::var("ACCESS_TOKEN_SECRET")?,
        ),
      },
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
