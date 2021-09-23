use easy_rss::RssParser;

pub fn fetch_rss_text_feed(address: &str) -> Result<Vec<String>, std::io::Error> {
    let mut parser = RssParser::from_url(address, "utf8")?;
    Ok(parser.parse_vec()?.iter().map(|item| item.title.clone()).collect())
}
