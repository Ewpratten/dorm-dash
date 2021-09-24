use ansi_to_tui::ansi_to_text;
use chrono::Local;
use figlet_rs::FIGfont;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{data::OnScreenData, ui::utils::pre_pad_raw_text};

/// Build the clock widget
pub fn build_clock<'a>(_data: &'a OnScreenData) -> Paragraph<'a> {
    // Create the container
    let container = Block::default()
        .title("Clock")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    // Get time as ascii art
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font
        .convert(&Local::now().format("%H:%M:%S").to_string())
        .unwrap();

    // Build the final text
    let text = format!("\x1b[1m\x1b[37m{}", pre_pad_raw_text(&figure.to_string()));

    Paragraph::new(ansi_to_text(text.bytes()).unwrap())
        .block(container)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}
