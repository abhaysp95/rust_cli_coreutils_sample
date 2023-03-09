use std::{error::Error, ops::Range};

use clap::{arg, command, ArgGroup};

type MyResult<T> = Result<T, Box<dyn Error>>;
type ExtractRange = Vec<Range<usize>>;

#[derive(Debug)]
pub struct Config {
    path: String,
    delim: Option<char>,
    extract: ExtractCount,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtractCount {
    Byte(ExtractRange),
    Char(ExtractRange),
    Fields(ExtractRange),
}

fn get_ranges(input: &str) -> Vec<Range<usize>> {
    let mut vec = vec![];
    for each_range in input.trim().split(",") {
        let idx = each_range.split("-").collect::<Vec<&str>>();
        vec.push(Range {
            start: idx[0].parse().unwrap(),
            end: idx[1].parse().unwrap(),
        });
    }

    vec
}

#[test]
fn get_ranges_test() {
    // assert_eq!(get_ranges("1-"), [(1..)]);
}

pub fn parse_args() -> MyResult<Config> {
    let mut cmd = command!()
        .arg(arg!(<Path>))
        .arg(arg!(-b --byte <Byte> "Select only these bytes"))
        .arg(arg!(-c --char <Char> "Select only these chars"))
        .arg(arg!(-f --field <Field> "Select only these fields"))
        .group(
            ArgGroup::new("Extract")
                .required(true)
                .args(["byte", "char", "field"]),
        )
        .arg(arg!(-d --delim <Delimeter> "Provide the delim char"));
    let matches = cmd.get_matches_mut();

    let delim = matches.get_one::<char>("delim");  // <-- gives Option<&char>, want Option<char>
    let delim = match delim {
        Some(c) => Some(c.to_owned()),
        None => None,
    };

    let extract = if let Some(fr) = matches.get_one::<String>("field") {
        match delim {
            None => ExtractCount::Fields(get_ranges(fr)),
            Some(_) => {
                cmd.error(
                    clap::error::ErrorKind::ArgumentConflict,
                    "Delimiter can only be used with Fields",
                )
                .exit();
            }
        }
    } else {
        if let Some(br) = matches.get_one::<String>("byte") {
            ExtractCount::Byte(get_ranges(br))
        } else if let Some(cr) = matches.get_one::<String>("char") {
            ExtractCount::Char(get_ranges(cr))
        } else {
            unreachable!()
        }
    };

    Ok(Config {
        path: matches.get_one::<String>("path").unwrap().to_string(),
        delim,
        extract,
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    dbg!(cfg);

    Ok(())
}
