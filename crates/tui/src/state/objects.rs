use linkerland_metrics::ObjectMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectSortKey {
    Total,
    Text,
    Data,
    Bss,
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// State for the Objects List
pub struct ObjectsState {
    // Object data
    objects: Vec<ObjectMetrics>,

    // Filter state
    pub filter_text: String,
    pub filtered_indices: Vec<usize>,

    // Sort state
    pub sort_key: ObjectSortKey,
    pub sort_direction: SortDirection,

    // Scroll state
    pub selected_position: usize,
    pub scroll_offset: usize,

    // Internal scroll tracking
    visible_row_count: usize,
}

impl ObjectsState {
    pub fn new(data: Vec<ObjectMetrics>) -> Self {
        let filtered_indices: Vec<usize> = (0..data.len()).collect();
        let mut state = Self {
            objects: data,
            filtered_indices,
            selected_position: 0,
            scroll_offset: 0,
            visible_row_count: 0,
            filter_text: String::new(),
            sort_key: ObjectSortKey::Total,
            sort_direction: SortDirection::Descending,
        };
        state.sort();
        state
    }

    pub fn objects(&self) -> &[ObjectMetrics] {
        &self.objects
    }

    pub fn set_view_rows(&mut self, rows: usize) {
        self.visible_row_count = rows;
    }

    pub fn current_index(&self) -> Option<usize> {
        self.filtered_indices.get(self.selected_position).copied()
    }

    pub fn current_object_id(&self) -> Option<i32> {
        self.current_index().map(|idx| self.objects[idx].id)
    }

    pub fn filter(&mut self) {
        self.filtered_indices = self
            .objects
            .iter()
            .enumerate()
            .filter(|(_, obj)| {
                self.filter_text.is_empty()
                    || obj
                        .path
                        .to_ascii_lowercase()
                        .contains(&self.filter_text.to_ascii_lowercase())
            })
            .map(|(idx, _)| idx)
            .collect();

        self.sort();

        if self.selected_position >= self.filtered_indices.len() {
            self.selected_position = self.filtered_indices.len().saturating_sub(1);
        }
    }

    pub fn sort(&mut self) {
        let key = self.sort_key;
        let direction = self.sort_direction;

        self.filtered_indices.sort_by(|&a, &b| {
            let oa = &self.objects[a];
            let ob = &self.objects[b];
            let ord = match key {
                ObjectSortKey::Total => oa.total.cmp(&ob.total),
                ObjectSortKey::Text => oa.text.cmp(&ob.text),
                ObjectSortKey::Data => oa.data.cmp(&ob.data),
                ObjectSortKey::Bss => oa.bss.cmp(&ob.bss),
                ObjectSortKey::Path => oa.path.cmp(&ob.path),
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
            ObjectSortKey::Total => ObjectSortKey::Text,
            ObjectSortKey::Text => ObjectSortKey::Data,
            ObjectSortKey::Data => ObjectSortKey::Bss,
            ObjectSortKey::Bss => ObjectSortKey::Path,
            ObjectSortKey::Path => ObjectSortKey::Total,
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

    pub fn navigate_up(&mut self) -> bool {
        if self.selected_position > 0 {
            self.selected_position -= 1;
            self.ensure_visible();
            true
        } else {
            false
        }
    }

    pub fn navigate_down(&mut self) -> bool {
        if self.selected_position + 1 < self.filtered_indices.len() {
            self.selected_position += 1;
            self.ensure_visible();
            true
        } else {
            false
        }
    }

    pub fn ensure_visible(&mut self) {
        if self.visible_row_count == 0 {
            return;
        }

        if self.selected_position < self.scroll_offset {
            self.scroll_offset = self.selected_position;
        } else if self.selected_position >= self.scroll_offset + self.visible_row_count {
            self.scroll_offset = self.selected_position + 1 - self.visible_row_count;
        }

        let max_offset = self
            .filtered_indices
            .len()
            .saturating_sub(self.visible_row_count);
        if self.scroll_offset > max_offset {
            self.scroll_offset = max_offset;
        }
    }
}
