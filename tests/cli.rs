use std::io::Write;
use std::process::{Command, Output, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_schema-lang"))
}

fn run_stdin(args: &[&str], input: &str) -> Output {
    let mut child = bin()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("schema-lang-{}-{}", std::process::id(), name));
    std::fs::write(&p, content).unwrap();
    p
}

#[test]
fn reads_a_file() {
    let p = temp_file("ok.sl", "dir src { require \"SCHEMA.md\" }");
    let out = bin().arg(&p).output().unwrap();
    std::fs::remove_file(&p).ok();
    assert_eq!(out.status.code(), Some(0));
    let so = text(&out.stdout);
    assert!(so.contains("Ident(\"dir\")"), "{so}");
    assert!(so.contains("Str(\"SCHEMA.md\")"), "{so}");
}

#[test]
fn reads_stdin() {
    let out = run_stdin(&["-"], "dir src {}");
    assert_eq!(out.status.code(), Some(0));
    assert!(text(&out.stdout).starts_with("Ident(\"dir\")"));
}

#[test]
fn help_goes_to_stdout() {
    for flag in ["--help", "-h"] {
        let out = bin().arg(flag).output().unwrap();
        assert_eq!(out.status.code(), Some(0));
        assert!(text(&out.stdout).contains("usage: schema-lang"));
    }
}

#[test]
fn version_prints_package_version() {
    let out = bin().arg("-V").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(text(&out.stdout).trim(), format!("schema-lang {}", env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_file_exits_1_naming_path() {
    let out = bin().arg("nonexistent.sl").output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let se = text(&out.stderr);
    assert!(se.contains("cannot read nonexistent.sl"), "{se}");
    assert!(se.to_lowercase().contains("no such file") || se.contains("os error"), "{se}");
}

#[test]
fn no_args_prints_usage_and_exits_2() {
    let out = bin().output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(text(&out.stderr).contains("usage: schema-lang"));
    assert!(out.stdout.is_empty());
}

#[test]
fn unknown_flag_exits_2() {
    let out = bin().arg("--bogus").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(text(&out.stderr).contains("--bogus"));
}

#[test]
fn lex_error_exits_1_with_position() {
    let out = run_stdin(&["-"], "dir src { require \"SCHEMA.md }");
    assert_eq!(out.status.code(), Some(1));
    assert!(text(&out.stderr).contains("1:19: unterminated string literal"));
}
