use logos::{Logos, Span};

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    #[regex(r"[\s\t\n\f]+")]
    Whitespace,
    #[regex(r"//.*")]
    Comment,

    // Keywords
    #[token("actor")]
    Actor,
    #[token("let")]
    Let,
    #[token("on")]
    On,
    #[token("print")]
    Print,
    #[token("die")]
    Die,

    // Literals
    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    Ident,
    #[regex(r#""[^"]*""#)]
    String,
    #[regex("[0-9]+")]
    Int,
    #[regex(r#"[0-9]+\.[0-9]+"#)]
    Float,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Single character tokens
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurlyBrace,
    #[token("}")]
    RCurlyBrace,
    #[token("=")]
    Equals,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span
}
