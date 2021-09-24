use std::io::Stdout;

use ansi_to_tui::ansi_to_text;
use chrono::{Duration, Timelike};
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tui::{Frame, backend::TermionBackend, layout::{Alignment, Rect}, style::{Color, Modifier, Style}, symbols::Marker, text::{Span, Text}, widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap}};

use crate::{data::OnScreenData, ui::utils::pre_pad_raw_text};

const TEMP_GRAPH_HOURS: usize = 15;

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
        .unwrap_or(Text::raw("An error occurred")),
    )
    .block(container)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}

/// Build the weather graph widget
pub fn render_weather_graph<'a>(
    data: &'a OnScreenData,
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    area: Rect,
) {
    // Create the container
    let container = Block::default()
        .title("Temperature")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    // Smash together the temperature data
    let temp_data = itertools::concat(
        data.weather
            .weather
            .iter()
            .map(|w| {
                w.hourly
                    .iter()
                    .map(|forecast| forecast.temperature.parse::<i8>().unwrap_or(0) as f64)
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<_>>>(),
    )
    .iter()
    .take(TEMP_GRAPH_HOURS)
    .enumerate()
    .map(|(i, x)| (i as f64, *x))
    .collect::<Vec<(f64, f64)>>();
    let min_temp = temp_data.iter().map(|(_, t)| *t as i8).min().unwrap_or(-10);
    let max_temp = temp_data.iter().map(|(_, t)| *t as i8).max().unwrap_or(10);
    let x_markers = temp_data
        .iter()
        .map(|(h, _)| {
            Span::from(
                data.timestamp
                    .map(|t| format!("{}h", (t + Duration::hours(*h as i64)).hour()))
                    .unwrap_or("?".to_string()),
            )
        })
        .collect::<Vec<Span>>();

    // Build a dataset from the temperature
    let temperature_data = vec![Dataset::default()
        .name("Temperature")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Yellow))
        .data(&temp_data)];

    let chart = Chart::new(temperature_data)
        .block(container)
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled(
                        min_temp.to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        max_temp.to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ])
                .bounds([min_temp as f64, max_temp as f64]),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, TEMP_GRAPH_HOURS as f64])
                .labels(x_markers),
        );
    f.render_widget(chart, area);
}
