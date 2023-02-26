fn main() {
    if let Err(e) = runiq::parse_args().and_then(runiq::run) {
        eprintln!("{}", e);
    }
}
