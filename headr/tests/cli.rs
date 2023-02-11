use std::{error::Error, fs::{self, File}, io::Read};

use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{Rng, distributions::Alphanumeric};

const PRG: &str = "headr";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

type TestResult = Result<(), Box<dyn Error>>;

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
            return filename
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad_file = gen_bad_file();
    // let expected = format!("{}: .* [(]os error 2[)]", bad_file);
    let expected = format!("{}: No such file or directory", bad_file);
    Command::cargo_bin(PRG)?
        .args([EMPTY, &bad_file, ONE])
        .assert()
        .success()  // the command will succeed
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = get_random_string();
    let expected = format!("invalid value '{}'", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_lines() -> TestResult {
    let bad = get_random_string();
    let expected = format!("invalid value '{}'", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bytes_and_lines() -> TestResult {
    let expected = "the argument '--bytes <BYTES>' cannot be used with '--lines <LINES>'";
    Command::cargo_bin(PRG)?
        .args(&["-c", "10", "-n", "20"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

fn run(args: &[&str], filename: &str) -> TestResult {
    let mut file = File::open(filename)?;
    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;
    let expected = String::from_utf8_lossy(&buff);
    // let expected = fs::read_to_string(filename)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));
        // .stdout(expected);

    Ok(())
}

fn run_stdin(args: &[&str], input_filename: &str, expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;
    let expected = String::from_utf8_lossy(&buff);
    let input = fs::read_to_string(input_filename)?;

    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));
        // .stdout(buff);  // try something like this too, cause it's also a slice of u8

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn empty_n4() -> TestResult {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_c2() -> TestResult {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

#[test]
fn empty_c4() -> TestResult {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> TestResult {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_stdin() -> TestResult {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() -> TestResult {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> TestResult {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> TestResult {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> TestResult {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> TestResult {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn ten_stdin() -> TestResult {
    run_stdin(&[], TEN, "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TEN, "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TEN, "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TEN, "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TEN, "tests/expected/ten.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(&[EMPTY, ONE, TEN, THREE, TWO], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> TestResult {
    run(
        &[EMPTY, ONE, TEN, THREE, TWO, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> TestResult {
    run(
        &[EMPTY, ONE, TEN, THREE, TWO, "-n", "4"],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> TestResult {
    run(
        &[EMPTY, ONE, TEN, THREE, TWO, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> TestResult {
    run(
        &[EMPTY, ONE, TEN, THREE, TWO, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> TestResult {
    run(
        &["-c", "4", EMPTY, ONE, TEN, THREE, TWO],
        "tests/expected/all.c4.out",
    )
}
