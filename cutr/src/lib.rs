use std::{error::Error, num::NonZeroUsize, ops::Range};

use clap::{arg, command, ArgGroup};
use regex::Regex;

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

pub fn parse_index(input: &str) -> Result<usize, String> {
    let err_str = || format!("illegal list value: \"{input}\"");

    input
        .starts_with("+")
        .then(|| Err(err_str()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| err_str())
        })
}

fn get_positions(input: &str) -> MyResult<ExtractRange> {
    let regex_str = Regex::new(r"^(\d+)-(\d+)$").unwrap();

    input
        .split(",")
        .into_iter()
        .map(|val| {
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                regex_str.captures(val).ok_or(e).and_then(|captures| {
                    println!("captures: {:?}", captures);
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 > n2 {
                        return Err(format!(
                            "first number in range ({}) must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        ));
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

#[cfg(test)]
mod unit_tests {
    use super::get_positions;

    #[test]
    fn get_ranges_test() {
        // The empty string is an error
        assert!(get_positions("").is_err());
        // Zero is an error
        let res = get_positions("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);
        let res = get_positions("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);
        // A leading "+" is an error
        let res = get_positions("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"",);
        let res = get_positions("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"",);
        let res = get_positions("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-+2\"",);
        // Any non-number is an error
        let res = get_positions("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);
        let res = get_positions("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);
        let res = get_positions("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-a\"",);
        let res = get_positions("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a-1\"",);
        // Wonky ranges
        let res = get_positions("-");
        assert!(res.is_err());
        let res = get_positions(",");
        assert!(res.is_err());
        let res = get_positions("1,");
        assert!(res.is_err());
        let res = get_positions("1-");
        assert!(res.is_err());
        assert!(res.is_err());
        let res = get_positions("1-1-1");
        assert!(res.is_err());
        let res = get_positions("1-1-a");
        assert!(res.is_err());
        // First number must be less than second
        let res = get_positions("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );
        let res = get_positions("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );
        // All the following are acceptable
        let res = get_positions("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = get_positions("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = get_positions("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = get_positions("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = get_positions("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
        let res = get_positions("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
        let res = get_positions("1,7,3-5");
        assert!(res.is_ok());
        let res = get_positions("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);
        let res = get_positions("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}

pub fn parse_args() -> MyResult<Config> {
    let mut cmd = command!()
        .arg(arg!(<Path>).default_value("-"))
        .arg(arg!(-b --byte <Byte> "Select only these bytes"))
        .arg(arg!(-c --char <Char> "Select only these chars"))
        .arg(arg!(-f --field <Field> "Select only these fields").default_value(" "))
        .group(
            ArgGroup::new("Extract")
                .required(true)
                .args(["byte", "char", "field"]),
        )
        .arg(arg!(-d --delim <Delimeter> "Provide the delim char"));
    let matches = cmd.get_matches_mut();

    let delim = matches.get_one::<char>("delim"); // <-- gives Option<&char>, want Option<char>
    let delim = match delim {
        Some(c) => Some(c.to_owned()),
        None => None,
    };

    let extract = if let Some(fr) = matches.get_one::<String>("field") {
        match delim {
            None => ExtractCount::Fields(get_positions(fr).unwrap()),
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
            ExtractCount::Byte(get_positions(br).unwrap())
        } else if let Some(cr) = matches.get_one::<String>("char") {
            ExtractCount::Char(get_positions(cr).unwrap())
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
    println!("{:#?}", &cfg);

    Ok(())
}
