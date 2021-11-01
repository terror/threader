use crate::common::*;

/// Upon construction of a `Client` instance, if proper values are set in the
/// configuration file, various methods on the newly constructed instance are
/// able to make calls to Twitter's API based on the `token` value it stores.

#[derive(Debug)]
pub struct Client {
  token: egg_mode::Token,
}

impl Client {
  /// Construct a new `Client` instance based on a configuration `Config`.
  pub async fn new(config: Config) -> Self {
    Client {
      token: Token::Access {
        consumer: KeyPair::new(config.consumer_key, config.consumer_secret),
        access:   KeyPair::new(
          config.access_token_key,
          config.access_token_secret,
        ),
      },
    }
  }

  /// Crafts a Twitter thread based on the passed in list of `Tweet` instances.
  /// The function will fail if it encounters an error when interacting with
  /// Twitter's API.
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
