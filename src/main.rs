use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use regex::Regex;

use parser::parse;
use tui::{run as tui_run, TuiOptions};

/// Linker map exploration toolkit
#[derive(ClapParser, Debug)]
#[command(
    version,
    about = "Inspect and visualize linker map (.map) files",
    arg_required_else_help = true
)]
struct Cli {
    /// Path to a .map file (shorthand default action = viz TUI)
    #[arg(value_parser = validate_map_path, required = false)]
    mapfile: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn validate_map_path(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if p.extension().map(|e| e == "map").unwrap_or(false) {
        Ok(p)
    } else {
        Err("expected path ending with .map".into())
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive terminal visualization (TUI)
    Viz(VizArgs),
    /// Export map data to JSON or CSV
    Export(ExportArgs),
}

#[derive(clap::Args, Debug)]
struct VizArgs {
    /// Map file to visualize
    #[arg(value_parser = validate_map_path)]
    mapfile: PathBuf,
    /// Filter symbols (regex applied to symbol name)
    #[arg(long)]
    filter: Option<String>,
    /// Sort key
    #[arg(long, value_enum, default_value_t = SortKey::Size)]
    sort: SortKey,
    /// Sort order
    #[arg(long, value_enum, default_value_t = SortOrder::Desc)]
    order: SortOrder,
}

#[derive(clap::Args, Debug)]
struct ExportArgs {
    /// Map file to export
    #[arg(value_parser = validate_map_path)]
    mapfile: PathBuf,
    /// Output format
    #[arg(long, value_enum, default_value_t = ExportFormat::Json)]
    format: ExportFormat,
    /// Regex filter on symbol name
    #[arg(long)]
    filter: Option<String>,
    /// Output file (stdout if omitted)
    #[arg(long)]
    out: Option<PathBuf>,
    /// Sort key
    #[arg(long, value_enum, default_value_t = SortKey::Size)]
    sort: SortKey,
    /// Sort order
    #[arg(long, value_enum, default_value_t = SortOrder::Desc)]
    order: SortOrder,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortKey {
    Size,
    Name,
    Path,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortOrder {
    Asc,
    Desc,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ExportFormat {
    Json,
    Csv,
}

fn main() {
    let cli = Cli::parse();

    if let (Some(mapfile), None) = (&cli.mapfile, &cli.command) {
        run_viz(VizArgs {
            mapfile: mapfile.clone(),
            filter: None,
            sort: SortKey::Size,
            order: SortOrder::Desc,
        });
        return;
    }

    match cli.command {
        Some(Commands::Viz(args)) => run_viz(args),
        Some(Commands::Export(args)) => {
            if let Err(e) = run_export(args) {
                eprintln!("Error: Failed to export symbols: {}", e);
                std::process::exit(1);
            }
        }
        None => {}
    }
}

fn run_viz(args: VizArgs) {
    // Placeholder: parse file and print summary until TUI implemented
    match parse(&args.mapfile) {
        Ok(map) => {
            let opts = TuiOptions {
                sort: format!("{:?}", args.sort),
                filter: args.filter.clone(),
            };
            if let Err(e) = tui_run(&map, &opts) {
                eprintln!("TUI error: {e}");
            }
        }
        Err(e) => eprintln!("Failed to parse map: {:?}", e),
    }
}

fn run_export(args: ExportArgs) -> anyhow::Result<()> {
    let map = parse(&args.mapfile).map_err(|e| anyhow::anyhow!("parse error: {:?}", e))?;

    let mut symbols: Vec<_> = map.symbols.iter().collect();
    // Filter
    if let Some(f) = &args.filter {
        let re = Regex::new(f).map_err(|e| anyhow::anyhow!(e))?;
        symbols.retain(|s| re.is_match(&s.name));
    }
    // Sort
    symbols.sort_by(|a, b| {
        use SortKey::*;
        use SortOrder::*;
        let ord = match args.sort {
            Size => cmp_hex_size(&a.size, &b.size),
            Name => a.name.cmp(&b.name),
            Path => a.file_index.cmp(&b.file_index),
        };
        match args.order {
            Asc => ord,
            Desc => ord.reverse(),
        }
    });

    match args.format {
        ExportFormat::Json => export_json(&symbols, &args),
        ExportFormat::Csv => export_csv(&symbols, &args),
    }
}

fn cmp_hex_size(a: &str, b: &str) -> std::cmp::Ordering {
    let pa = u64::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
    let pb = u64::from_str_radix(b.trim_start_matches("0x"), 16).unwrap_or(0);
    pa.cmp(&pb)
}

fn export_json(symbols: &[&parser::Symbol], args: &ExportArgs) -> anyhow::Result<()> {
    let mut out: Box<dyn Write> = if let Some(path) = &args.out {
        Box::new(File::create(path)?)
    } else {
        Box::new(io::stdout())
    };
    serde_json::to_writer_pretty(&mut out, &symbols)?;
    writeln!(out)?;
    Ok(())
}

fn export_csv(symbols: &[&parser::Symbol], args: &ExportArgs) -> anyhow::Result<()> {
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
