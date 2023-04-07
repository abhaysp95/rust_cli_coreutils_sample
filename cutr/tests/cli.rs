use std::fs;

use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "cutr";
const BOOKS: &str = "tests/inputs/books.tsv";
const CSV: &str = "tests/inputs/movies1.csv";
const TSV: &str = "tests/inputs/movies1.tsv";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad_file = gen_bad_file();
    let expected = format!(".* (os error 2).*");

    Command::cargo_bin(&PRG)?
        .args(&["-b", "1", CSV, &bad_file])
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}