use std::{error::Error, ops::Range, fmt::Error};

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


fn parse_ranges(input: &str) -> MyResult<Range<usize>> {
    let input = input.trim();
    let valueError = format!("illegal list value: \"{}\"", input);
    if input.chars().any(|c| !c.is_numeric()) {
        return Err(valueError);
    }

    Ok(Range{
    })
}

fn get_positions(input: &str) -> MyResult<Vec<Range<usize>>> {
    let mut vec = vec![];

    Ok(vec)
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
