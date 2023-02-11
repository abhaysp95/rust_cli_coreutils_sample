use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, about, version)]
#[command(author = "someone")]
#[command(about = "rust clone of gnu cat")]
pub struct Config {
    /// Provide filenames (default: stdin)
    #[arg(action = clap::ArgAction::Append)]
    files: Vec<String>,
    /// number all output lines
    #[arg(short = 'n')]  // short option must be unique
    number_lines: bool,
    /// number nonempty output lines
    #[arg(short = 'b')]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut count = 1;
    for filename in &config.files {
        /* if let Err(e) = open(&filename) {
            eprintln!("Failed to open '{}': {}", filename, e);
        } */

        match open(&filename) {
            Err(e) => eprintln!("Failed to open '{}': {}", filename, e),
            Ok(buf) => {
                if let Err(e) = read(buf, &config, &mut count) {
                    eprintln!("Can't read line for file '{}', {}", filename, e);
                }
            },
        }
    }

    Ok(())
}


pub fn get_args() -> MyResult<Config> {
    let mut cli = Config::parse();

    // check if there's better way to do this
    // one way is to do #[arg(default_values_t)], but I can't work it
    if cli.files.len() == 0 {
        cli.files = vec![String::from("-")];
    }

    if cli.number_lines && cli.number_nonblank_lines {
        eprintln!("The argument '--number-nonblank' can't be used with '--number'");
        std::process::exit(1)
    }

    Ok(cli)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read(buf_read: Box<dyn BufRead>, config: &Config, count: &mut u32) -> MyResult<()> {

    // something addition for demo
    /* for (line_num, line) in buf_read.lines().enumerate() {
        let line = line?;  // shadowing
        // ...
    } */

    for line in buf_read.lines() {
        match line {
            Ok(s) => {
                if config.number_nonblank_lines {
                    if s.is_empty() && *count != 0 {
                        *count -= 1;
                        println!("{:8}  {}", ' ', s);  // < left-aligned, ^ center-aligned, > right-aligned
                    } else {
                        println!("{:8}  {}", count, s);  // < left-aligned, ^ center-aligned, > right-aligned
                    }
                } else if config.number_lines {
                    println!("{:8}  {}", count, s);  // < left-aligned, ^ center-aligned, > right-aligned
                } else {
                    println!("{}", s);  // < left-aligned, ^ center-aligned, > right-aligned
                }
                *count += 1;
            }
            Err(_) => {
                eprintln!("Can't read file");
                break;
            }
        }
    }

    Ok(())
}
