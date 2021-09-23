use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}};

use crate::config::Config;

use self::rss::fetch_rss_text_feed;

mod rss;

#[derive(Debug, Clone, Default)]
pub struct OnScreenData {
    pub notifications: Vec<String>,
}

pub struct DataFetchService {
    last_known_data: OnScreenData,
    config: Config,
    thread_rx: Option<Receiver<OnScreenData>>
}

impl DataFetchService {
    /// Construct a new DataFetchService
    pub fn new(config: &Config) -> Self {
        Self {
            last_known_data: OnScreenData::default(),
            config: config.clone(),
            thread_rx: None
        }
    }

    pub fn start(&mut self) -> JoinHandle<()> {

        // Make a clone of the config for use in-thread
        let thread_config = self.config.clone();

        // Build an mpsc channel for sending data to the main thread
        let (tx, rx) = mpsc::channel();
        self.thread_rx = Some(rx);

        thread::spawn(move || {

            // Build a new data object
            let mut data = OnScreenData::default();

            // Get every twitter user's feed
            for user in &thread_config.twitter_sources {
                let mut user_feed = fetch_rss_text_feed(&format!("https://nitter.net/{}/rss", user)).unwrap();
                data.notifications.append(&mut user_feed);
            }

            // Send the data to the main thread
            tx.send(data).unwrap();
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
