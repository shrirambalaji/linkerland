#![allow(dead_code)]

use std::path::Path;
use std::str;
use std::string::ParseError;

use winnow::ascii::{alpha0, alphanumeric1, digit1, newline, till_line_ending};
use winnow::combinator::delimited;
use winnow::error::{ContextError, ErrorKind, InputError, StrContext, StrContextValue};
use winnow::PResult;
use winnow::{
    ascii::multispace0,
    ascii::multispace1,
    combinator::{preceded, repeat, terminated},
    token::literal,
    Parser,
};

#[derive(Debug)]
struct Symbol {
    address: String,
    size: String,
    file_index: String,
    name: String,
}

/// We are referring to every meaningful section in the linker map file as a block.

#[derive(Debug)]
enum BlockHeaders {
    Path,
    Architecture,
    ObjectFiles,
    Symbols,
    Sections,
}

#[cfg(target_os = "macos")]
impl BlockHeaders {
    fn as_str(&self) -> &str {
        match self {
            BlockHeaders::Path => "# Path:",
            BlockHeaders::Architecture => "# Arch:",
            BlockHeaders::ObjectFiles => "# Object files:",
            BlockHeaders::Sections => "# Sections:",
            BlockHeaders::Symbols => "# Symbols:",
        }
    }
}

// trait Parser {
//     fn read_file(&mut self, map_file: &Path) -> String;
//     fn parse(&mut self, contents: &str);
//     fn parse_blocks(&mut self, contents: &str) -> IResult<&str, &str>;
//     fn parse_block(&mut self, contents: &str, header: &str, until: &str) -> IResult<&str, &str>;
// }

// pub(crate) type Stream<'i> = &'i [u8];

#[derive(Debug)]
enum ParsedData {
    Path(String),
    Arch(String),
    ObjectFiles(Vec<String>),
    Symbols(Vec<Symbol>),
}

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

fn object_files<'i>(input: &mut &'i str) -> PResult<Vec<ObjectFile>> {
    let _ =
        terminated(literal(BlockHeaders::ObjectFiles.as_str()), multispace0).parse_next(input)?;

    let mut object_files = Vec::new();
    let index = delimited('[', preceded(spaces, winnow::ascii::digit1), ']').parse_next(input);

    let path = preceded(spaces, winnow::ascii::alphanumeric1).parse_next(input);

    object_files.push(ObjectFile {
        index: index.unwrap().parse::<i32>().unwrap(),
        path: path.unwrap().to_string(),
    });

    Ok(object_files)
}

fn target_path<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded(
        literal(BlockHeaders::Path.as_str()),
        preceded(spaces, till_line_ending),
    )
    .context(StrContext::Label("Path"))
    .context(StrContext::Expected(
        (StrContextValue::Description(("Expected a Path"))),
    ))
    .parse_next(input)
}

fn arch<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded(
        literal(BlockHeaders::Architecture.as_str()),
        preceded(spaces, till_line_ending),
    )
    .parse_next(input)
}

// TODO: fix this so that it returns the column headers
fn symbol_table_header<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded(literal("# Address"), preceded(spaces, till_line_ending)).parse_next(input)
}

fn hex_value<'i>(input: &mut &'i str) -> PResult<&'i str> {
    preceded("0x", winnow::ascii::hex_digit1).parse_next(input)
}

fn symbol<'i>(input: &mut &'i str) -> PResult<Symbol> {
    let address = preceded(spaces, hex_value).parse_next(input);
    let size = preceded(spaces, hex_value).parse_next(input);
    let file_index =
        preceded(spaces, delimited('[', preceded(spaces, digit1), ']')).parse_next(input);

    let name = preceded(spaces, winnow::ascii::till_line_ending).parse_next(input);

    // TODO: I don't like that we have to prepend "0x" to the address and size values
    let mut symbol_addr = address.unwrap().to_string();
    symbol_addr.insert_str(0, "0x");

    let mut symbol_size = size.unwrap().to_string();
    symbol_size.insert_str(0, "0x");

    Ok(Symbol {
        address: symbol_addr,
        size: symbol_size,
        file_index: file_index.unwrap().to_string(),
        name: name.unwrap().to_string(),
    })
}

fn symbols<'i>(input: &mut &'i str) -> PResult<Vec<Symbol>> {
    preceded(
        literal(BlockHeaders::Symbols.as_str()),
        preceded(
            symbol_table_header,
            repeat(0.., symbol).fold(Vec::new, |mut acc, x| {
                acc.push(x);
                acc
            }),
        ),
    )
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
    fn test_target_path() {
        let mut input = "# Path: /target/debug/deps/sample-app";
        let result = target_path(&mut input);
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
    fn test_symbol_table_header() {
        let mut input = "# Address	Size    	File  Name";
        let result = symbol_table_header(&mut input);
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
    fn test_symbol() {
        let mut input = r"0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E";
        let result = symbol(&mut input);
        match result {
            Ok(data) => {
                println!("Parsed data: {:?}", data);
            }
            Err(e) => {
                panic!("Error parsing: {:?}", e);
            }
        }
    }

    #[ignore]
    #[test]
    fn test_symbols() {
        let mut input = r"# Symbols:
        # Address	Size    	File  Name
        0x10004C058	0x00000018	[  1] __ZN3std3sys3pal4unix17thread_local_dtor13register_dtor5DTORS17hf7230a0b661819a4E
        ";
        let result = symbols(&mut input);
        match result {
            Ok(data) => {
                println!("Parsed data: {:?}", data);
            }
            Err(e) => {
                panic!("Error parsing: {:?}", e);
            }
        }
    }

    // #[test]
    // fn test_parser() {
    //     let contents = read_file(Path::new("./fixtures/tiny_sample.map"));
    //     let result = parse(&contents);
    //     match result {
    //         Ok((_, data)) => {
    //             println!("Parsed data: {:?}", data);
    //         }
    //         Err(e) => {
    //             panic!("Error parsing: {:?}", e);
    //         }
    //     }
    // }
}
