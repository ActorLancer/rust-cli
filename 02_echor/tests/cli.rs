use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::{fs, process::Output};

// ------------------------------------------------------
// 确保当用户没有输入任何参数时，程序会报错并输出帮助信息（包含 "Usage" 字样）
#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        // 断案标准错误输出（stderr）中包含 “Usage" 字样。通常 clap 在参数错误时会输出 usage 帮助信息
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}
// ------------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected: String = fs::read_to_string(expected_file)?;
    let output: Output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout: String = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}
// ------------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
