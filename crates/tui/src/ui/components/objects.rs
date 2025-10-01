use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane};
use crate::style::{header_style, objects_block_title, selection_style};
use crate::ui::components::truncate_path;

pub fn render_objects(frame: &mut Frame, area: Rect, app: &AppState) {
    let header =
        Row::new(vec!["Id", "TEXT", "DATA", "BSS", "TOTAL", "Object"]).style(header_style());

    let rows = app
        .filtered_object_indices
        .iter()
        .enumerate()
        .map(|(visible_row, &obj_idx)| {
            let o = &app.objects[obj_idx];
            let style = if visible_row == app.selected_object_pos && app.focus == FocusPane::Objects
            {
                selection_style()
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(o.id.to_string()),
                Cell::from(format!("0x{:X}", o.text)),
                Cell::from(format!("0x{:X}", o.data)),
                Cell::from(format!("0x{:X}", o.bss)),
                Cell::from(format!("0x{:X}", o.total)),
                Cell::from(truncate_path(
                    &o.path,
                    area.width.saturating_sub(25) as usize,
                )),
            ])
            .style(style)
        });
    let table = Table::new(
        rows,
        &[
            Constraint::Length(4),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(objects_block_title()),
    )
    .column_spacing(1);
    frame.render_widget(table, area);
}
