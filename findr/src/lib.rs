use std::{error::Error, fs::DirEntry};

use clap::{command, Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

// TODO: switch to Builder pattern (possibly, derive pattern not working)

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
    /// paths to find on
    #[arg()]
    paths: Vec<String>,
    #[arg(short = 't', long = "type", value_enum)]
    ftype: Vec<FindType>,
    /// pattern to find
    #[arg(long = "name")]
    names: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> MyResult<Config> {
    let mut cfg = Config::parse();

    if cfg.paths.is_empty() {
        cfg.paths = vec![String::from(".")];
    }

    Ok(cfg)
}

pub fn run(cfg: Config) -> MyResult<()> {
    dbg!(&cfg); // will show everything escaped (but actually it is not)

    #[allow(unused_variables)]
    let names = cfg
        .names
        .iter()
        .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
        .collect::<Result<Vec<_>, _>>()?;

    let mut find_res: Vec<String> = vec![];

    for path in cfg.paths {
        for entry in WalkDir::new(&path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let ft = entry.file_type();
                    for ftype in &cfg.ftype {
                        match ftype {
                            FindType::File => {
                                if ft.is_file() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            },
                            FindType::Dir => {
                                if ft.is_dir() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            },
                            FindType::Link => {
                                if ft.is_symlink() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            },
                        }
                    }
                }
            }
        }
    }

    for fr in &find_res {
        for np in &names {
            if np.is_match(&fr) {
                println!("{}", fr);
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
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
