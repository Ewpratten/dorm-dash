use tui::{layout::Alignment, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::{Block, Borders, Paragraph, Wrap}};

use crate::data::OnScreenData;



/// Build the weather widget
pub fn build_weather_info<'a>(data: &'a OnScreenData) -> Paragraph<'a> {
    // Create the container
    let container = Block::default()
        .title("Weather")
        .borders(Borders::ALL);

    Paragraph::new(
        Text::raw(data.raw_weather.clone().replace("\n", "\n>")),
    )
    .block(container)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}
