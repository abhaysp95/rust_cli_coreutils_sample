use std::{fs, error::Error};

use assert_cmd::Command;
use rand::{Rng, distributions::Alphanumeric};

type TestResult = Result<(), Box<dyn Error>>;

const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

const PRG: &str = "wcr";

fn get_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn gen_bad_file() -> String {
    loop {
        let filename = get_random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn dies_char_and_bytes() -> TestResult {
    let expected = "the argument '--chars' cannot be used with '--bytes'";

    Command::cargo_bin(PRG)?
        .args(&["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));

    Ok(())
}

// function with arguments can't be used for test
fn run(args: &[&str], filename: &str) -> TestResult {
    let expected = fs::read_to_string(filename)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::is_match(expected)?);

    Ok(())
}

fn run_stdin(args: &[&str], input_file: &str, expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::is_match(expected)?);

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    // TODO: figure out the glob pattern (cases: file not found, no permission on file)
    let expected = format!("{}: No such file or directory", &bad);
    Command::cargo_bin(PRG)?
        .args(&[EMPTY, &bad, FOX])
        .assert()
        .success()
        .stderr(predicates::str::contains(expected));

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_bytes() -> TestResult {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

#[test]
fn fox_chars() -> TestResult {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

#[test]
fn fox_words() -> TestResult {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

#[test]
fn fox_lines() -> TestResult {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

#[test]
fn fox_words_bytes() -> TestResult {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

#[test]
fn fox_words_lines() -> TestResult {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

#[test]
fn fox_bytes_lines() -> TestResult {
    run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
}

#[test]
fn atlamal() -> TestResult {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

#[test]
fn atlamal_bytes() -> TestResult {
    run(&["-c", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

#[test]
fn atlamal_words() -> TestResult {
    run(&["-w", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

#[test]
fn atlamal_lines() -> TestResult {
    run(&["-l", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

#[test]
fn atlamal_words_bytes() -> TestResult {
    run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

#[test]
fn atlamal_words_lines() -> TestResult {
    run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

#[test]
fn atlamal_bytes_lines() -> TestResult {
    run(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

#[test]
fn atlamal_stdin() -> TestResult {
    run_stdin(&[], ATLAMAL, "tests/expected/atlamal.txt.stdin.out")
}

#[test]
fn test_all() -> TestResult {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

#[test]
fn test_all_lines() -> TestResult {
    run(&["-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
}

#[test]
fn test_all_words() -> TestResult {
    run(&["-w", EMPTY, FOX, ATLAMAL], "tests/expected/all.w.out")
}

#[test]
fn test_all_bytes() -> TestResult {
    run(&["-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
}

#[test]
fn test_all_words_bytes() -> TestResult {
    run(&["-cw", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
}

#[test]
fn test_all_words_lines() -> TestResult {
    run(&["-wl", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
}

#[test]
fn test_all_bytes_lines() -> TestResult {
    run(&["-cl", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
}
