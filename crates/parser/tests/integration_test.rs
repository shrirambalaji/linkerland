use linkerland_parser::{parse, BinaryFormat};
use std::path::Path;

#[test]
fn test_parse_linker_map_detects_macho() {
    let fixture_path = Path::new("tests/fixtures/linker.map");
    let result = parse(fixture_path);
    assert!(
        result.is_ok(),
        "Failed to parse linker map: {:?}",
        result.err()
    );
    let map = result.unwrap();
    assert_eq!(map.arch, "arm64");
    assert!(map.target_path.contains("learning_linkers"));
    assert_eq!(map.binary_format, BinaryFormat::MachO);
    assert!(!map.sections.is_empty());
    assert!(map.sections.iter().any(|s| s.segment.starts_with("__")));
    assert!(!map.symbols.is_empty());
}
