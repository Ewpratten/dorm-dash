use std::sync::mpsc::{self, Receiver};

use chrono::{DateTime, Local, Utc};
use tokio::{task::JoinHandle, time::sleep};

use crate::config::Config;

use self::{
    rss::fetch_rss_text_feed,
    weather::{
        api::{get_ansii_weather, get_programmatic_weather},
        model::WttrData,
    },
};

mod rss;
mod twitter;
mod weather;

/// All data that is sent between coroutines.
#[derive(Debug, Clone, Default)]
pub struct OnScreenData {
    pub notifications: Vec<String>,
    pub raw_weather: String,
    pub timestamp: Option<DateTime<Local>>,
    pub weather: WttrData,
}

pub struct DataFetchService {
    last_known_data: OnScreenData,
    config: Config,
    thread_rx: Option<Receiver<OnScreenData>>,
}

impl DataFetchService {
    /// Construct a new DataFetchService
    pub fn new(config: &Config) -> Self {
        Self {
            last_known_data: OnScreenData::default(),
            config: config.clone(),
            thread_rx: None,
        }
    }

    pub async fn start(&mut self) -> JoinHandle<()> {
        // Make a clone of the config for use in-thread
        let thread_config = self.config.clone();

        // Build an mpsc channel for sending data to the main thread
        let (tx, rx) = mpsc::channel();
        self.thread_rx = Some(rx);

        tokio::spawn(async move {
            loop {
                // Build a new data object
                let mut data = OnScreenData::default();

                // Set the timestamp
                data.timestamp = Some(Local::now());

                // Fetch all tweets
                let mut tweets = itertools::concat(
                    thread_config
                        .twitter_sources
                        .iter()
                        .filter_map(|source| twitter::fetch_tweets(&source).ok()),
                );
                tweets.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                data.notifications = tweets
                    .iter()
                    .filter(|item| {
                        Utc::now() - item.timestamp.with_timezone(&Utc) < chrono::Duration::hours(3)
                    })
                    .map(|t| {
                        format!(
                            "[{}] {}",
                            t.timestamp.with_timezone(&Local).format("%H:%M"),
                            t.content.clone()
                        )
                    })
                    .collect();

                // Fetch the weather
                data.raw_weather = get_ansii_weather()
                    .await
                    .unwrap_or("Could not fetch the weather!".to_string());
                data.weather = get_programmatic_weather()
                    .await
                    .unwrap_or(WttrData::default());

                // Send the data to the main thread
                tx.send(data).unwrap();

                // Wait for a few seconds
                sleep(std::time::Duration::from_secs(15)).await;
            }
        })
    }

    pub fn query_data(&mut self) -> OnScreenData {
        if let Some(ref rx) = self.thread_rx {
            if let Ok(data) = rx.try_recv() {
                self.last_known_data = data;
            }
        }
        self.last_known_data.clone()
    }
}
