use chrono::{Local, Utc};
use figlet_rs::FIGfont;
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::data::OnScreenData;

/// Build the clock widget
pub fn build_clock<'a>(data: &'a OnScreenData) -> Paragraph<'a> {
    // Create the container
    let container = Block::default().title("Clock").borders(Borders::ALL);

    // Get time as ascii art
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font
        .convert(&Local::now().format("%H:%M:%S").to_string())
        .unwrap();

    // Build the final text
    let text = format!(
        "\n{}\nUpdated: {}\n",
        figure.to_string(),
        data.timestamp
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or("Never".to_string())
    )
    .replace("\n", "\n.");

    Paragraph::new(Text::raw(text))
        .block(container)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}
