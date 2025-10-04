use anyhow::Result;
use crossterm::event::KeyCode;

use crate::app::{AppState, DisplayUnits, FocusPane};

pub fn handle_key(code: KeyCode, app: &mut AppState) -> Result<bool> {
    if app.show_help {
        match code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => app.show_help = false,
            _ => {}
        }
        return Ok(false);
    }

    // Handle filter mode
    if app.filter_mode {
        match code {
            KeyCode::Esc | KeyCode::Enter => {
                app.filter_mode = false;
            }
            KeyCode::Char(c) if !c.is_control() => match app.focus {
                FocusPane::Objects => {
                    app.objects.push_filter_char(c);
                    app.symbols
                        .refresh_for_object(app.objects.current_object_id());
                }
                FocusPane::Symbols => {
                    app.symbols.push_filter_char(c);
                }
            },
            KeyCode::Backspace => match app.focus {
                FocusPane::Objects => {
                    app.objects.pop_filter_char();
                    app.symbols
                        .refresh_for_object(app.objects.current_object_id());
                }
                FocusPane::Symbols => {
                    app.symbols.pop_filter_char();
                }
            },
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
        KeyCode::Char('/') => {
            app.filter_mode = true;
        }
        KeyCode::Char('?') => {
            app.show_help = true;
        }
        KeyCode::Up => match app.focus {
            FocusPane::Objects => {
                let prev_id = app.objects.current_object_id();
                app.objects.navigate_up();
                let new_id = app.objects.current_object_id();
                if prev_id != new_id {
                    app.symbols.refresh_for_object(new_id);
                    app.symbols.reset_selection();
                }
            }
            FocusPane::Symbols => {
                app.symbols.navigate_up();
            }
        },
        KeyCode::Down => match app.focus {
            FocusPane::Objects => {
                let prev_id = app.objects.current_object_id();
                app.objects.navigate_down();
                let new_id = app.objects.current_object_id();
                if prev_id != new_id {
                    app.symbols.refresh_for_object(new_id);
                    app.symbols.reset_selection();
                }
            }
            FocusPane::Symbols => {
                app.symbols.navigate_down();
            }
        },
        KeyCode::Char('s') => match app.focus {
            FocusPane::Objects => {
                app.objects.cycle_sort_key();
            }
            FocusPane::Symbols => {
                app.symbols.cycle_sort_key();
            }
        },
        KeyCode::Char('r') => match app.focus {
            FocusPane::Objects => {
                app.objects.toggle_sort_direction();
            }
            FocusPane::Symbols => {
                app.symbols.toggle_sort_direction();
            }
        },
        KeyCode::Char('u') => {
            app.display_units = match app.display_units {
                DisplayUnits::Human => DisplayUnits::Hex,
                DisplayUnits::Hex => DisplayUnits::Human,
            };
        }
        _ => {}
    }
    Ok(false)
}
