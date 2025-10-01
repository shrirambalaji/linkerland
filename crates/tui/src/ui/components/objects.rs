use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane};
use crate::style::{header_style, objects_block_title, selection_style};
use crate::ui::components::truncate_path;
use crate::units::format_size;

pub fn render_objects(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let header = Row::new(vec![
        Cell::from("TEXT").style(Style::default().fg(Color::LightMagenta)),
        Cell::from("DATA").style(Style::default().fg(Color::LightCyan)),
        Cell::from("BSS").style(Style::default().fg(Color::LightYellow)),
        Cell::from("TOTAL").style(Style::default().fg(Color::LightBlue)),
        Cell::from("OBJECT"),
    ])
    .style(header_style());
    // We'll approximate visible rows as height - 3 (top border + header + bottom border)
    let body_rows = area.height.saturating_sub(3) as usize;
    app.objects_view_rows = body_rows;
    app.ensure_object_visible();
    let start = app.objects_offset;
    let end = (start + body_rows).min(app.filtered_object_indices.len());

    let rows = app.filtered_object_indices[start..end]
        .iter()
        .enumerate()
        .map(|(i, &obj_idx)| {
            let actual_index = start + i;
            let o = &app.objects[obj_idx];
            let style =
                if actual_index == app.selected_object_pos && app.focus == FocusPane::Objects {
                    selection_style()
                } else {
                    Style::default()
                };
            Row::new(vec![
                Cell::from(format_size(o.text, app.display_units)),
                Cell::from(format_size(o.data, app.display_units)),
                Cell::from(format_size(o.bss, app.display_units)),
                Cell::from(format_size(o.total, app.display_units)),
                Cell::from(truncate_path(
                    &o.path,
                    area.width.saturating_sub(18) as usize,
                )),
            ])
            .style(style)
        });
    let table = Table::new(
        rows,
        &[
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(objects_block_title())
            .padding(ratatui::widgets::Padding::horizontal(1)),
    )
    .column_spacing(1);
    frame.render_widget(table, area);
}
