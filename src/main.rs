mod config;
mod data;
mod ui;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use data::DataFetchService;
use std::{error::Error, io};
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use ui::block_render_looping;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .takes_value(true)
                .help("Path to config file")
                .required(true),
        )
        .get_matches();

    // Get data
    let config: Config = autojson::structify(matches.value_of("config").unwrap())
        .expect("Failed to load config file");

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Begin the data fetching service
    let mut data_service = DataFetchService::new(&config);
    data_service.start().await;

    // Begin the render loop
    block_render_looping(&mut terminal, &mut data_service);

    Ok(())
}
