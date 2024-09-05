use crate::lexer::Token;

pub struct Expr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Expr {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Expr {
        Expr {
            left,
            operator,
            right,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
}
