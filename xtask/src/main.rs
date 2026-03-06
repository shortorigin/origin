fn main() {
    if let Err(error) = xtask::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
