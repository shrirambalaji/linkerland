use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use parser::MapFile;
use ratatui::{prelude::*, widgets::*};
use std::io;

pub struct TuiOptions {
    pub sort: String,
    pub filter: Option<String>,
}

pub fn run(map: &MapFile, opts: &TuiOptions) -> io::Result<()> {
    let _ = opts; // future use
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected_section: usize = 0;
    let mut section_state = ListState::default();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let chunks =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

            let mut section_items: Vec<ListItem> = map
                .sections
                .iter()
                .map(|s| {
                    let size_hex = &s.size;
                    let parsed =
                        u64::from_str_radix(size_hex.trim_start_matches("0x"), 16).unwrap_or(0);
                    let mb = (parsed as f64) / (1024.0 * 1024.0);
                    ListItem::new(format!("[{}] {:>8.2} MB  {}", s.section, mb, s.segment))
                })
                .collect();
            if section_items.is_empty() {
                section_items.push(ListItem::new("(no sections)"));
            }
            section_state.select(Some(
                selected_section.min(section_items.len().saturating_sub(1)),
            ));

            let sections_block = List::new(section_items)
                .block(
                    Block::default()
                        .title(" Sections (q to quit) ")
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(sections_block, chunks[0], &mut section_state);

            let symbol_items: Vec<ListItem> = map
                .symbols
                .iter()
                .take(50)
                .map(|sym| {
                    let size_hex = &sym.size;
                    let parsed =
                        u64::from_str_radix(size_hex.trim_start_matches("0x"), 16).unwrap_or(0);
                    let kb = (parsed as f64) / 1024.0;
                    ListItem::new(format!(
                        "{:<40} {:>7.1} KB",
                        sym.name.chars().take(38).collect::<String>(),
                        kb
                    ))
                })
                .collect();
            let symbols_block = List::new(symbol_items).block(
                Block::default()
                    .title(" Symbols (top N) ")
                    .borders(Borders::ALL),
            );
            f.render_widget(symbols_block, chunks[1]);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        if selected_section + 1 < map.sections.len() {
                            selected_section += 1;
                        }
                    }
                    KeyCode::Up => {
                        selected_section = selected_section.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
