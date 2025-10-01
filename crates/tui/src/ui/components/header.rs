use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::AppState;

pub fn render_header(frame: &mut Frame, area: Rect, app: &AppState) {
    let text = Line::from(vec![
        Span::styled("PATH: ", Style::default().fg(Color::Gray)),
        Span::raw(&app.map_path),
        Span::raw("  "),
        Span::styled("ARCH: ", Style::default().fg(Color::Gray)),
        Span::raw(&app.arch),
        Span::raw("  OBJ-FILTER: "),
        Span::styled(&app.object_filter, Style::default().fg(Color::Cyan)),
        Span::raw("  SYM-FILTER: "),
        Span::styled(&app.symbol_filter, Style::default().fg(Color::Cyan)),
        Span::raw("  (q quit ? help)"),
    ]);
    frame.render_widget(Paragraph::new(text), area);
}
