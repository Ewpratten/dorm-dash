use std::io::Stdout;

use termion::{event::Key, raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Span, Text},
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::data::DataFetchService;

use self::components::{
    clock::build_clock,
    log::build_log_viewer,
    notification_feed::build_notification_feed,
    weather::{build_weather_info, render_weather_graph},
};
use self::tui_event::{Event, Events};

mod components;
mod tui_event;
mod utils;

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
                    .constraints(
                        [
                            Constraint::Percentage(32),
                            Constraint::Percentage(30),
                            Constraint::Percentage(30),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // Render the notification feed
                let notification_feed = build_notification_feed(&data_frame);
                f.render_widget(notification_feed, chunks[2]);

                // Set up the left panel
                let left_panel = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(15),
                            Constraint::Percentage(68),
                            Constraint::Percentage(17),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[0]);

                // Render the clock widget
                let clock_widget = build_clock(&data_frame);
                f.render_widget(clock_widget, left_panel[0]);

                // Render the weather widget
                let weather_widget = build_weather_info(&data_frame);
                render_weather_graph(&data_frame, f, left_panel[2]);
                f.render_widget(weather_widget, left_panel[1]);

                // Set up center panel
                let center_panel = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);

                // Render the log viewer
                let log_viewer = build_log_viewer(&data_frame);
                f.render_widget(log_viewer, center_panel[1]);

                // Render a welcome message
                let welcome_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Ratio(1, 3),
                            Constraint::Ratio(1, 3),
                            Constraint::Ratio(1, 3),
                        ]
                        .as_ref(),
                    )
                    .split(center_panel[0]);
                let welcome_message = Paragraph::new(Text::raw(
                    "Welcome to Evan's room!\nThis display shows real-time updates for the area\n\nSee: https://github.com/ewpratten/dorm-dash",
                ))
                .alignment(Alignment::Center)
                .block(Block::default());
                f.render_widget(welcome_message, welcome_layout[1]);
            })
            .unwrap();

        if let Event::Input(input) = events.next().unwrap() {
            if let Key::Char('q') = input {
                break;
            }
        }
    }
}
