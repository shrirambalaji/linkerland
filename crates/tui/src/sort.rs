use crate::app::{AppState, ObjectSortKey, SymbolSortKey};

pub fn apply_object_sort(app: &mut AppState) {
    let key = app.object_sort;
    let reverse = app.object_sort_reverse;
    app.filtered_object_indices.sort_by(|&a, &b| {
        let oa = &app.objects[a];
        let ob = &app.objects[b];
        let ord = match key {
            ObjectSortKey::Total => oa.total.cmp(&ob.total),
            ObjectSortKey::Text => oa.text.cmp(&ob.text),
            ObjectSortKey::Data => oa.data.cmp(&ob.data),
            ObjectSortKey::Bss => oa.bss.cmp(&ob.bss),
            ObjectSortKey::Path => oa.path.cmp(&ob.path),
            ObjectSortKey::Id => oa.id.cmp(&ob.id),
        };
        if reverse { ord.reverse() } else { ord }
    });
}

pub fn apply_symbol_sort(app: &mut AppState) {
    let key = app.symbol_sort;
    let reverse = app.symbol_sort_reverse;
    app.filtered_symbol_indices.sort_by(|&a, &b| {
        let sa = &app.symbols[a];
        let sb = &app.symbols[b];
        let ord = match key {
            SymbolSortKey::Size => sa.size.cmp(&sb.size),
            SymbolSortKey::Address => sa.address.cmp(&sb.address),
            SymbolSortKey::Name => sa.name.cmp(&sb.name),
        };
        if reverse { ord.reverse() } else { ord }
    });
}
