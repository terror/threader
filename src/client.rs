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

    // read pin from user
    let mut verifier = String::new();
    io::stdin().read_line(&mut verifier)?;
    println!("PIN: {}", verifier);

    // get token
    let (token, user_id, screen_name) =
      auth::access_token(con_token, &request_token, verifier).await?;

    println!("Logged in as ({} - {})", user_id, screen_name);

    Ok(Client {
      token,
    })
  }

  pub async fn tweet(&self, thread: Thread) -> Result<()> {
    let tweets = thread.tweets();

    let mut prev: Option<Response<egg_mode::tweet::Tweet>> = None;

    for i in 0..thread.length() {
      let index = i as usize;
      match index {
        0 => {
          let tweet =
            DraftTweet::new(tweets[index].clone().add_title(thread.title()));
          prev = Some(tweet.send(&self.token).await?);
        }
        _ => {
          if let Some(prev_tweet) = prev {
            let tweet = DraftTweet::new(tweets[index].clone().to_string())
              .in_reply_to(prev_tweet.response.id);
            prev = Some(tweet.send(&self.token).await?);
          }
        }
      }
    }

    Ok(())
  }
}
