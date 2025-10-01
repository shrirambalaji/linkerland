use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::shared::{centered_rect, inner};
use crate::app::AppState;

pub fn render_help(frame: &mut Frame, _app: &AppState) {
    let block = Block::default().borders(Borders::ALL).title("Help");
    let area = centered_rect(60, 50, frame.area());
    let text = Paragraph::new(Line::from(vec![Span::raw(
        "Arrows navigate  / filter  s sort cycle  r reverse sort  Tab switch pane  q quit  ? close help",
    )]));
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(text, inner(area));
}
