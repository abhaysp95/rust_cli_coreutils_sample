use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{ArgAction, ArgGroup, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author = "someone", version = "1.0.0")]
#[command(about = "wc clone for rust")]
#[command(group(ArgGroup::new("bc_count").args(["chars", "bytes"])))]
/// minimal wc clone for rust
pub struct ArgConfig {
    /// Provide filename(s)
    #[arg(action = ArgAction::Append)]
    files: Vec<String>,
    /// print the newline counts
    #[arg(short, long)]
    lines: bool,
    /// print the character counts
    #[arg(short = 'm', long)]
    chars: bool,
    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,
    /// print the word counts
    #[arg(short, long)]
    words: bool,
}

pub fn get_args() -> MyResult<ArgConfig> {
    let mut cfg = ArgConfig::parse();

    if cfg.files.is_empty() {
        cfg.files = vec!["-".to_string()];
    }

    if [cfg.lines, cfg.words, cfg.bytes, cfg.chars]
        .iter()
        .all(|a| false == *a)
    {
        cfg.lines = true;
        cfg.words = true;
        cfg.bytes = true;
    }

    Ok(cfg)
}

pub fn run(cfg: ArgConfig) -> MyResult<()> {
    let mut total_file_info = FileInfo::new();
    let mut file_infos: Vec<FileInfo> = Vec::new();

    for filename in &cfg.files {
        match open(filename) {
            Err(e) => eprintln!("can't open file {}: {}", &filename, e),
            Ok(buf) => {
                let file_info = count(buf)?;
                total_file_info.line_count += file_info.line_count;
                total_file_info.word_count += file_info.word_count;
                total_file_info.char_count += file_info.char_count;
                total_file_info.byte_count += file_info.byte_count;
                file_infos.push(file_info);
            }
        }
    }

    /* now get the pad and print the file infos */
    let pad = get_pad(total_file_info.byte_count);
    for (idx, file_info) in file_infos.iter().enumerate() {
        print_fileinfo(file_info, &cfg.files[idx], &cfg, pad);
    }

    if 1 < file_infos.len() {
        print_fileinfo(&total_file_info, "total", &cfg, pad);
    }

    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(Configfilename)?))),
    }
}

/* pub fn open(filename: &str) -> MyResult<BufReader<Box<dyn Read>>> {

    match filename {
        "-" => Ok(BufReader::new(Box::new(io::stdin()))),
        _ => Ok(BufReader::new(Box::new(File::open(filename)?))),
    }

} */

#[derive(Debug, PartialEq)]
struct FileInfo {
    line_count: usize,
    word_count: usize,
    char_count: usize,
    byte_count: usize,
}

impl FileInfo {
    fn new() -> Self {
        Self {
            line_count: 0,
            word_count: 0,
            char_count: 0,
            byte_count: 0,
        }
    }
}

/**
 * "mut stream: &TcpStream"  -> stream is mutable, meaning to what TcpStream, stream should point can be
 * changed, but to what TcpStream, stream points to can't be changed
 * stream = (some other TcpStream)  // ok
 * *stream = 10  // not ok
 *
 * "stream: &mut TcpStream" -> to what TcpStream, stream points to can be changed, but stream can't
 * point to any other TcpStream
 */

// pub fn count(buf: &mut Box<dyn BufRead>) {
fn count(mut buf: impl BufRead) -> MyResult<FileInfo> {
    let mut file_info = FileInfo::new();
    let mut line_buf = String::new();

    // while let Ok(line) = buf.read_line(&mut line_buf) {
    loop {
        // shouldn't use while, because at EOF also, it returns Ok(0), resulting in infinite
        // loop
        let bread = buf.read_line(&mut line_buf)?;
        if 0 == bread {
            break;
        }
        file_info.byte_count += bread;
        file_info.line_count += 1;
        file_info.word_count += line_buf.split_whitespace().count();
        file_info.char_count += line_buf.chars().count();
        line_buf.clear();
    }

    Ok(file_info)
}

fn print_fileinfo(fileinfo: &FileInfo, filename: &str, cfg: &ArgConfig, pad: usize) {
    // println!("{:>8} {:>8} {:>8} {}", fileinfo.line_count, fileinfo.word_count, fileinfo.byte_count, filename);

    println!(
        "{}{}{}{}{}",
        format_field(&fileinfo.line_count, pad - 1, cfg.lines),
        format_field(&fileinfo.word_count, pad, cfg.words),
        format_field(&fileinfo.char_count, pad, cfg.chars),
        format_field(&fileinfo.byte_count, pad, cfg.bytes),
        if "-" == filename {
            "".to_string()
        } else {
            format!(" {}", filename)
        }
    );
    // res += format_field(&fileinfo.line_count, &pad, cfg.lines);
    // println!("{:>pad$} {:>pad$} {:>pad$} {}", fileinfo.line_count, fileinfo.word_count, fileinfo.byte_count, filename);
}

fn format_field(value: &usize, pad: usize, show: bool) -> String {
    if show {
        format!("{:pad$}", value)
    } else {
        "".to_string()
    }
}

fn get_pad(bc: usize) -> usize {
    if 0 == bc {
        2
    } else {
        let mut temp = bc;
        let mut pad = 0;
        while temp > 0 {
            temp /= 10;
            pad += 1;
        }
        pad + 1
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, get_pad, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok()); // cursor create successfully

        let expected = FileInfo {
            line_count: 1,
            word_count: 10,
            char_count: 48,
            byte_count: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(&1, 1, false), "");
        assert_eq!(format_field(&101, 4, true), " 101");
        assert_eq!(format_field(&10, 5, true), "   10");
    }

    #[test]
    fn test_get_pad() {
        assert_eq!(get_pad(100), 4);
        assert_eq!(get_pad(0), 2);
        assert_eq!(get_pad(2), 2);
        assert_eq!(get_pad(20), 3);
    }
}

/* pub fn get_line_count(buf_reader: &mut BufReader<Box<dyn Read>>) -> MyResult<usize> {
    Ok(buf_reader.lines().count())
}

pub fn get_byte_count(buf_reader: &mut BufReader<Box<dyn Read>>) -> MyResult<usize> {
    Ok(buf_reader.bytes().count())
}

pub fn get_word_count(buf_reader: &mut BufReader<Box<dyn Read>>) -> MyResult<usize> {
    // TODO: split over any whitespace char (multiple spaces, tab char etc.)
    Ok(buf_reader.split(b' ').count())
} */
