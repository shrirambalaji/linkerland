use assert_cmd::Command;

#[test]
fn help_shows() {
    let mut cmd = Command::cargo_bin("linkerland").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn rejects_non_map_path() {
    let mut cmd = Command::cargo_bin("linkerland").unwrap();
    cmd.arg("foo.txt").assert().failure();
}

#[test]
fn export_parses() {
    let mut cmd = Command::cargo_bin("linkerland").unwrap();
    cmd.args(["export", "file.map", "--format", "json"])
        .assert()
        .failure();
}
