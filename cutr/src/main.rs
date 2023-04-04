fn main() {
    if let Err(e) = cutr::parse_args().and_then(|cfg| cutr::run(cfg)) {
        eprintln!("{}", e);
    }
}
