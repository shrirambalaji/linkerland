use std::path::Path;
use std::str;

use serde::Serialize;

use winnow::{
    ascii::{digit1, line_ending, multispace0, till_line_ending},
    combinator::{alt, delimited, eof, opt, peek, preceded, repeat, repeat_till, seq, terminated},
    error::{StrContext, StrContextValue},
    token::literal,
    Parser, Result as ParserResult,
};

#[derive(Debug, Serialize)]
pub struct Symbol {
    pub address: String,
    pub size: String,
    pub file_index: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Section {
    pub address: String,
    pub size: String,
    pub segment: String,
    pub section: String,
}

#[derive(Debug)]
enum MapFileHeaders {
    Path,
    Architecture,
    ObjectFiles,
    Symbols,
    Sections,
}

#[cfg(target_os = "macos")]
impl MapFileHeaders {
    fn as_str(&self) -> &str {
        match self {
            MapFileHeaders::Path => "# Path:",
            MapFileHeaders::Architecture => "# Arch:",
            MapFileHeaders::ObjectFiles => "# Object files:",
            MapFileHeaders::Sections => "# Sections:",
            MapFileHeaders::Symbols => "# Symbols:",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ObjectFile {
    pub index: i32,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct MapFile {
    pub arch: String,
    pub object_files: Vec<ObjectFile>,
    pub target_path: String,
    pub symbols: Vec<Symbol>,
    pub sections: Vec<Section>,
}

fn read_file(map_file: &Path) -> String {
    match std::fs::read_to_string(map_file) {
        Ok(contents) => contents,
        Err(e) => {
            panic!("Error reading file: {:?}", e);
        }
    }
}

/// Consume zero or more whitespace (spaces/tabs/newlines); helper (e.g. before `__TEXT`).
fn spaces<'i>(input: &mut &'i str) -> ParserResult<&'i str> {
    multispace0(input)
}

/// Parse architecture header (e.g. `# Arch: arm64` -> `arm64`).
fn arch<'i>(input: &mut &'i str) -> ParserResult<&'i str> {
    preceded(
        preceded(
            opt(line_ending),
            preceded(literal(MapFileHeaders::Architecture.as_str()), multispace0),
        ),
        till_line_ending,
    )
    .parse_next(input)
}

/// Parse hex value starting `0x` (e.g. `0x1000007DC`).
fn hex_value<'i>(input: &mut &'i str) -> ParserResult<&'i str> {
    preceded("0x", winnow::ascii::hex_digit1)
        .take()
        .parse_next(input)
}

/// Parse one object file line (e.g. `[ 66] /path/libunwind.tbd`).
fn object_file(input: &mut &str) -> ParserResult<ObjectFile> {
    (
        delimited('[', preceded(multispace0, digit1), ']'),
        // Do NOT consume an optional preceding line_ending here; each object file line must start with '['
        preceded(multispace0, till_line_ending),
    )
        .map(|(index, path): (&str, &str)| ObjectFile {
            index: index.parse().unwrap(),
            path: path.to_string(),
        })
        .parse_next(input)
}

/// Parse entire object files block after `# Object files:` until next header.
fn object_files(input: &mut &str) -> ParserResult<Vec<ObjectFile>> {
    delimited(
        opt(line_ending),
        literal(MapFileHeaders::ObjectFiles.as_str()),
        line_ending,
    )
    .parse_next(input)?;

    repeat(
        0..,
        terminated(preceded(peek('['), object_file), opt(line_ending)),
    )
    .parse_next(input)
}

/// Parse one symbol row (e.g. `0xADDR\t0xSIZE\t[  1] name`).
fn symbol(input: &mut &str) -> ParserResult<Symbol> {
    let address = preceded(multispace0, hex_value).parse_next(input);
    let size = preceded(multispace0, hex_value).parse_next(input);
    let file_index = preceded(
        preceded(opt(line_ending), spaces),
        delimited('[', preceded(multispace0, digit1), ']'),
    )
    .parse_next(input);
    let name = preceded(spaces, till_line_ending).parse_next(input);
    let symbol_addr = address.unwrap().to_string();
    let symbol_size = size.unwrap().to_string();

    let symbol = Symbol {
        address: symbol_addr,
        size: symbol_size,
        file_index: file_index.unwrap().to_string(),
        name: name.unwrap().to_string(),
    };

    Ok(symbol)
}

/// Parse consecutive symbol rows until header or EOF.
fn symbols(input: &mut &str) -> ParserResult<Vec<Symbol>> {
    repeat_till(
        0..,
        terminated(symbol, opt(line_ending)),
        alt((literal("# "), eof)),
    )
    .parse_next(input)
    .map(|(symbols, _)| symbols)
}

/// Parse full symbol table after `# Symbols:`.
fn symbol_table(input: &mut &str) -> ParserResult<Vec<Symbol>> {
    terminated(literal(MapFileHeaders::Symbols.as_str()), line_ending).parse_next(input)?;
    terminated(till_line_ending, line_ending).parse_next(input)?;
    symbols.parse_next(input)
}

/// Parse `__` identifier (segment/section) like `__TEXT` / `__text`.
fn parse_segment_or_section<'a>(i: &mut &'a str) -> ParserResult<&'a str> {
    let start = *i;
    let _ = preceded(
        "__",
        winnow::token::take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '_'),
    )
    .parse_next(i)?;
    let consumed_len = start.len() - i.len();
    Ok(&start[..consumed_len])
}

/// Parse one section row (e.g. `0xADDR\t0xSIZE\t__SEG\t__sect`).
fn section(input: &mut &str) -> ParserResult<Section> {
    let address = preceded(multispace0, hex_value).parse_next(input);
    let size = preceded(multispace0, hex_value).parse_next(input);
    spaces.parse_next(input)?;
    let segment_name = parse_segment_or_section.parse_next(input)?;
    spaces.parse_next(input)?;
    let section_name = parse_segment_or_section.parse_next(input)?;

    Ok(Section {
        address: address.unwrap().to_string(),
        size: size.unwrap().to_string(),
        segment: segment_name.to_string(),
        section: section_name.to_string(),
    })
}

/// Parse consecutive section rows until `# Symbols:` or EOF.
fn sections(input: &mut &str) -> ParserResult<Vec<Section>> {
    repeat_till(
        0..,
        terminated(section, opt(line_ending)),
        alt((
            peek(terminated(
                preceded(spaces, literal(MapFileHeaders::Symbols.as_str())),
                opt(line_ending),
            )),
            eof,
        )),
    )
    .parse_next(input)
    .map(|(sections, _)| sections)
}

/// Parse sections block after `# Sections:` header.
fn section_table(input: &mut &str) -> ParserResult<Vec<Section>> {
    delimited(
        opt(line_ending),
        literal(MapFileHeaders::Sections.as_str()),
        line_ending,
    )
    .parse_next(input)?;
    terminated(till_line_ending, line_ending).parse_next(input)?;
    sections.parse_next(input)
}

/// Parse target path header (e.g. `# Path: /target/debug/deps/sample-app`).
fn target_path<'i>(input: &mut &'i str) -> ParserResult<&'i str> {
    preceded(
        literal(MapFileHeaders::Path.as_str()),
        preceded(multispace0, till_line_ending),
    )
    .context(StrContext::Label("Path"))
    .context(StrContext::Expected(StrContextValue::Description(
        "Expected a Path",
    )))
    .parse_next(input)
}

/// Parse whole map: Path -> Arch -> Object files -> Sections -> Symbols.
pub fn parse(map_file: &Path) -> ParserResult<MapFile> {
    let contents = read_file(map_file);
    let mut input = contents.as_str();
    let parsed_map_file = seq!(MapFile {
        target_path: target_path.map(|s: &str| s.to_string()),
        arch: arch.map(|s: &str| s.to_string()),
        object_files: object_files,
        sections: section_table,
        symbols: symbol_table,
    })
    .parse_next(&mut input)?;

    Ok(parsed_map_file)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arch() {
        let mut input = "# Arch: x86_64";
        let result = arch(&mut input);
        assert_eq!(result.unwrap(), "x86_64");
    }

    #[test]
    fn test_object_files() {
        let mut input = r"# Object files:
[ 66] /Library/Developer/CommandLineTools/SDKs/MacOSX14.4.sdk/usr/lib/system/libunwind.tbd";
        let result = object_files(&mut input);
        let object_files = result.unwrap();

        assert_eq!(object_files.len(), 1);
        assert_eq!(object_files[0].index, 66);
        assert_eq!(
            object_files[0].path,
            "/Library/Developer/CommandLineTools/SDKs/MacOSX14.4.sdk/usr/lib/system/libunwind.tbd"
        );
    }

    #[test]
    fn test_target_path() {
        let mut input = "# Path: /target/debug/deps/sample-app";
        let result = target_path(&mut input);
        assert_eq!(result.unwrap(), "/target/debug/deps/sample-app");
    }

    #[test]
    fn test_object_files_immediately_followed_by_sections() {
        let mut input = r"# Object files:
[  0] a.o
[  1] b.o
# Sections:
# Address	Size    	Segment Section
0x0	0x1	__TEXT	__text";
        let result = object_files(&mut input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].index, 0);
        assert_eq!(result[0].path, "a.o");
        // Remaining input should start with '# Sections:' now
        assert!(input.starts_with("# Sections:"));
    }

    #[test]
    fn test_symbol() {
        let mut input = r"0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E";
        let result = symbol(&mut input);
        let symbol = result.unwrap();
        assert_eq!(symbol.address, "0x10004C058");
        assert_eq!(symbol.size, "0x00000018");
        assert_eq!(symbol.file_index, "1");
        assert_eq!(
            symbol.name,
            "__ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E"
        );
    }

    #[test]
    fn test_single_symbol_row() {
        let mut input = r"0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E";
        let result = symbols(&mut input);
        let symbols = result.unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].address, "0x10004C058");
    }

    #[test]
    fn test_multiple_symbol_rows() {
        let mut input = r"
        0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E
        0x10004C059	0x00000020	[  2] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E";
        let result = symbols(&mut input);
        let symbols = result.unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].address, "0x10004C058");
        assert_eq!(symbols[1].address, "0x10004C059");
    }

    #[test]
    fn test_symbol_table() {
        let mut input = r"# Symbols:
    # Address	Size    	File  Name
    0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E";
        let result = symbol_table(&mut input);
        let symbols = result.unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].address, "0x10004C058");
    }

    #[test]
    fn test_single_section_row() {
        let mut input = r"0x10004C058	0x00000018	__TEXT	__text";
        let result = section(&mut input);
        let section = result.unwrap();
        assert_eq!(section.address, "0x10004C058");
        assert_eq!(section.size, "0x00000018");
        assert_eq!(section.segment, "__TEXT");
        assert_eq!(section.section, "__text");
    }

    #[test]
    fn test_section_with_underscores() {
        // Use actual tab characters, matching fixture formatting
        let mut input = "0x10003BBB8\t0x000017F4\t__TEXT\t__gcc_except_tab";
        let result = section(&mut input).unwrap();
        assert_eq!(result.segment, "__TEXT");
        assert_eq!(result.section, "__gcc_except_tab");
    }

    #[test]
    fn test_sections() {
        let mut input = r"0x10004C058	0x00000018	__TEXT	__text
        0x10004C059	0x00000020	__TEXT	__text";
        let result = sections(&mut input);
        let sections = result.unwrap();
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].address, "0x10004C058");
        assert_eq!(sections[0].section, "__text");
        assert_eq!(sections[0].segment, "__TEXT");
        assert_eq!(sections[1].address, "0x10004C059");
        assert_eq!(sections[1].section, "__text");
        assert_eq!(sections[1].segment, "__TEXT");
    }

    #[test]
    fn test_section_table() {
        let mut input = r"# Sections:
        # Address	Size    	Segment Section
        0x10004C058	0x00000018	__TEXT	__text";
        let result = section_table(&mut input);
        let sections = result.unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].address, "0x10004C058");
        assert_eq!(sections[0].size, "0x00000018");
        assert_eq!(sections[0].segment, "__TEXT");
        assert_eq!(sections[0].section, "__text");
    }

    #[test]
    fn test_section_table_stops_at_symbols() {
        let mut input = r"# Sections:
    # Address	Size    	Segment Section
    0x10004C058	0x00000018	__TEXT	__text
    # Symbols:
    0x10004C060	0x00000030	[  1] some_symbol";
        let result = section_table(&mut input);
        let sections = result.unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].address, "0x10004C058");
    }

    #[test]
    fn test_parse_small_sample() {
        let map_file = Path::new("tests/fixtures/sample.map");
        let result = parse(map_file).unwrap();
        assert_eq!(result.target_path, "/target/debug/deps/sample-app");
        assert_eq!(result.arch, "arm64");
        assert_eq!(result.object_files.len(), 1);
        assert_eq!(result.sections.len(), 1);
        assert_eq!(result.symbols.len(), 1);
    }

    #[test]
    fn test_parse_large_sample() {
        let map_file = Path::new("tests/fixtures/linker.map");
        let result = parse(map_file).unwrap();
        assert_eq!(
            result.target_path,
            "/target/debug/deps/learning_linkers-97732971bdfee10d"
        );
        assert_eq!(result.arch, "arm64");
        assert_eq!(result.object_files.len(), 67);
        assert_eq!(result.sections.len(), 14);
        assert!(result.symbols.len() > 0);
    }
}
