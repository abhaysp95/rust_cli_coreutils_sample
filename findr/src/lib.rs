use std::{error::Error, path::PathBuf};

use clap::{arg, Arg, ArgAction, Command};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FindType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<PathBuf>,
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
                .default_value(".")
                // .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Append)
                .help("Provide paths to search"),
        )
        .arg(
            arg!(-t --type <TYPE> "match type of the file")
                .action(ArgAction::Append)
                .value_parser(["d", "f", "l"])
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
        .map(|path| PathBuf::from(path)) // why do this ?
        .collect::<Vec<PathBuf>>();

    let ftypes = matches
        .get_many::<String>("type")
        .unwrap_or_default()
        .map(|t| t.as_str())
        .map(|t| match t {
            "d" => FindType::Dir,
            "f" => FindType::File,
            "l" => FindType::Link,
            _ => unreachable!(),
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
    let type_filter = |entry: &DirEntry| {
        cfg.ftypes.is_empty()
            || cfg.ftypes.iter().any(|ft| match ft {
                FindType::Link => entry.file_type().is_symlink(),
                FindType::Dir => entry.file_type().is_dir(),
                FindType::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        return cfg.names.is_empty()
            || cfg
                .names
                .iter()
                .any(|name| name.is_match(&entry.file_name().to_string_lossy()));
    };

    for path in &cfg.paths {
        let entries = WalkDir::new(&path)
            .into_iter()
            .filter_map(|entry| match entry {
                Err(e) => {
                    println!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<String>>();

        println!("{}", entries.join("\n"));
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

// -----------------------------------------------------------

/* for path in cfg.paths {
    for entry in WalkDir::new(&path) {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(entry) => {
                if (cfg.ftypes.is_empty()
                    || cfg.ftypes.iter().any(|ft| match ft {
                        FindType::File => entry.file_type().is_file(),
                        FindType::Dir => entry.file_type().is_dir(),
                        FindType::Link => entry.file_type().is_symlink(),
                    }))
                    && (cfg.names.is_empty()
                        || cfg
                            .names
                            .iter()
                            .any(|name| name.is_match(entry.file_name().to_str().unwrap())))
                {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
} */

/* let entries = WalkDir::new(&path)
    .into_iter()  // iterators are lazy and do nothing unless consumed
    .filter_map(|entry| match entry {
        Err(e) => {
            println!("{}", e);
            None
        }
        Ok(entry) => Some(entry),
    })
    .filter(type_filter)
    .filter(name_filter)
    .map(|entry| entry.path().display().to_string())
    .collect::<Vec<String>>();

println!("{}", entries.join("\n")); */
