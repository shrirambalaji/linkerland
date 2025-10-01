use std::io;
use std::time::Instant;

use anyhow::{Result, anyhow};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::event::handle_key;
use crate::filter::{apply_object_filter, refresh_symbols};
use crate::style::TICK_RATE;
use crate::ui::render;

use metrics::build_metrics;
use parser::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Objects,
    Symbols,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectSortKey {
    Total,
    Text,
    Data,
    Bss,
    Path,
    Id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSortKey {
    Size,
    Address,
    Name,
}

pub struct AppState {
    pub map_path: String,
    pub arch: String,
    pub objects: Vec<metrics::ObjectMetrics>,
    pub symbols: Vec<metrics::SymbolMetrics>,
    pub filtered_object_indices: Vec<usize>,
    pub filtered_symbol_indices: Vec<usize>,
    pub selected_object_pos: usize,
    pub selected_symbol_pos: usize,
    pub object_filter: String,
    pub symbol_filter: String,
    pub focus: FocusPane,
    pub object_sort: ObjectSortKey,
    pub symbol_sort: SymbolSortKey,
    pub object_sort_reverse: bool,
    pub symbol_sort_reverse: bool,
    pub show_help: bool,
    pub last_tick: Instant,
}

impl AppState {
    pub fn new(map_path: &str) -> Result<Self> {
        let map =
            parse(std::path::Path::new(map_path)).map_err(|e| anyhow!("parse error: {:?}", e))?;
        let metrics = build_metrics(&map);
        let objects = metrics.objects;
        let symbols = metrics.symbols;
        Ok(Self {
            map_path: map.target_path,
            arch: map.arch,
            filtered_object_indices: (0..objects.len()).collect(),
            filtered_symbol_indices: Vec::new(),
            selected_object_pos: 0,
            selected_symbol_pos: 0,
            object_filter: String::new(),
            symbol_filter: String::new(),
            focus: FocusPane::Objects,
            object_sort: ObjectSortKey::Total,
            symbol_sort: SymbolSortKey::Size,
            object_sort_reverse: false,
            symbol_sort_reverse: false,
            show_help: false,
            last_tick: Instant::now(),
            objects,
            symbols,
        })
    }

    pub fn current_object_index(&self) -> Option<usize> {
        self.filtered_object_indices
            .get(self.selected_object_pos)
            .copied()
    }
}

pub fn run(map_path: &str) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new(map_path)?;
    apply_object_filter(&mut app);
    refresh_symbols(&mut app);

    loop {
        terminal.draw(|f| render(f, &mut app))?;

        let elapsed = app.last_tick.elapsed();
        if event::poll(TICK_RATE.saturating_sub(elapsed)).unwrap_or(false) {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && handle_key(key.code, &mut app)? {
                    break;
                }
            }
        }
        if app.last_tick.elapsed() >= TICK_RATE {
            app.last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
