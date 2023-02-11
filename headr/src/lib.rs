use std::{num::NonZeroUsize, fs::File, io::{BufReader, self, Read}};
#[allow(unused_imports)]
use std::{error::Error, io::BufRead};

use clap::{Parser, ArgGroup};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// argument parser for headr
#[derive(Debug, Parser)]
#[command(version)]
#[command(author = "someone")]
#[command(about = "rust clone of head")]
#[command(group(
        ArgGroup::new("count")
        .args(["bytes", "lines"])
        ))]
pub struct Config {
    /// provide filenames
    #[arg(action = clap::ArgAction::Append)]
    filenames: Vec<String>,
    /// print the first N bytes of file<s>
    #[arg(short = 'c', long)]
    bytes: Option<NonZeroUsize>,
    /// print first N lines of file<s>
    #[arg(short = 'n', long, default_value = "10")]
    lines: NonZeroUsize,
}

pub fn get_args() -> MyResult<Config> {
    let mut cli = Config::parse();

    if cli.filenames.is_empty() {
        cli.filenames = vec![String::from("-")];
    }

    /* if let Err(e) = parse_positive_int(cli.line_count) {
        Err(format!("illegal line count -- {}", cli.line_count))
    } */

    Ok(Config {
        filenames: cli.filenames,
        bytes: cli.bytes,
        lines: cli.lines
    })
}

#[allow(unused_variables)]
pub fn run(config: Config) -> MyResult<()> {

    for (filenum, filename) in config.filenames.iter().enumerate() {
        if config.filenames.len() > 1 {
            println!("{}==> {} <==", if 0 == filenum { "" } else { "\n" }, filename);
        }
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut buf) => read(&mut buf, &config)?,
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<BufReader<Box<dyn Read>>> {
    match filename {
        "-" => Ok(BufReader::new(Box::new(io::stdin()))),
        _ => Ok(BufReader::new(Box::new(File::open(&filename)?))),
    }
}

fn read(buf_read: &mut BufReader<Box<dyn Read>>, cfg: &Config) -> MyResult<()> {
    if let Some(byte_count) = cfg.bytes {
        /* let mut buff = vec![0; byte_count.into()];
        let bytes_read = buf_read.read(&mut buff[..])?; */
        // print!("{}", String::from_utf8_lossy(&buff[..bytes_read]));
        let read_bytes = buf_read.bytes().take(byte_count.into()).collect::<Result<Vec<_>, _>>();
        print!("{}", String::from_utf8_lossy(&read_bytes?[..]));
    } else {
        /* for line in buf_read.lines().take(cfg.lines.into()) {
            println!("{}", line?);
        } */
        let mut line = String::new();
        for _ in 0..cfg.lines.into() {
            let bytes = buf_read.read_line(&mut line)?;
            if 0 == bytes {
                break;
            }
            print!("{}", &line);
            line.clear();
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[allow(dead_code)]
fn parse_positive_int(num: &str) -> MyResult<usize> {
    match num.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(Into::<Box<dyn Error>>::into(num)),
        // _ => Err(Box::<dyn Error>::from(num)),
        // _ => Err(num.into()),
        // _ => Err(From::from(num)),
    }  // the syntax inside match is "match guard"
}

// unit test
#[test]
fn test_parse_positive_int() {
    // a positive integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(3, res.unwrap());

    // any string should give error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // a zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

/* fn open(filename: &String) -> MyResult<BufRead> {
} */
