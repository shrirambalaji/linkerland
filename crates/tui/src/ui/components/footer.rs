use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::AppState;

pub fn render_footer(frame: &mut Frame, area: Rect, app: &AppState) {
    let total_text: u64 = app.objects.iter().map(|o| o.text).sum();
    let total_data: u64 = app.objects.iter().map(|o| o.data).sum();
    let total_bss: u64 = app.objects.iter().map(|o| o.bss).sum();
    let footer = Line::from(vec![
        Span::styled("TOTALS: ", Style::default().fg(Color::Gray)),
        Span::raw(format!(
            "TEXT 0x{:X}  DATA 0x{:X}  BSS 0x{:X}  OBJECTS {}  SYMBOLS {}",
            total_text,
            total_data,
            total_bss,
            app.filtered_object_indices.len(),
            app.filtered_symbol_indices.len()
        )),
    ]);
    frame.render_widget(Paragraph::new(footer), area);
}
