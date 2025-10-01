use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use regex::Regex;

use parser::parse;
use tui::run as tui_run;

#[derive(ClapParser, Debug)]
#[command(version, arg_required_else_help = true)]
pub struct Cli {
    #[arg(value_parser = validate_map_path, required = false)]
    pub mapfile: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

fn validate_map_path(s: &str) -> std::result::Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if p.extension().map(|e| e == "map").unwrap_or(false) {
        Ok(p)
    } else {
        Err("expected path ending with .map".into())
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Viz(VizArgs),
    Export(ExportArgs),
}

#[derive(clap::Args, Debug)]
pub struct VizArgs {
    #[arg(value_parser = validate_map_path)]
    pub mapfile: PathBuf,
    #[arg(long)]
    pub filter: Option<String>,
    #[arg(long, value_enum, default_value_t = SortKey::Size)]
    pub sort: SortKey,
    #[arg(long, value_enum, default_value_t = SortOrder::Desc)]
    pub order: SortOrder,
}

#[derive(clap::Args, Debug)]
pub struct ExportArgs {
    #[arg(value_parser = validate_map_path)]
    pub mapfile: PathBuf,
    #[arg(long, value_enum, default_value_t = ExportFormat::Json)]
    pub format: ExportFormat,
    #[arg(long)]
    pub filter: Option<String>,
    #[arg(long)]
    pub out: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = SortKey::Size)]
    pub sort: SortKey,
    #[arg(long, value_enum, default_value_t = SortOrder::Desc)]
    pub order: SortOrder,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum SortKey {
    Size,
    Name,
    Path,
}
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ExportFormat {
    Json,
    Csv,
}

pub fn run() -> Result<()> {
    run_with(Cli::parse())
}

pub fn run_with(cli: Cli) -> Result<()> {
    if let (Some(mapfile), None) = (&cli.mapfile, &cli.command) {
        viz(VizArgs {
            mapfile: mapfile.clone(),
            filter: None,
            sort: SortKey::Size,
            order: SortOrder::Desc,
        })?;
        return Ok(());
    }
    match cli.command {
        Some(Commands::Viz(args)) => viz(args)?,
        Some(Commands::Export(args)) => export(args)?,
        None => { /* clap already showed help */ }
    }
    Ok(())
}

fn viz(args: VizArgs) -> Result<()> {
    parse(&args.mapfile).map_err(|e| anyhow::anyhow!("parse error: {:?}", e))?;
    if let Err(e) = tui_run(args.mapfile.to_string_lossy().as_ref()) {
        eprintln!("TUI error: {e}");
    }
    Ok(())
}

fn export(args: ExportArgs) -> Result<()> {
    let map = parse(&args.mapfile).map_err(|e| anyhow::anyhow!("parse error: {:?}", e))?;
    let mut symbols: Vec<_> = map.symbols.iter().collect();
    if let Some(f) = &args.filter {
        let re = Regex::new(f)?;
        symbols.retain(|s| re.is_match(&s.name));
    }
    symbols.sort_by(|a, b| {
        use SortKey::*;
        use SortOrder::*;
        let ord = match args.sort {
            Size => compare_hex_size(&a.size, &b.size),
            Name => a.name.cmp(&b.name),
            Path => a.file_index.cmp(&b.file_index),
        };
        match args.order {
            Asc => ord,
            Desc => ord.reverse(),
        }
    });
    match args.format {
        ExportFormat::Json => export_json(&symbols, &args)?,
        ExportFormat::Csv => export_csv(&symbols, &args)?,
    }
    Ok(())
}

fn compare_hex_size(a: &str, b: &str) -> std::cmp::Ordering {
    let pa = u64::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
    let pb = u64::from_str_radix(b.trim_start_matches("0x"), 16).unwrap_or(0);
    pa.cmp(&pb)
}

fn export_json(symbols: &[&parser::Symbol], args: &ExportArgs) -> Result<()> {
    let mut out: Box<dyn Write> = if let Some(path) = &args.out {
        Box::new(File::create(path)?)
    } else {
        Box::new(io::stdout())
    };
    serde_json::to_writer_pretty(&mut out, &symbols)?;
    writeln!(out)?;
    Ok(())
}

fn export_csv(symbols: &[&parser::Symbol], args: &ExportArgs) -> Result<()> {
    let mut wtr: csv::Writer<Box<dyn Write>> = if let Some(path) = &args.out {
        csv::Writer::from_writer(Box::new(File::create(path)?))
    } else {
        csv::Writer::from_writer(Box::new(io::stdout()))
    };
    for s in symbols {
        wtr.serialize(s)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_extension_rejected() {
        assert!(validate_map_path("foo.txt").is_err());
    }
}
