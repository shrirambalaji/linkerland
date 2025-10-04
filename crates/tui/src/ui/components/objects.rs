use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane};
use crate::state::{ObjectSortKey, SortDirection};
use crate::style::{header_style, objects_block_title, selection_style};
use crate::ui::components::truncate_path;
use crate::units::format_size;

pub fn render_objects(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let arrow = match app.objects.sort_direction {
        SortDirection::Ascending => " ↑",
        SortDirection::Descending => " ↓",
    };
    let sort_key = app.objects.sort_key;

    let make_label = |base: &str, key: ObjectSortKey| {
        if sort_key == key {
            format!("{}{}", base, arrow)
        } else {
            base.to_string()
        }
    };

    let make_style = |color: Color, key: ObjectSortKey| {
        let style = Style::default().fg(color);
        if sort_key == key {
            style.add_modifier(Modifier::BOLD)
        } else {
            style
        }
    };

    let header = Row::new(vec![
        Cell::from(make_label("TEXT", ObjectSortKey::Text))
            .style(make_style(Color::LightMagenta, ObjectSortKey::Text)),
        Cell::from(make_label("DATA", ObjectSortKey::Data))
            .style(make_style(Color::LightCyan, ObjectSortKey::Data)),
        Cell::from(make_label("BSS", ObjectSortKey::Bss))
            .style(make_style(Color::LightYellow, ObjectSortKey::Bss)),
        Cell::from(make_label("TOTAL", ObjectSortKey::Total))
            .style(make_style(Color::LightBlue, ObjectSortKey::Total)),
        Cell::from(make_label("OBJECT", ObjectSortKey::Path)).style(
            if sort_key == ObjectSortKey::Path {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        ),
    ])
    .style(header_style());
    // We'll approximate visible rows as height - 3 (top border + header + bottom border)
    let body_rows = area.height.saturating_sub(3) as usize;
    app.objects.set_view_rows(body_rows);
    let start = app.objects.scroll_offset;
    let end = (start + body_rows).min(app.objects.filtered_indices.len());

    let rows = app.objects.filtered_indices[start..end]
        .iter()
        .enumerate()
        .map(|(i, &obj_idx)| {
            let actual_index = start + i;
            let o = &app.objects.objects()[obj_idx];
            let style = if actual_index == app.objects.selected_position
                && app.focus == FocusPane::Objects
            {
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
