use ansi_to_tui::ansi_to_text;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{data::OnScreenData, ui::utils::pre_pad_raw_text};

/// Build the weather widget
pub fn build_weather_info<'a>(data: &'a OnScreenData) -> Paragraph<'a> {
    // Create the container
    let container = Block::default()
        .title("Weather")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    Paragraph::new(
        ansi_to_text(
            pre_pad_raw_text(&data.raw_weather)
                .replace("[0m", "[39m")
                .bytes(),
        )
        .unwrap(),
    )
    .block(container)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}
