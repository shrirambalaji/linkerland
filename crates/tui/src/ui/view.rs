use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};

use super::components::{
    render_header, render_help, render_objects, render_status, render_symbols,
};
use crate::app::AppState;

pub fn render(frame: &mut Frame, app: &mut AppState) {
    let size = frame.area();
    let window = Block::default().borders(Borders::ALL);
    frame.render_widget(&window, size);
    let inner = window.inner(size);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Min(1),    // tables
            Constraint::Length(1), // status
        ])
        .split(inner);

    render_header(frame, layout[0], app);

    let body_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(layout[1]);
    render_objects(frame, body_cols[0], app);
    render_symbols(frame, body_cols[1], app);

    render_status(frame, layout[2], app);

    if app.show_help {
        render_help(frame, app);
    }
}
