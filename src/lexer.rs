#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Str(String),
    Punct(char),
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut it = src.chars().peekable();
    while let Some(&c) = it.peek() {
        if c.is_whitespace() {
            it.next();
        } else if c == '"' {
            it.next();
            let s: String = it.by_ref().take_while(|&ch| ch != '"').collect();
            out.push(Token::Str(s));
        } else if c.is_alphanumeric() || c == '_' {
            let mut s = String::new();
            while let Some(&ch) = it.peek() {
                if ch.is_alphanumeric() || ch == '_' { s.push(ch); it.next(); } else { break; }
            }
            out.push(Token::Ident(s));
        } else {
            out.push(Token::Punct(c));
            it.next();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexes_block() {
        let t = lex("dir src { require \"SCHEMA.md\" }");
        assert_eq!(t[0], Token::Ident("dir".into()));
        assert_eq!(t[2], Token::Punct('{'));
        assert_eq!(t[4], Token::Str("SCHEMA.md".into()));
    }
}
