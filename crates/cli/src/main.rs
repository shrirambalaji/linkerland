fn main() {
    if let Err(e) = linkerland::run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
