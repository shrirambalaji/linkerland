use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::{AppState, FocusPane};
use crate::style::{header_style, selection_style, symbols_block_title};
use metrics::Bucket;

pub fn render_symbols(frame: &mut Frame, area: Rect, app: &AppState) {
    let header = Row::new(vec!["Addr", "Size", "Bucket", "Name"]).style(header_style());
    let rows = app
        .filtered_symbol_indices
        .iter()
        .enumerate()
        .map(|(visible_row, &sym_idx)| {
            let s = &app.symbols[sym_idx];
            let style = if visible_row == app.selected_symbol_pos && app.focus == FocusPane::Symbols
            {
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
                Cell::from(format!("0x{:X}", s.address)),
                Cell::from(format!("0x{:X}", s.size)),
                Cell::from(Line::from(bucket_span)),
                Cell::from(s.name.clone()),
            ])
            .style(style)
        });
    let table = Table::new(
        rows,
        &[
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(symbols_block_title()),
    )
    .column_spacing(1);
    frame.render_widget(table, area);
}
