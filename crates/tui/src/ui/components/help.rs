use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::shared::{centered_rect, inner};
use crate::app::AppState;

pub fn render_help(frame: &mut Frame, _app: &AppState) {
    let block = Block::default().borders(Borders::ALL).title("Help");
    let area = centered_rect(60, 50, frame.area());
    let lines = vec![
        Line::from(vec![Span::styled(
            " ● linkerland ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation:",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  ↑↓        Navigate up/down")]),
        Line::from(vec![Span::raw(
            "  Tab       Switch between Objects and Symbols panes",
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Filtering:",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  /         Enter filter mode")]),
        Line::from(vec![Span::raw("  [type]    Type to filter current pane")]),
        Line::from(vec![Span::raw("  Backspace Delete last character")]),
        Line::from(vec![Span::raw("  Esc       Exit filter mode")]),
        Line::from(vec![Span::raw("  Enter     Exit filter mode")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Sorting & Display:",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  s         Cycle sort key")]),
        Line::from(vec![Span::raw("  r         Reverse sort order")]),
        Line::from(vec![Span::raw("  u         Toggle units (human ↔ hex)")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General:",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  ?         Show/hide this help")]),
        Line::from(vec![Span::raw("  q         Quit")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press Esc or ? to close",
            Style::default().fg(Color::Gray),
        )]),
    ];
    let text = Paragraph::new(lines);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(text, inner(area));
}
