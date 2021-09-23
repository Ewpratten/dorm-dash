use chrono::{Local, Utc};
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::data::OnScreenData;

/// Build the clock widget
pub fn build_clock<'a>(data: &'a OnScreenData) -> List<'a> {
    // Create the container
    let container = Block::default().title("Clock").borders(Borders::ALL);

    List::new(vec![
        ListItem::new(Spans::from(vec![
            Span::raw("It is "),
            Span::styled(
                Local::now().format("%a, %b %e %Y").to_string(),
                Style::default().fg(Color::White),
            ),
        ])),
        ListItem::new(Spans::from(vec![
            Span::raw("Local: "),
            Span::styled(
                Local::now().format("%H:%M:%S").to_string(),
                Style::default().fg(Color::White),
            ),
        ])),
        ListItem::new(Spans::from(vec![
            Span::raw("UTC: "),
            Span::styled(
                Utc::now().format("%H:%M:%S").to_string(),
                Style::default().fg(Color::White),
            ),
        ])),
    ])
    .block(container)
}
