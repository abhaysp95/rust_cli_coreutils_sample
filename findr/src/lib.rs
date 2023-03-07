use std::{error::Error, path::PathBuf};

use clap::{arg, value_parser, Arg, ArgAction, Command};
use regex::Regex;
use walkdir::WalkDir;

// TODO: switch to Builder pattern (possibly, derive pattern not working)

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FindType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
struct Config {
    paths: Vec<String>,
    ftypes: Vec<FindType>,
    names: Vec<Regex>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("coolabhays")
        .about("simple clone of find in rust")
        .arg(
            Arg::new("paths")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Append)
                .help("Provide paths to search"),
        )
        .arg(
            arg!(-t --type <TYPE> "match type of the file")
                .action(ArgAction::Append)
                .value_parser(["d", "f", "l"])
                .default_value("f")
                .help("Types of file to search"),
        )
        .arg(
            arg!(-n --name <NAME> "match the pattern for the result")
                .action(ArgAction::Append)
                .help("Patterns to search"),
        )
        .get_matches();

    let paths = matches
        .get_many::<String>("paths")
        .unwrap_or_default()
        .map(|path| path.clone()) // why do this ?
        .collect::<Vec<String>>();

    let ftypes = matches
        .get_many::<String>("type")
        .unwrap_or_default()
        .map(|t| t.as_str())
        .map(|t| match t {
            "d" => FindType::Dir,
            "f" => FindType::File,
            "l" => FindType::Link,
        })
        .collect::<Vec<FindType>>();

    let names = matches
        .get_many::<String>("name")
        .unwrap_or_default()
        .map(move |n| Regex::new(n).unwrap())
        .collect::<Vec<Regex>>();

    Ok(Config {
        paths,
        ftypes,
        names,
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    dbg!(&cfg); // will show everything escaped (but actually it is not)

    let mut find_res: Vec<String> = vec![];

    for path in cfg.paths {
        for entry in WalkDir::new(&path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let ft = entry.file_type();
                    for ftype in &cfg.ftypes {
                        match ftype {
                            FindType::File => {
                                if ft.is_file() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            }
                            FindType::Dir => {
                                if ft.is_dir() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            }
                            FindType::Link => {
                                if ft.is_symlink() {
                                    find_res.push(entry.path().display().to_string());
                                }
                            }
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
