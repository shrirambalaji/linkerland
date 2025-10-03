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

use linkerland_metrics::build_metrics;
use linkerland_parser::parse;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSortKey {
    Size,
    Address,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayUnits {
    Human,
    Hex,
}

pub struct AppState {
    pub map_path: String,
    pub arch: String,
    pub binary_format: String,
    pub objects: Vec<linkerland_metrics::ObjectMetrics>,
    pub symbols: Vec<linkerland_metrics::SymbolMetrics>,
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
    pub filter_mode: bool,
    pub last_tick: Instant,
    // Scrolling state
    pub objects_offset: usize,
    pub symbols_offset: usize,
    // How many body rows (excluding header) are currently visible; set during render
    pub objects_view_rows: usize,
    pub symbols_view_rows: usize,
    pub display_units: DisplayUnits,
}

impl AppState {
    pub fn new(map_path: &str) -> Result<Self> {
        let map =
            parse(std::path::Path::new(map_path)).map_err(|e| anyhow!("parse error: {:?}", e))?;
        let metrics = build_metrics(&map);
        let objects = metrics.objects;
        let symbols = metrics.symbols;
        Ok(Self {
            arch: map.arch,
            binary_format: map.binary_format.as_str().to_string(),
            display_units: DisplayUnits::Human,
            filter_mode: false,
            filtered_object_indices: (0..objects.len()).collect(),
            filtered_symbol_indices: Vec::new(),
            focus: FocusPane::Objects,
            last_tick: Instant::now(),
            map_path: map.target_path,
            object_filter: String::new(),
            object_sort_reverse: false,
            object_sort: ObjectSortKey::Total,
            objects_offset: 0,
            objects_view_rows: 0,
            objects,
            selected_object_pos: 0,
            selected_symbol_pos: 0,
            show_help: false,
            symbol_filter: String::new(),
            symbol_sort_reverse: false,
            symbol_sort: SymbolSortKey::Size,
            symbols_offset: 0,
            symbols_view_rows: 0,
            symbols,
        })
    }

    pub fn current_object_index(&self) -> Option<usize> {
        self.filtered_object_indices
            .get(self.selected_object_pos)
            .copied()
    }

    pub fn ensure_object_visible(&mut self) {
        let rows = self.objects_view_rows;
        if rows == 0 {
            return;
        }
        if self.selected_object_pos < self.objects_offset {
            self.objects_offset = self.selected_object_pos;
        } else if self.selected_object_pos >= self.objects_offset + rows {
            self.objects_offset = self.selected_object_pos + 1 - rows;
        }
        let max_off = self.filtered_object_indices.len().saturating_sub(rows);
        if self.objects_offset > max_off {
            self.objects_offset = max_off;
        }
    }

    pub fn ensure_symbol_visible(&mut self) {
        let rows = self.symbols_view_rows;
        if rows == 0 {
            return;
        }
        if self.selected_symbol_pos < self.symbols_offset {
            self.symbols_offset = self.selected_symbol_pos;
        } else if self.selected_symbol_pos >= self.symbols_offset + rows {
            self.symbols_offset = self.selected_symbol_pos + 1 - rows;
        }
        let max_off = self.filtered_symbol_indices.len().saturating_sub(rows);
        if self.symbols_offset > max_off {
            self.symbols_offset = max_off;
        }
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
