use linkerland_metrics::SymbolMetrics;

use super::SortDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSortKey {
    Size,
    Address,
    Name,
}

/// State for the Symbols List
pub struct SymbolsState {
    // Symbol data
    symbols: Vec<SymbolMetrics>,
    current_object_id: Option<i32>,

    // Filter state
    pub filter_text: String,
    pub filtered_indices: Vec<usize>,

    // Sort state
    pub sort_key: SymbolSortKey,
    pub sort_direction: SortDirection,

    // Scroll state
    pub selected_pos: usize,
    pub offset: usize,

    // Internal scroll tracking
    view_rows: usize,
}

impl SymbolsState {
    pub fn new(symbols: Vec<SymbolMetrics>) -> Self {
        Self {
            symbols,
            filtered_indices: Vec::new(),
            selected_pos: 0,
            offset: 0,
            view_rows: 0,
            filter_text: String::new(),
            sort_key: SymbolSortKey::Size,
            sort_direction: SortDirection::Descending,
            current_object_id: None,
        }
    }

    pub fn symbols(&self) -> &[SymbolMetrics] {
        &self.symbols
    }

    pub fn set_view_rows(&mut self, rows: usize) {
        self.view_rows = rows;
    }

    pub fn refresh_for_object(&mut self, object_id: Option<i32>) {
        self.current_object_id = object_id;

        let Some(obj_id) = object_id else {
            self.filtered_indices.clear();
            self.selected_pos = 0;
            return;
        };

        self.filtered_indices = self
            .symbols
            .iter()
            .enumerate()
            .filter(|(_, sym)| sym.file_index == obj_id)
            .filter(|(_, sym)| {
                self.filter_text.is_empty()
                    || sym
                        .name
                        .to_ascii_lowercase()
                        .contains(&self.filter_text.to_ascii_lowercase())
            })
            .map(|(idx, _)| idx)
            .collect();

        self.sort();

        if self.selected_pos >= self.filtered_indices.len() {
            self.selected_pos = self.filtered_indices.len().saturating_sub(1);
        }
    }

    pub fn filter(&mut self) {
        self.refresh_for_object(self.current_object_id);
    }

    pub fn sort(&mut self) {
        let key = self.sort_key;
        let direction = self.sort_direction;

        self.filtered_indices.sort_by(|&a, &b| {
            let sa = &self.symbols[a];
            let sb = &self.symbols[b];
            let ord = match key {
                SymbolSortKey::Size => sa.size.cmp(&sb.size),
                SymbolSortKey::Address => sa.address.cmp(&sb.address),
                SymbolSortKey::Name => sa.name.cmp(&sb.name),
            };
            match direction {
                SortDirection::Ascending => ord,
                SortDirection::Descending => ord.reverse(),
            }
        });
    }

    pub fn push_filter_char(&mut self, c: char) {
        self.filter_text.push(c);
        self.filter();
    }

    pub fn pop_filter_char(&mut self) {
        self.filter_text.pop();
        self.filter();
    }

    pub fn cycle_sort_key(&mut self) {
        self.sort_key = match self.sort_key {
            SymbolSortKey::Size => SymbolSortKey::Address,
            SymbolSortKey::Address => SymbolSortKey::Name,
            SymbolSortKey::Name => SymbolSortKey::Size,
        };
        self.sort();
    }

    pub fn toggle_sort_direction(&mut self) {
        self.sort_direction = match self.sort_direction {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        };
        self.sort();
    }

    pub fn navigate_up(&mut self) {
        if self.selected_pos > 0 {
            self.selected_pos -= 1;
            self.ensure_visible();
        }
    }

    pub fn navigate_down(&mut self) {
        if self.selected_pos + 1 < self.filtered_indices.len() {
            self.selected_pos += 1;
            self.ensure_visible();
        }
    }

    pub fn ensure_visible(&mut self) {
        if self.view_rows == 0 {
            return;
        }

        if self.selected_pos < self.offset {
            self.offset = self.selected_pos;
        } else if self.selected_pos >= self.offset + self.view_rows {
            self.offset = self.selected_pos + 1 - self.view_rows;
        }

        let max_offset = self.filtered_indices.len().saturating_sub(self.view_rows);
        if self.offset > max_offset {
            self.offset = max_offset;
        }
    }

    pub fn reset_selection(&mut self) {
        self.selected_pos = 0;
        self.ensure_visible();
    }
}
