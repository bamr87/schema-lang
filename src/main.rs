mod lexer;

fn main() {
    let src = std::env::args().nth(1).unwrap_or_else(|| "dir src { require \"SCHEMA.md\" }".into());
    for t in lexer::lex(&src) {
        println!("{:?}", t);
    }
}
