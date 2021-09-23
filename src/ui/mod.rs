use std::io::Stdout;

use termion::{event::Key, raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::data::DataFetchService;

use self::{clock::build_clock, notification_feed::build_notification_feed, tui_event::{Event, Events}, weather::build_weather_info};

mod notification_feed;
mod tui_event;
mod weather;
mod clock;

/// Begins the render loop and blocks on it
pub fn block_render_looping(
    terminal: &mut Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    data_service: &mut DataFetchService,
) {
    let events = Events::new();
    loop {
        // Fetch a screen data frame
        let data_frame = data_service.query_data();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());

                // Render the notification feed
                let notification_feed = build_notification_feed(&data_frame);
                f.render_widget(notification_feed, chunks[1]);

                // Set up the left panel
                let left_panel = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                    .split(chunks[0]);
                
                // Render the weather widget
                let weather_widget = build_weather_info(&data_frame);
                f.render_widget(weather_widget, left_panel[1]);

                // Render the clock widget
                let clock_widget = build_clock(&data_frame);
                f.render_widget(clock_widget, left_panel[0]);
                
            })
            .unwrap();

        if let Event::Input(input) = events.next().unwrap() {
            if let Key::Char('q') = input {
                break;
            }
        }
    }
}
