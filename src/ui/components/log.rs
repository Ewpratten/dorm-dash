use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_logger::TuiLoggerWidget;

use crate::data::OnScreenData;

/// Build the log widget
pub fn build_log_viewer<'a>(_data: &'a OnScreenData) -> TuiLoggerWidget<'a> {
    // Create the container
    let container = Block::default()
        .title("Logs")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    TuiLoggerWidget::default()
        .block(container)
        .style(Style::default().fg(Color::DarkGray))
}
