use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::truncate_path;
use crate::app::AppState;

pub fn render_header(frame: &mut Frame, area: Rect, app: &AppState) {
    let truncated_path = truncate_path(&app.map_path, (area.width.saturating_sub(50)) as usize);

    let header_line = Line::from(vec![
        Span::styled(
            " ● linkerland ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled(
            " PATH: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&truncated_path, Style::default().fg(Color::LightBlue)),
        Span::styled("  | ", Style::default().fg(Color::Gray)),
        Span::styled(
            " ARCH: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&app.arch, Style::default().fg(Color::LightMagenta)),
        Span::styled("  | ", Style::default().fg(Color::Gray)),
        Span::styled(
            " FORMAT: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&app.binary_format, Style::default().fg(Color::LightCyan)),
    ]);

    frame.render_widget(Paragraph::new(header_line), area);
}
