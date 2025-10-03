use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{AppState, DisplayUnits, FocusPane};

pub fn render_status(frame: &mut Frame, area: Rect, app: &AppState) {
    let (prompt_label, current_filter) = if app.focus == FocusPane::Objects {
        ("filter", &app.object_filter)
    } else {
        ("filter", &app.symbol_filter)
    };

    let units_label = match app.display_units {
        DisplayUnits::Human => "human",
        DisplayUnits::Hex => "hex",
    };

    let status = Line::from(vec![
        Span::styled(
            " ↑↓ navigate  |  / filter  | s sort | r reverse | u units | <Tab> pane | q quit | ? help  |  ",
            Style::default().fg(Color::Gray),
        ),
        Span::styled("units: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}  ", units_label),
            Style::default().fg(Color::LightGreen),
        ),
        Span::raw(format!("{}:", prompt_label)),
        Span::styled(
            format!(" /{}", current_filter),
            Style::default().fg(Color::Yellow),
        ),
    ]);

    frame.render_widget(Paragraph::new(status), area);
}
