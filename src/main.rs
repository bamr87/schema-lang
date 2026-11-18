mod lexer;

use std::io::Read;
use std::process::exit;

const USAGE: &str = "\
usage: schema-lang [OPTIONS] <FILE | ->

Lex a SCHEMA blueprint and print one token per line.

  <FILE>        read the blueprint from FILE
  -             read the blueprint from standard input

options:
  -h, --help    print this help and exit
  -V, --version print the version and exit

exit codes: 0 success, 1 unreadable input or lex error, 2 usage error";

fn main() {
    let mut args = std::env::args().skip(1);
    let first = match args.next() {
        Some(a) => a,
        None => {
            eprintln!("{}", USAGE);
            exit(2);
        }
    };
    match first.as_str() {
        "-h" | "--help" => {
            println!("{}", USAGE);
            return;
        }
        "-V" | "--version" => {
            println!("schema-lang {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        "-" => {}
        s if s.starts_with('-') => {
            eprintln!("schema-lang: unknown option {}\n\n{}", s, USAGE);
            exit(2);
        }
        _ => {}
    }
    if let Some(extra) = args.next() {
        eprintln!("schema-lang: unexpected argument {}\n\n{}", extra, USAGE);
        exit(2);
    }

    let src = if first == "-" {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
            eprintln!("schema-lang: cannot read stdin: {}", e);
            exit(1);
        }
        buf
    } else {
        match std::fs::read_to_string(&first) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("schema-lang: cannot read {}: {}", first, e);
                exit(1);
            }
        }
    };

    match lexer::lex(&src) {
        Ok(tokens) => {
            for t in tokens {
                println!("{:?}", t.tok);
            }
        }
        Err(e) => {
            eprintln!("schema-lang: {}", e);
            exit(1);
        }
    }
}
