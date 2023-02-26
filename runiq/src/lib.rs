use std::{
    cell::Cell,
    error::Error,
    fs::File,
    io::{self, stdout, BufRead, BufReader, BufWriter, Write},
};

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// simple clone for uniq coreutils in rust
pub struct Config {
    /// provide input file
    #[arg(default_value = "-")]
    in_file: String,
    /// provide filename to give output (else STDOUT is used)
    #[arg(long = "output", short)]
    out_file: Option<String>,
    /// prefix lines by the number of occurrences
    #[arg(short, long, action = ArgAction::SetTrue)]
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> MyResult<Config> {
    let mut cfg = Config::parse();

    if cfg.in_file.is_empty() {
        cfg.in_file = "-".to_string();
    }

    Ok(cfg)
}

pub fn run(cfg: Config) -> MyResult<()> {
    let file_name = cfg.in_file;
    let file_opened = open(&file_name).map_err(|e| format!("{}: {}", &file_name, e))?;
    // read_file(open(&file_name)?)?;
    let uniq_res = read_file(file_opened)?;

    if let Some(out_stream) = cfg.out_file {
        let new_file = File::create(out_stream)?;
        write_result(new_file, &uniq_res, cfg.count)?;
    } else {
        write_result(stdout(), &uniq_res, cfg.count)?;
    }

    Ok(())
}

fn write_result(
    write_to: impl Write,
    uniq_res: &Vec<CountLines>,
    with_count: bool,
) -> MyResult<()> {
    let mut new_buf = BufWriter::new(write_to);
    let mut buf_to_write: String;

    for cl in uniq_res {
        buf_to_write = if with_count {
            format!("{:>7} {}", cl.count.get(), cl.line)
        } else {
            format!("{}", cl.line).to_string()
        };

        if let Ok(n) = new_buf.write(buf_to_write.as_bytes()) {
            if n < buf_to_write.len() {
                panic!("Can't write buf to file");
            }
        }
    }

    if 0 != uniq_res.len() && !uniq_res[uniq_res.len() - 1].line.ends_with('\n') {
        new_buf.write(b"\n")?;
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(&filename)?))),
    }
}

struct CountLines {
    count: Cell<usize>,
    line: String,
}

fn read_file(mut buf_read: impl BufRead) -> MyResult<Vec<CountLines>> {
    let mut prev_line = String::new();

    let mut uniq_data: Vec<CountLines> = vec![];

    loop {
        let mut cur_line = String::new();
        if let Ok(n) = buf_read.read_line(&mut cur_line) {
            if n <= 0 {
                break;
            }
        }

        // let cur_line = cur_line.trim_end();
        if prev_line != cur_line.trim_end() {
            uniq_data.push(CountLines {
                count: Cell::new(1),
                line: cur_line.clone(),
            });
        } else {
            let lcount = &uniq_data[uniq_data.len() - 1].count;
            lcount.set(lcount.get() + 1);
        }

        prev_line = cur_line.trim_end().to_string();
    }

    Ok(uniq_data)
}

/* fn read_file(mut buf_read: impl BufRead) -> MyResult<()> {
    let mut my_buf = Vec::new();
    my_buf.resize(1024, 0);
    loop {
        if let Ok(n) = buf_read.read(&mut my_buf) {
            if n <= 0 {
                break;
            }
            println!("{}", String::from_utf8_lossy(&my_buf));
            my_buf.clear();
        }
    }

    Ok(())
} */
