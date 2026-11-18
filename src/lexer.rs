use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Str(String),
    Punct(char),
}

/// A 1-based source position (`col` counts characters, not bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

/// A token together with where it starts in the source.
#[derive(Debug, PartialEq)]
pub struct Spanned {
    pub tok: Token,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexErrorKind {
    /// A string literal that reaches end of input without a closing quote.
    UnterminatedString,
    /// A backslash followed by a character other than `"` or `\`.
    InvalidEscape(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: ", self.span.line, self.span.col)?;
        match &self.kind {
            LexErrorKind::UnterminatedString => write!(f, "unterminated string literal"),
            LexErrorKind::InvalidEscape(c) => write!(f, "invalid escape sequence '\\{}'", c),
        }
    }
}

impl std::error::Error for LexError {}

pub fn lex(src: &str) -> Result<Vec<Spanned>, LexError> {
    let mut out = Vec::new();
    let mut it = src.chars().peekable();
    let (mut line, mut col) = (1usize, 1usize);

    // Consume one char, keeping the position in step.
    macro_rules! bump {
        () => {{
            let ch = it.next();
            if ch == Some('\n') {
                line += 1;
                col = 1;
            } else if ch.is_some() {
                col += 1;
            }
            ch
        }};
    }

    while let Some(&c) = it.peek() {
        let span = Span { line, col };
        if c.is_whitespace() {
            bump!();
        } else if c == '"' {
            bump!();
            let mut s = String::new();
            loop {
                let esc_span = Span { line, col };
                match bump!() {
                    None => {
                        return Err(LexError { kind: LexErrorKind::UnterminatedString, span });
                    }
                    Some('"') => break,
                    Some('\\') => match bump!() {
                        Some(e @ ('"' | '\\')) => s.push(e),
                        Some(e) => {
                            return Err(LexError { kind: LexErrorKind::InvalidEscape(e), span: esc_span });
                        }
                        None => {
                            return Err(LexError { kind: LexErrorKind::UnterminatedString, span });
                        }
                    },
                    Some(ch) => s.push(ch),
                }
            }
            out.push(Spanned { tok: Token::Str(s), span });
        } else if c.is_alphanumeric() || c == '_' {
            let mut s = String::new();
            while let Some(&ch) = it.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    s.push(ch);
                    bump!();
                } else {
                    break;
                }
            }
            out.push(Spanned { tok: Token::Ident(s), span });
        } else {
            out.push(Spanned { tok: Token::Punct(c), span });
            bump!();
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_block() {
        let t = lex("dir src { require \"SCHEMA.md\" }").unwrap();
        assert_eq!(t[0].tok, Token::Ident("dir".into()));
        assert_eq!(t[2].tok, Token::Punct('{'));
        assert_eq!(t[4].tok, Token::Str("SCHEMA.md".into()));
    }

    #[test]
    fn unterminated_string_points_at_opening_quote() {
        let e = lex("require \"abc").unwrap_err();
        assert_eq!(e.kind, LexErrorKind::UnterminatedString);
        assert_eq!((e.span.line, e.span.col), (1, 9));
        assert_eq!(e.to_string(), "1:9: unterminated string literal");
    }

    #[test]
    fn closing_brace_is_not_swallowed() {
        let e = lex("dir src { require \"SCHEMA.md }").unwrap_err();
        assert_eq!(e.to_string(), "1:19: unterminated string literal");
    }

    #[test]
    fn spans_track_lines_and_columns() {
        let t = lex("a\n  b").unwrap();
        assert_eq!(t[0].span, Span { line: 1, col: 1 });
        assert_eq!((t[1].span.line, t[1].span.col), (2, 3));
    }

    #[test]
    fn escapes_in_strings() {
        let t = lex("\"a\\\"b\\\\c\"").unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].tok, Token::Str("a\"b\\c".into()));
    }

    #[test]
    fn invalid_escape_is_an_error() {
        let e = lex("\"a\\qb\"").unwrap_err();
        assert_eq!(e.kind, LexErrorKind::InvalidEscape('q'));
        assert_eq!((e.span.line, e.span.col), (1, 3));
    }

    #[test]
    fn trailing_backslash_is_unterminated() {
        let e = lex("\"abc\\").unwrap_err();
        assert_eq!(e.kind, LexErrorKind::UnterminatedString);
    }

    #[test]
    fn multiline_string_advances_line() {
        let t = lex("\"x\ny\" z").unwrap();
        assert_eq!(t[0].tok, Token::Str("x\ny".into()));
        assert_eq!(t[1].span, Span { line: 2, col: 4 });
    }
}
