#![allow(dead_code)]

use std::path::Path;
use std::str;

use winnow::{
    ascii::{digit1, line_ending, multispace0, till_line_ending},
    combinator::{alt, delimited, eof, opt, peek, preceded, repeat, repeat_till, terminated},
    error::{StrContext, StrContextValue},
    token::literal,
    PResult, Parser,
};

#[derive(Debug)]
struct Symbol {
    address: String,
    size: String,
    file_index: String,
    name: String,
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

#[derive(Debug)]
struct ObjectFile {
    index: i32,
    path: String,
}

fn read_file(map_file: &Path) -> String {
    match std::fs::read_to_string(map_file) {
        Ok(contents) => contents,
        Err(e) => {
            panic!("Error reading file: {:?}", e);
        }
    }
}

fn spaces<'i>(input: &mut &'i str) -> PResult<&'i str> {
    multispace0(input)
}

fn arch<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded(
        literal(MapFileHeaders::Architecture.as_str()),
        preceded(spaces, till_line_ending),
    )
    .parse_next(input)
}

fn hex_value<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded("0x", winnow::ascii::hex_digit1).parse_next(input)
}

fn object_file(input: &mut &str) -> PResult<ObjectFile> {
    (
        delimited('[', preceded(spaces, digit1), ']'),
        preceded(spaces, till_line_ending),
    )
        .map(|(index, path)| ObjectFile {
            index: index.parse().unwrap(),
            path: path.to_string(),
        })
        .parse_next(input)
}

fn object_files(input: &mut &str) -> PResult<Vec<ObjectFile>> {
    terminated(literal(MapFileHeaders::ObjectFiles.as_str()), line_ending).parse_next(input)?;
    repeat_till(
        0..,
        object_file,
        alt((peek(preceded(line_ending, literal("#"))), eof)),
    )
    .parse_next(input)
    .map(|(object_files, _)| object_files)
}

fn symbol(input: &mut &str) -> PResult<Symbol> {
    let address = preceded(spaces, hex_value).parse_next(input);
    let size = preceded(spaces, hex_value).parse_next(input);
    let file_index =
        preceded(spaces, delimited('[', preceded(spaces, digit1), ']')).parse_next(input);
    let name = preceded(spaces, till_line_ending).parse_next(input);

    // TODO: I don't like that we have to prepend "0x" to the address and size values
    let mut symbol_addr = address.unwrap().to_string();
    symbol_addr.insert_str(0, "0x");

    let mut symbol_size = size.unwrap().to_string();
    symbol_size.insert_str(0, "0x");

    let symbol = Symbol {
        address: symbol_addr,
        size: symbol_size,
        file_index: file_index.unwrap().to_string(),
        name: name.unwrap().to_string(),
    };

    Ok(symbol)
}

fn symbols(input: &mut &str) -> PResult<Vec<Symbol>> {
    ((
        opt(line_ending),
        symbol,
        repeat(0.., preceded(line_ending, symbol)).fold(Vec::new, |mut acc, item| {
            acc.push(item);
            acc
        }),
    ))
        .parse_next(input)
        .map(|(_, first, rest)| {
            let mut symbols = vec![first];
            symbols.extend(rest);
            symbols
        })
}

fn symbol_table<'i>(input: &mut &'i str) -> PResult<Vec<Symbol>> {
    terminated(literal(MapFileHeaders::Symbols.as_str()), line_ending).parse_next(input)?;
    terminated(till_line_ending, line_ending).parse_next(input)?;
    symbols.parse_next(input)
}

fn target_path<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded(
        literal(MapFileHeaders::Path.as_str()),
        preceded(spaces, till_line_ending),
    )
    .context(StrContext::Label("Path"))
    .context(StrContext::Expected(StrContextValue::Description(
        "Expected a Path",
    )))
    .parse_next(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arch() {
        let mut input = "# Arch: x86_64";
        let result = arch(&mut input);
        match result {
            Ok(data) => {
                println!("Parsed data: {:?}", data);
            }
            Err(e) => {
                panic!("Error parsing: {:?}", e);
            }
        }
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
}
