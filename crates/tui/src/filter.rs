use crate::app::AppState;
use crate::sort::{apply_object_sort, apply_symbol_sort};

pub fn refresh_symbols(app: &mut AppState) {
    let Some(object_idx) = app.current_object_index() else {
        return;
    };
    let object_id = app.objects[object_idx].id;
    app.filtered_symbol_indices = app
        .symbols
        .iter()
        .enumerate()
        .filter(|(_, s)| s.file_index == object_id)
        .filter(|(_, s)| {
            app.symbol_filter.is_empty()
                || s.name
                    .to_ascii_lowercase()
                    .contains(&app.symbol_filter.to_ascii_lowercase())
        })
        .map(|(i, _)| i)
        .collect();
    apply_symbol_sort(app);
    if app.selected_symbol_pos >= app.filtered_symbol_indices.len() {
        app.selected_symbol_pos = app.filtered_symbol_indices.len().saturating_sub(1);
    }
}

pub fn apply_object_filter(app: &mut AppState) {
    app.filtered_object_indices = app
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            app.object_filter.is_empty()
                || o.path
                    .to_ascii_lowercase()
                    .contains(&app.object_filter.to_ascii_lowercase())
        })
        .map(|(i, _)| i)
        .collect();
    apply_object_sort(app);
    if app.selected_object_pos >= app.filtered_object_indices.len() {
        app.selected_object_pos = app.filtered_object_indices.len().saturating_sub(1);
    }
    refresh_symbols(app);
}
