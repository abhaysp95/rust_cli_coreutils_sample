use std::{error::Error, io};

use clap::{command, Parser, ValueEnum};
use regex::Regex;

#[derive(PartialEq, Eq, Debug, Clone, ValueEnum)]
enum FindType {
    /// search directories
    Dir,
    /// search files
    File,
    /// search symlinks
    Link,
}

#[derive(Parser, Debug)]
#[command(about, version, long_about=None)]
/// simple rust clone of find
pub struct Config {
    /// pathes to find on
    #[arg()]
    path: Vec<String>,
    #[arg(short='t', long="type", value_enum)]
    ftype: Vec<FindType>,
    /// pattern to find
    #[arg(long)]
    name: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> MyResult<Config> {
    let mut cfg = Config::parse();

    if cfg.path.is_empty() {
        cfg.path = vec![String::from(".")];
    }

    Ok(cfg)
}

pub fn run(cfg: Config) -> MyResult<()> {
    dbg!(&cfg);  // will show everything escaped (but actually it is not)

    println!("{:?}", cfg.name);

    let mut to_regex = String::from("");


    println!("{}", to_regex);

    Ok(())
}

fn regex_do_match(haystack: &str, needle: &str) -> bool {
    let reg = Regex::new(needle).unwrap();
    reg.is_match(&haystack)
}

#[test]
fn test_regex() {
    assert_eq!(regex_do_match("this is good", "is g"), true);
    assert_eq!(regex_do_match("this is good", ".*is g"), true);
    assert_eq!(regex_do_match("this. is good", r"\..*is g"), true);
    assert_eq!(regex_do_match("this. is good", "\\..*is g"), true);

    // rest, I'm trusting regex package here and will add tests if something I need needed to be
    // tested for my understanding
}
