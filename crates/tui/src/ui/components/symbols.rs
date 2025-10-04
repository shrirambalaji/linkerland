use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane};
use crate::state::{SortDirection, SymbolSortKey};
use crate::style::{header_style, selection_style, symbols_block_title};
use crate::units::format_size;
use linkerland_metrics::Bucket;

pub fn render_symbols(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let arrow = match app.symbols.sort_direction {
        SortDirection::Ascending => " ↑",
        SortDirection::Descending => " ↓",
    };
    let sort_key = app.symbols.sort_key;

    let make_label = |base: &str, key: SymbolSortKey| {
        if sort_key == key {
            format!("{}{}", base, arrow)
        } else {
            base.to_string()
        }
    };

    let make_style = |key: SymbolSortKey| {
        if sort_key == key {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        }
    };

    let header = Row::new(vec![
        Cell::from(make_label("Addr", SymbolSortKey::Address))
            .style(make_style(SymbolSortKey::Address)),
        Cell::from(make_label("Size", SymbolSortKey::Size)).style(make_style(SymbolSortKey::Size)),
        Cell::from("Bucket"),
        Cell::from(make_label("Name", SymbolSortKey::Name)).style(make_style(SymbolSortKey::Name)),
    ])
    .style(header_style());
    let body_rows = area.height.saturating_sub(3) as usize; // header + borders
    app.symbols.set_view_rows(body_rows);
    let start = app.symbols.offset;
    let end = (start + body_rows).min(app.symbols.filtered_indices.len());
    let rows = app.symbols.filtered_indices[start..end]
        .iter()
        .enumerate()
        .map(|(i, &sym_idx)| {
            let actual_index = start + i;
            let s = &app.symbols.symbols()[sym_idx];
            let style =
                if actual_index == app.symbols.selected_pos && app.focus == FocusPane::Symbols {
                    selection_style()
                } else {
                    Style::default()
                };
            let bucket_span = match s.bucket {
                Bucket::Text => Span::styled("TEXT", Style::default().fg(Color::Magenta)),
                Bucket::Data => Span::styled("DATA", Style::default().fg(Color::Yellow)),
                Bucket::Bss => Span::styled("BSS", Style::default().fg(Color::Green)),
                Bucket::Other => Span::raw("OTHER"),
            };
            Row::new(vec![
                Cell::from(format!("0x{:08X}", s.address)),
                Cell::from(format_size(s.size, app.display_units)),
                Cell::from(Line::from(bucket_span)),
                Cell::from(s.name.clone()),
            ])
            .style(style)
        });
    let table = Table::new(
        rows,
        &[
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(symbols_block_title())
            .padding(ratatui::widgets::Padding::horizontal(1)),
    )
    .column_spacing(1);
    frame.render_widget(table, area);
}
