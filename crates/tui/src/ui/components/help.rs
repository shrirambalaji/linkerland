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
        Line::from(vec![Span::styled(
            "↑↓ navigate  |  / filter  | s sort  | r reverse  | u units  | <Tab> pane  | q quit  | ? help",
            Style::default().fg(Color::Gray),
        )]),
        Line::from(vec![Span::raw("Esc or ? to close")]),
    ];
    let text = Paragraph::new(lines);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(text, inner(area));
}
