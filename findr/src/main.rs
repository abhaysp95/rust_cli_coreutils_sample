use findr::{parse_args, run};

fn main() {
    if let Err(e) = parse_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
