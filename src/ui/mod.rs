use std::io::Stdout;

use termion::{event::Key, raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::data::DataFetchService;

use self::{
    notification_feed::build_notification_feed,
    tui_event::{Event, Events},
};

mod notification_feed;
mod tui_event;

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
            })
            .unwrap();

        if let Event::Input(input) = events.next().unwrap() {
            if let Key::Char('q') = input {
                break;
            }
        }
    }
}
