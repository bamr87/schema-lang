mod lexer;

fn main() {
    let src = std::env::args().nth(1).unwrap_or_else(|| "dir src { require \"SCHEMA.md\" }".into());
    match lexer::lex(&src) {
        Ok(tokens) => {
            for t in tokens {
                println!("{:?}", t.tok);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
