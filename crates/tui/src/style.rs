use ratatui::style::{Color, Modifier, Style};
use std::time::Duration;

pub const TICK_MS: u64 = 250;
pub const TICK_RATE: Duration = Duration::from_millis(TICK_MS);

pub fn header_style() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}
pub fn selection_style() -> Style {
    Style::default().bg(Color::Blue).fg(Color::Black)
}
pub fn objects_block_title() -> &'static str {
    "OBJECTS"
}
pub fn symbols_block_title() -> &'static str {
    "SYMBOLS"
}
