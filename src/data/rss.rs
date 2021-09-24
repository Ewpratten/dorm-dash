use chrono::{DateTime, FixedOffset};
use easy_rss::RssParser;
use log::info;

pub struct RssData {
    pub text: String,
    pub timestamp: DateTime<FixedOffset>,
    pub author: String
}

pub fn fetch_rss_text_feed(address: &str) -> Result<Vec<RssData>, std::io::Error> {
    info!("Fetching RSS feed from {}", address);
    let mut parser = RssParser::from_url(address, "utf8")?;
    Ok(parser
        .parse_vec()?
        .iter()
        .map(|item| RssData {
            text: item.title.clone(),
            timestamp: DateTime::parse_from_rfc2822(&item.publish.clone()).unwrap(),
            author: item.author.clone()
        })
        .collect())
}
