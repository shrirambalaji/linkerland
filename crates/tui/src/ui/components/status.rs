use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{AppState, DisplayUnits, FocusPane};

pub fn render_status(frame: &mut Frame, area: Rect, app: &AppState) {
    let units_label = match app.display_units {
        DisplayUnits::Human => "human",
        DisplayUnits::Hex => "hex",
    };

    let mut spans = vec![
        Span::styled(
            " ↑↓ navigate  |  / filter  | s sort | r reverse | u units | <Tab> pane | q quit | ? help  |  ",
            Style::default().fg(Color::Gray),
        ),
        Span::styled("units: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}  ", units_label),
            Style::default().fg(Color::LightGreen),
        ),
    ];

    if app.filter_mode {
        let current_filter = if app.focus == FocusPane::Objects {
            &app.object_filter
        } else {
            &app.symbol_filter
        };

        spans.push(Span::styled(
            format!("/{}", current_filter),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::styled(
            "  (esc exit filter mode)",
            Style::default().fg(Color::Gray),
        ));
    }

    let status = Line::from(spans);

    frame.render_widget(Paragraph::new(status), area);
}
