use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::components::truncate_path;
use super::components::{render_help, render_objects, render_symbols};
use crate::app::{AppState, DisplayUnits};

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
            Constraint::Length(1), // status (hints + filter)
        ])
        .split(inner);

    let truncated_path =
        truncate_path(&app.map_path, (layout[0].width.saturating_sub(30)) as usize);
    let header_line = Line::from(vec![
        Span::styled(
            " ● linkerland ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        // Span::styled("visualizer ", Style::default().fg(Color::White)),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled(" PATH: ", Style::default().fg(Color::Gray).bold()),
        Span::styled(&truncated_path, Style::default().fg(Color::LightBlue)),
        Span::styled("  | ", Style::default().fg(Color::Gray)),
        Span::styled(" ARCH: ", Style::default().fg(Color::Gray).bold()),
        Span::styled(&app.arch, Style::default().fg(Color::LightMagenta)),
    ]);
    frame.render_widget(Paragraph::new(header_line), layout[0]);

    let body_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(layout[1]);
    render_objects(frame, body_cols[0], app);
    render_symbols(frame, body_cols[1], app);
    let (prompt_label, current_filter) = if app.focus == crate::app::FocusPane::Objects {
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
    frame.render_widget(Paragraph::new(status), layout[2]);
    if app.show_help {
        render_help(frame, app);
    }
}
