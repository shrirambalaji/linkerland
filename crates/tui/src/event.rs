use anyhow::Result;
use crossterm::event::KeyCode;

use crate::app::{AppState, FocusPane, ObjectSortKey, SymbolSortKey};
use crate::filter::{apply_object_filter, refresh_symbols};
use crate::sort::{apply_object_sort, apply_symbol_sort};

pub fn handle_key(code: KeyCode, app: &mut AppState) -> Result<bool> {
    if app.show_help {
        match code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => app.show_help = false,
            _ => {}
        }
        return Ok(false);
    }
    match code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Tab => {
            app.focus = match app.focus {
                FocusPane::Objects => FocusPane::Symbols,
                FocusPane::Symbols => FocusPane::Objects,
            };
        }
        KeyCode::Char('/') => match app.focus {
            FocusPane::Objects => app.object_filter.clear(),
            FocusPane::Symbols => app.symbol_filter.clear(),
        },
        KeyCode::Char('?') => {
            app.show_help = true;
        }
        KeyCode::Up => match app.focus {
            FocusPane::Objects => {
                app.selected_object_pos = app.selected_object_pos.saturating_sub(1)
            }
            FocusPane::Symbols => {
                app.selected_symbol_pos = app.selected_symbol_pos.saturating_sub(1)
            }
        },
        KeyCode::Down => match app.focus {
            FocusPane::Objects => {
                if app.selected_object_pos + 1 < app.filtered_object_indices.len() {
                    app.selected_object_pos += 1;
                    refresh_symbols(app);
                    app.selected_symbol_pos = 0;
                }
            }
            FocusPane::Symbols => {
                if app.selected_symbol_pos + 1 < app.filtered_symbol_indices.len() {
                    app.selected_symbol_pos += 1;
                }
            }
        },
        KeyCode::Char('s') => match app.focus {
            FocusPane::Objects => {
                app.object_sort = match app.object_sort {
                    ObjectSortKey::Total => ObjectSortKey::Text,
                    ObjectSortKey::Text => ObjectSortKey::Data,
                    ObjectSortKey::Data => ObjectSortKey::Bss,
                    ObjectSortKey::Bss => ObjectSortKey::Path,
                    ObjectSortKey::Path => ObjectSortKey::Id,
                    ObjectSortKey::Id => ObjectSortKey::Total,
                };
                apply_object_sort(app);
            }
            FocusPane::Symbols => {
                app.symbol_sort = match app.symbol_sort {
                    SymbolSortKey::Size => SymbolSortKey::Address,
                    SymbolSortKey::Address => SymbolSortKey::Name,
                    SymbolSortKey::Name => SymbolSortKey::Size,
                };
                apply_symbol_sort(app);
            }
        },
        KeyCode::Char('r') => match app.focus {
            FocusPane::Objects => {
                app.object_sort_reverse = !app.object_sort_reverse;
                apply_object_sort(app);
            }
            FocusPane::Symbols => {
                app.symbol_sort_reverse = !app.symbol_sort_reverse;
                apply_symbol_sort(app);
            }
        },
        KeyCode::Char(c) if !c.is_control() => match app.focus {
            FocusPane::Objects => {
                app.object_filter.push(c);
                apply_object_filter(app);
            }
            FocusPane::Symbols => {
                app.symbol_filter.push(c);
                refresh_symbols(app);
            }
        },
        KeyCode::Backspace => match app.focus {
            FocusPane::Objects => {
                app.object_filter.pop();
                apply_object_filter(app);
            }
            FocusPane::Symbols => {
                app.symbol_filter.pop();
                refresh_symbols(app);
            }
        },
        _ => {}
    }
    Ok(false)
}
