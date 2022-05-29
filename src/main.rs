fn main() {
    if let Err(err) = csvblanky::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
