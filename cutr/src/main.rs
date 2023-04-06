fn main() {
    if let Err(e) = cutr::parse_args().and_then(cutr::run) {
        eprintln!("{}", e);
    }
}
