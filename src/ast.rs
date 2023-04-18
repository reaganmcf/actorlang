use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ActorDef {
        name: Token,
        message_handlers: Vec<MessageHandler>,
    },
    Die,
    Block(Vec<Stmt>),
    Expr(Expr),
    Print(Expr),
    Assign {
        name: Token,
        value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHandler {
    pub on: Token,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
}
