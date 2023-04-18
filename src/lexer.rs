use logos::Logos;

use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?.expect("Failed to lex");

        let text = self.inner.slice();
        let span = self.inner.span();

        Some(Self::Item {
            kind,
            text: text.into(),
            span,
        })
    }
}
