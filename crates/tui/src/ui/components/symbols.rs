use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane, SymbolSortKey};
use crate::style::{header_style, selection_style, symbols_block_title};
use crate::units::format_size;
use linkerland_metrics::Bucket;

pub fn render_symbols(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let arrow = if app.symbol_sort_reverse {
        " ↑"
    } else {
        " ↓"
    };

    let addr_label = if matches!(app.symbol_sort, SymbolSortKey::Address) {
        format!("Addr{}", arrow)
    } else {
        "Addr".to_string()
    };
    let size_label = if matches!(app.symbol_sort, SymbolSortKey::Size) {
        format!("Size{}", arrow)
    } else {
        "Size".to_string()
    };
    let name_label = if matches!(app.symbol_sort, SymbolSortKey::Name) {
        format!("Name{}", arrow)
    } else {
        "Name".to_string()
    };

    let addr_style = if matches!(app.symbol_sort, SymbolSortKey::Address) {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let size_style = if matches!(app.symbol_sort, SymbolSortKey::Size) {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let name_style = if matches!(app.symbol_sort, SymbolSortKey::Name) {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let header = Row::new(vec![
        Cell::from(addr_label).style(addr_style),
        Cell::from(size_label).style(size_style),
        Cell::from("Bucket"),
        Cell::from(name_label).style(name_style),
    ])
    .style(header_style());
    let body_rows = area.height.saturating_sub(3) as usize; // header + borders
    app.symbols_view_rows = body_rows;
    app.ensure_symbol_visible();
    let start = app.symbols_offset;
    let end = (start + body_rows).min(app.filtered_symbol_indices.len());
    let rows = app.filtered_symbol_indices[start..end]
        .iter()
        .enumerate()
        .map(|(i, &sym_idx)| {
            let actual_index = start + i;
            let s = &app.symbols[sym_idx];
            let style =
                if actual_index == app.selected_symbol_pos && app.focus == FocusPane::Symbols {
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
