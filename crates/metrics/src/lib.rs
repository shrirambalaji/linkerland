use std::collections::HashMap;

use linkerland_parser::{MapFile, Section};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum Bucket {
    Text,
    Data,
    Bss,
    Other,
}

#[derive(Debug, Serialize, Clone)]
pub struct ObjectMetrics {
    pub id: i32,
    pub path: String,
    pub text: u64,
    pub data: u64,
    pub bss: u64,
    pub other: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct SymbolMetrics {
    pub address: u64,
    pub size: u64,
    pub file_index: i32,
    pub name: String,
    pub bucket: Bucket,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct GlobalTotals {
    pub text: u64,
    pub data: u64,
    pub bss: u64,
    pub other: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Metrics {
    pub objects: Vec<ObjectMetrics>,
    pub symbols: Vec<SymbolMetrics>,
    pub totals: GlobalTotals,
}

#[derive(Debug, Clone)]
struct SectionRange {
    start: u64,
    end: u64,
    segment: String,
    section: String,
}

fn parse_hex_u64(value: &str) -> Option<u64> {
    let trimmed = value.strip_prefix("0x").unwrap_or(value);
    u64::from_str_radix(trimmed, 16).ok()
}

fn build_section_ranges(sections: &[Section]) -> Vec<SectionRange> {
    let mut ranges: Vec<SectionRange> = sections
        .iter()
        .filter_map(|section| {
            let start = parse_hex_u64(&section.address)?;
            let size = parse_hex_u64(&section.size)?;
            Some(SectionRange {
                start,
                end: start.saturating_add(size),
                segment: section.segment.clone(),
                section: section.section.clone(),
            })
        })
        .collect();
    ranges.sort_by_key(|range| range.start);
    ranges
}

fn find_section(ranges: &[SectionRange], address: u64) -> Option<&SectionRange> {
    let mut low = 0usize;
    let mut high = ranges.len();
    while low < high {
        let mid = (low + high) / 2;
        if ranges[mid].start <= address {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if low == 0 {
        return None;
    }
    let candidate = &ranges[low - 1];
    (address < candidate.end).then_some(candidate)
}

fn classify(segment: &str, section: &str) -> Bucket {
    let segment_upper = segment.to_ascii_uppercase();
    if segment_upper == "__TEXT" {
        return Bucket::Text;
    }

    let section_lower = section.to_ascii_lowercase();
    const DATA_SECTIONS: &[&str] = &[
        "__data",
        "__const",
        "__got",
        "__mod_init_func",
        "__cstring",
        "__const_coal",
    ];
    const BSS_SECTIONS: &[&str] = &["__bss", "__bss_coal", "__common"];
    if DATA_SECTIONS
        .iter()
        .any(|candidate| section_lower == *candidate)
    {
        return Bucket::Data;
    }
    if BSS_SECTIONS
        .iter()
        .any(|candidate| section_lower == *candidate)
    {
        return Bucket::Bss;
    }
    Bucket::Other
}

/// Build aggregated metrics (per-object, per-symbol classification, global totals).
pub fn build_metrics(map: &MapFile) -> Metrics {
    let section_ranges = build_section_ranges(&map.sections);

    let mut objects: Vec<ObjectMetrics> = map
        .object_files
        .iter()
        .map(|object_file| ObjectMetrics {
            id: object_file.index,
            path: object_file.path.clone(),
            text: 0,
            data: 0,
            bss: 0,
            other: 0,
            total: 0,
        })
        .collect();

    let object_index_lookup: HashMap<i32, usize> = objects
        .iter()
        .enumerate()
        .map(|(position, metrics)| (metrics.id, position))
        .collect();

    let mut totals = GlobalTotals::default();
    let mut symbol_metrics: Vec<SymbolMetrics> = Vec::with_capacity(map.symbols.len());

    for symbol in &map.symbols {
        let address = if let Some(parsed) = parse_hex_u64(&symbol.address) {
            parsed
        } else {
            continue;
        };
        let size = parse_hex_u64(&symbol.size).unwrap_or(0);
        let file_index: i32 = symbol.file_index.parse().unwrap_or(-1);

        let bucket = find_section(&section_ranges, address)
            .map(|range| classify(&range.segment, &range.section))
            .unwrap_or(Bucket::Other);

        if let Some(&object_pos) = object_index_lookup.get(&file_index) {
            let object_metrics = &mut objects[object_pos];
            match bucket {
                Bucket::Text => object_metrics.text += size,
                Bucket::Data => object_metrics.data += size,
                Bucket::Bss => object_metrics.bss += size,
                Bucket::Other => object_metrics.other += size,
            }
            object_metrics.total += size;
        }

        match bucket {
            Bucket::Text => totals.text += size,
            Bucket::Data => totals.data += size,
            Bucket::Bss => totals.bss += size,
            Bucket::Other => totals.other += size,
        }
        totals.total += size;

        symbol_metrics.push(SymbolMetrics {
            address,
            size,
            file_index,
            name: symbol.name.clone(),
            bucket,
        });
    }

    Metrics {
        objects,
        symbols: symbol_metrics,
        totals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linkerland_parser::{BinaryFormat, MapFile, ObjectFile, Symbol};

    fn mk_map() -> MapFile {
        MapFile {
            arch: "arm64".into(),
            target_path: "/tmp/app".into(),
            binary_format: BinaryFormat::MachO,
            object_files: vec![ObjectFile {
                index: 1,
                path: "a.o".into(),
            }],
            sections: vec![Section {
                address: "0x1000".into(),
                size: "0x50".into(),
                segment: "__TEXT".into(),
                section: "__text".into(),
            }],
            symbols: vec![Symbol {
                address: "0x1000".into(),
                size: "0x10".into(),
                file_index: "1".into(),
                name: "_foo".into(),
            }],
        }
    }

    #[test]
    fn build_basic() {
        let map = mk_map();
        let res = build_metrics(&map);
        assert_eq!(res.objects.len(), 1);
        assert_eq!(res.objects[0].text, 0x10);
        assert_eq!(res.totals.text, 0x10);
        assert_eq!(res.symbols[0].bucket, Bucket::Text);
    }
}
