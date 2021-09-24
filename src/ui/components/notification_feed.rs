use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::data::OnScreenData;

/// Build the notification feed widget
pub fn build_notification_feed<'a>(data: &'a OnScreenData) -> Paragraph<'a> {
    // Create the container
    let container = Block::default()
        .title("Notifications")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    Paragraph::new(
        data.notifications
            .iter()
            .filter(|text| !text.is_empty())
            .enumerate()
            .map(|(idx, text)| {
                Spans::from(vec![
                    Span::raw("-> "),
                    Span::styled(
                        text,
                        Style::default().fg(if idx == 0 {
                            Color::LightGreen
                        } else {
                            if idx % 2 == 0 {
                                Color::LightBlue
                            } else {
                                Color::Gray
                            }
                        }),
                    ),
                ])
            })
            .collect::<Vec<Spans>>(),
    )
    .block(container)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}
