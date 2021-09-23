use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use chrono::{DateTime, Local, Utc};

use crate::config::Config;

use self::rss::fetch_rss_text_feed;

mod rss;

#[derive(Debug, Clone, Default)]
pub struct OnScreenData {
    pub notifications: Vec<String>,
    pub raw_weather: String,
    pub timestamp: Option<DateTime<Local>>,
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

    pub fn start(&mut self) -> JoinHandle<()> {
        // Make a clone of the config for use in-thread
        let thread_config = self.config.clone();

        // Build an mpsc channel for sending data to the main thread
        let (tx, rx) = mpsc::channel();
        self.thread_rx = Some(rx);

        thread::spawn(move || {
            loop {
                // Build a new data object
                let mut data = OnScreenData::default();

                // Get every twitter user's feed
                let mut rss_data = Vec::new();
                for user in &thread_config.twitter_sources {
                    if let Ok(mut user_feed) =
                        fetch_rss_text_feed(&format!("https://nitter.net/{}/rss", user))
                    {
                        rss_data.append(&mut user_feed);
                    }
                }
                rss_data.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                data.notifications.append(
                    &mut rss_data
                        .iter()
                        .filter(|item| {
                            Utc::now() - item.timestamp.with_timezone(&Utc)
                                < chrono::Duration::hours(3)
                        })
                        .map(|x| {
                            format!(
                                "[{}] {}",
                                x.timestamp.with_timezone(&Local).format("%H:%M"),
                                x.text.clone()
                            )
                        })
                        .collect(),
                );

                // Get the weather
                data.raw_weather = reqwest::blocking::get("http://wttr.in/?QnmAFTp")
                    .unwrap()
                    .text()
                    .unwrap();

                // Set the timestamp
                data.timestamp = Some(Local::now());

                // Send the data to the main thread
                tx.send(data).unwrap();

                // Wait for a few seconds
                thread::sleep(std::time::Duration::from_secs(15));
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
