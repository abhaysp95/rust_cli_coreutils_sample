use std::{fs::{self, File}, io::Read};

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn test_dies_no_args() {
    let mut res = Command::cargo_bin("echor").unwrap();
    res.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_newline() {
    let mut res = Command::cargo_bin("echor").unwrap();
    res.args([ "this", "is" , "good" ])
        .assert()
        .success()
        .stdout("this is good\n");
}

#[test]
fn test_no_newline() {
    let mut res = Command::cargo_bin("echor").unwrap();
    res.args(["-n", "this", "is", "good"])
        .assert()
        .success()
        .stdout("this is good");
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut res = Command::cargo_bin("echor").unwrap();
    res.args(["Hello there"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello1n() {
    let mut file = File::open("tests/expected/hello1.n.txt").unwrap();
    let mut expected = String::new();
    file.read_to_string(&mut expected).unwrap();

    let mut res = Command::cargo_bin("echor").unwrap();
    res.args(["Hello there", "-n"])
        .assert()
        .success()
        .stdout(expected);
}

// the above tests hello1 and hello1n can also be written using run() helper function

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello2n() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
