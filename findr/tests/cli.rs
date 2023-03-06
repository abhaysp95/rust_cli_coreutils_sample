use std::{borrow::Cow, error::Error, fs};

use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

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

#[allow(dead_code)]
fn dies_bad_name() -> TestResult {
    Ok(())
}

#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<str> {
    expected_file.into()
}

#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    expected_file.into()
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected = contents.split("\n").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    expected.sort();

    let cmd = Command::cargo_bin(&PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines = stdout.split("\n").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    lines.sort();

    assert_eq!(expected, lines);

    Ok(())
}

// --------------------------------------------------
#[test]
fn path1() -> TestResult {
    run(&["tests/inputs"], "tests/expected/path1.txt")
}

// --------------------------------------------------
#[test]
fn path_a() -> TestResult {
    run(&["tests/inputs/a"], "tests/expected/path_a.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b() -> TestResult {
    run(&["tests/inputs/a/b"], "tests/expected/path_a_b.txt")
}

// --------------------------------------------------
#[test]
fn path_d() -> TestResult {
    run(&["tests/inputs/d"], "tests/expected/path_d.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d"],
        "tests/expected/path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f() -> TestResult {
    run(&["tests/inputs", "-t", "f"], "tests/expected/type_f.txt")
}

// --------------------------------------------------
#[test]
fn type_f_path_a() -> TestResult {
    run(
        &["tests/inputs/a", "-t", "f"],
        "tests/expected/type_f_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b() -> TestResult {
    run(
        &["tests/inputs/a/b", "--type", "f"],
        "tests/expected/type_f_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_d() -> TestResult {
    run(
        &["tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d() -> TestResult {
    run(&["tests/inputs", "-t", "d"], "tests/expected/type_d.txt")
}

// --------------------------------------------------
#[test]
fn type_d_path_a() -> TestResult {
    run(
        &["tests/inputs/a", "-t", "d"],
        "tests/expected/type_d_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b() -> TestResult {
    run(
        &["tests/inputs/a/b", "--type", "d"],
        "tests/expected/type_d_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_d() -> TestResult {
    run(
        &["tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_l() -> TestResult {
    run(&["tests/inputs", "-t", "l"], "tests/expected/type_l.txt")
}

// --------------------------------------------------
#[test]
fn type_f_l() -> TestResult {
    run(
        &["tests/inputs", "-t", "l", "f"],
        "tests/expected/type_f_l.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv() -> TestResult {
    run(
        &["tests/inputs", "-n", ".*[.]csv"],
        "tests/expected/name_csv.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> TestResult {
    run(
        &["tests/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
        "tests/expected/name_csv_mp3.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_txt_path_a_d() -> TestResult {
    run(
        &["tests/inputs/a", "tests/inputs/d", "--name", ".*.txt"],
        "tests/expected/name_txt_path_a_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_a() -> TestResult {
    run(&["tests/inputs", "-n", "a"], "tests/expected/name_a.txt")
}

// --------------------------------------------------
#[test]
fn type_f_name_a() -> TestResult {
    run(
        &["tests/inputs", "-t", "f", "-n", "a"],
        "tests/expected/type_f_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_name_a() -> TestResult {
    run(
        &["tests/inputs", "--type", "d", "--name", "a"],
        "tests/expected/type_d_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_g() -> TestResult {
    run(&["tests/inputs/g.csv"], "tests/expected/path_g.txt")
}
