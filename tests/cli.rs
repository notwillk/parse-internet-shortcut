use assert_cmd::Command;

#[test]
fn parses_basic_fixture() {
    let mut cmd = Command::cargo_bin("parse-internet-shortcut").expect("binary exists");
    cmd.arg("tests/fixtures/basic.url")
        .assert()
        .success()
        .stdout(
            "{\n  \"sections\": {\n    \"InternetShortcut\": {\n      \"IconFile\": \"https://example.com/favicon.ico\",\n      \"IconIndex\": \"0\",\n      \"URL\": \"https://example.com\"\n    }\n  }\n}\n",
        );
}

#[test]
fn handles_comments_blank_lines_whitespace_unknowns() {
    let mut cmd = Command::cargo_bin("parse-internet-shortcut").expect("binary exists");
    cmd.arg("tests/fixtures/comments.url").assert().success().stdout(
        "{\n  \"sections\": {\n    \"Extra\": {\n      \"Other\": \"unknown\"\n    },\n    \"InternetShortcut\": {\n      \"URL\": \"https://example.com\"\n    }\n  }\n}\n",
    );
}

#[test]
fn reads_from_stdin() {
    let mut cmd = Command::cargo_bin("parse-internet-shortcut").expect("binary exists");
    cmd.arg("-")
        .write_stdin(include_str!("fixtures/stdin.url"))
        .assert()
        .success()
        .stdout(
            "{\n  \"sections\": {\n    \"\": {\n      \"URL\": \"https://example.com/from-stdin\"\n    }\n  }\n}\n",
        );
}

#[test]
fn returns_parse_error_for_malformed_line() {
    let mut cmd = Command::cargo_bin("parse-internet-shortcut").expect("binary exists");
    cmd.arg("tests/fixtures/malformed.url")
        .assert()
        .code(3)
        .stderr("error: malformed line 2: expected key=value or [section]\n");
}

#[test]
fn returns_usage_error_without_input() {
    let mut cmd = Command::cargo_bin("parse-internet-shortcut").expect("binary exists");
    cmd.assert().code(1).stderr("error: missing input path\n");
}
