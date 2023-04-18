use crate::ast::*;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();

        while let Some(token) = {
            self.skip_trivia();
            self.tokens.peek()
        } {
            if token.kind == TokenKind::Actor {
                stmts.push(self.actor_def()?);
            } else {
                return Err(format!("Unexpected token: {:?}", token));
            }
        }

        Ok(stmts)
    }

    fn actor_def(&mut self) -> Result<Stmt, String> {
        self.expect_token(TokenKind::Actor)?;

        let name = self.expect_token(TokenKind::Ident)?;
        self.expect_token(TokenKind::LCurlyBrace)?;

        let mut message_handlers = Vec::new();

        while self.peek_token(TokenKind::On) {
            message_handlers.push(self.message_handler()?);
        }

        self.expect_token(TokenKind::RCurlyBrace)?;

        Ok(Stmt::ActorDef {
            name,
            message_handlers,
        })
    }

    fn message_handler(&mut self) -> Result<MessageHandler, String> {
        let on = self.expect_token(TokenKind::On)?;

        self.expect_token(TokenKind::Ident)?;
        self.expect_token(TokenKind::LCurlyBrace)?;

        let mut body = Vec::new();

        while !self.peek_token(TokenKind::RCurlyBrace) {
            body.push(self.stmt()?);
        }

        self.expect_token(TokenKind::RCurlyBrace)?;

        Ok(MessageHandler { on, body })
    }

    fn stmt(&mut self) -> Result<Stmt, String> {
        self.skip_trivia();

        match self.tokens.peek().map(|t| t.kind) {
            Some(TokenKind::Print) => {
                self.tokens.next();
                self.skip_trivia();
                let expr = self.expr()?;
                Ok(Stmt::Print(expr))
            }
            Some(TokenKind::Die) => Ok(Stmt::Die),
            _ => Err(format!("Unexpected token: {:?}", self.tokens.peek())),
        }
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let token = self.tokens.next().ok_or("Expected expression")?;

        let literal = match token.kind {
            TokenKind::String => Literal::String(self.parse_string_literal(&token.text)?),
            _ => return Err(format!("Unexpected token: {:?}", token)),
        };

        Ok(Expr::Literal(literal))
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<Token, String> {
        self.skip_trivia();

        let token = self.tokens.next().ok_or(format!("Expected {:?}", kind))?;

        if token.kind == kind {
            Ok(token.clone())
        } else {
            Err(format!("Expected {:?}, found {:?}", kind, token))
        }
    }

    fn peek_token(&mut self, kind: TokenKind) -> bool {
        self.skip_trivia();

        self.tokens.peek().map(|t| t.kind) == Some(kind)
    }

    fn skip_trivia(&mut self) {
        while self
            .tokens
            .peek()
            .map(|t| t.kind.is_trivia())
            .unwrap_or(false)
        {
            self.tokens.next();
        }
    }

    fn parse_string_literal(&self, text: &str) -> Result<String, String> {
        let mut chars = text.chars().peekable();
        let mut result = String::new();

        if chars.peek() != Some(&'"') {
            return Err("Invalid string literal".to_string());
        }
        chars.next(); // Consume the opening quote

        while let Some(&c) = chars.peek() {
            chars.next();

            if c == '"' {
                break;
            } else if c == '\\' {
                let next = chars.next().ok_or("Invalid escape sequence")?;
                let escaped = match next {
                    '"' => '"',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    _ => return Err(format!("Invalid escape sequence: \\{}", next)),
                };
                result.push(escaped);
            } else {
                result.push(c);
            }
        }

        if chars.peek() != None {
            return Err("Invalid string literal".to_string());
        }

        Ok(result)
    }
}
