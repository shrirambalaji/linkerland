use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let vert: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - pct_y) / 2),
            Constraint::Percentage(pct_y),
            Constraint::Percentage((100 - pct_y) / 2),
        ])
        .split(r);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - pct_x) / 2),
            Constraint::Percentage(pct_x),
            Constraint::Percentage((100 - pct_x) / 2),
        ])
        .split(vert[1]);
    horiz[1]
}

pub fn inner(r: Rect) -> Rect {
    Rect {
        x: r.x + 1,
        y: r.y + 1,
        width: r.width.saturating_sub(2),
        height: r.height.saturating_sub(2),
    }
}

pub fn truncate_path(path: &str, max: usize) -> String {
    if path.len() <= max {
        return path.to_string();
    }

    if max < 3 {
        return "...".to_string();
    }

    // Show both start and end: "start/.../end"
    let available = max.saturating_sub(3);

    let start_len = (available * 2) / 5;
    let end_len = available.saturating_sub(start_len);

    let start = &path[..start_len.min(path.len())];
    let end_start_pos = path.len().saturating_sub(end_len);
    let end = &path[end_start_pos..];

    format!("{}...{}", start, end)
}
