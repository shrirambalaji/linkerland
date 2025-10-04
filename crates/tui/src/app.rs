use std::io;
use std::time::Instant;

use anyhow::{Result, anyhow};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::event::handle_key;
use crate::state::{ObjectsState, SymbolsState};
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
pub enum DisplayUnits {
    Human,
    Hex,
}

pub struct AppState {
    pub map_path: String,
    pub arch: String,
    pub binary_format: String,
    pub objects: ObjectsState,
    pub symbols: SymbolsState,
    pub focus: FocusPane,
    pub show_help: bool,
    pub filter_mode: bool,
    pub last_tick: Instant,
    pub display_units: DisplayUnits,
}

impl AppState {
    pub fn new(map_path: &str) -> Result<Self> {
        let map =
            parse(std::path::Path::new(map_path)).map_err(|e| anyhow!("parse error: {:?}", e))?;
        let metrics = build_metrics(&map);

        let mut objects = ObjectsState::new(metrics.objects);
        let mut symbols = SymbolsState::new(metrics.symbols);

        objects.filter();
        symbols.refresh_for_object(objects.current_object_id());

        Ok(Self {
            arch: map.arch,
            binary_format: map.binary_format.as_str().to_string(),
            display_units: DisplayUnits::Human,
            filter_mode: false,
            focus: FocusPane::Objects,
            last_tick: Instant::now(),
            map_path: map.target_path,
            objects,
            show_help: false,
            symbols,
        })
    }
}

pub fn run(map_path: &str) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new(map_path)?;

    loop {
        terminal.draw(|f| render(f, &mut app))?;
        let elapsed = app.last_tick.elapsed();
        if event::poll(TICK_RATE.saturating_sub(elapsed)).unwrap_or(false)
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && handle_key(key.code, &mut app)?
        {
            break;
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
