use std::{error::Error, fs};

use assert_cmd::Command;
use rand::{Rng, distributions::Alphanumeric};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "findr";

#[allow(dead_code)]
fn get_bad_files() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// find doesn't give such  error
fn gen_bad_file() -> String {
    loop {
        let name = get_bad_files();

        if fs::metadata(&name).is_err() {
            return name;
        }
    }
}

#[test]
fn dies_bad_type() -> TestResult {
    let expected = "invalid value";

    Command::cargo_bin(&PRG)?
        .args(&["-t", "x"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));

    Ok(())
}

fn dies_bad_name() -> TestResult {

    Ok(())

}
