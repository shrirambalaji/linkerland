use crate::app::DisplayUnits;

// Format a size in bytes according to the selected display units.
// Human format uses binary (IEC) units with one decimal place: B, KiB, MiB, GiB, TiB.
// Hex format uses 0xHEX (uppercase) with no leading zeros.
pub fn format_size(size: u64, units: DisplayUnits) -> String {
    match units {
        DisplayUnits::Hex => format!("0x{:X}", size),
        DisplayUnits::Human => humanize(size),
    }
}

fn humanize(size: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;
    let s = size as f64;
    if s < KIB {
        format!("{}B", size)
    } else if s < MIB {
        format!("{:.1}KiB", s / KIB)
    } else if s < GIB {
        format!("{:.1}MiB", s / MIB)
    } else if s < TIB {
        format!("{:.1}GiB", s / GIB)
    } else {
        format!("{:.1}TiB", s / TIB)
    }
}
