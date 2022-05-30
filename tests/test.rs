use assert_cmd::Command;
use clap::crate_version;

fn genx() -> Command {
    let mut cmd = Command::cargo_bin("genx").unwrap();
    cmd.current_dir("tests/files");
    cmd
}

#[test]
fn test_version() {
    genx()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicates::str::contains(crate_version!()));
}

#[test]
fn test_extract() {
    genx().arg("extract").arg("-h").assert().success();
}
