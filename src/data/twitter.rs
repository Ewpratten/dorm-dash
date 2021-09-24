use chrono::{DateTime, Utc};

use crate::data::rss::fetch_rss_text_feed;

/// Defines a single tweet
#[derive(Debug, Clone)]
pub struct Tweet {
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

/// Fetch all recent tweets from a user
pub fn fetch_tweets(username: &str) -> Result<Vec<Tweet>, std::io::Error> {
    // Fetch the user timeline as an RSS feed
    let rss_data = fetch_rss_text_feed(&format!("https://nitter.net/{}/rss", username))?;

    // Parse the RSS feed into a vector of tweets
    let mut tweets = Vec::new();
    for item in rss_data {
        let tweet = Tweet {
            content: item.text.to_string(),
            timestamp: item.timestamp.with_timezone(&Utc),
        };
        tweets.push(tweet);
    }
    Ok(tweets)
}
