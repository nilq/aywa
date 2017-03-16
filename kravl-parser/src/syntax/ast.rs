use syntax::tokens::{
    Token,
    TokenType,
};

use syntax::lexer::Lexer;

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    If(Box<Expression>, Box<Statement>),
    IfElse(Box<Expression>, Box<Statement>, Box<Statement>),
    Variable(String, Box<Expression>),
    Block(Box<Vec<Statement>>),
    Expression(Box<Expression>),
    Pass,
    Return(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Text(String),
    Call(String, Box<Vec<Expression>>),
    Assignment(Box<Expression>, Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    Index(String, Box<Expression>),
    Array(Box<Vec<Expression>>),
    True,
    False,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
        }
    }

    fn parse_current(&mut self) -> Result<Expression, String> {
        match self.lexer.current_token().token_type {
            TokenType::Integer => {
                Ok(Expression::Integer(
                    self.lexer.current_token_content().parse::<i64>().unwrap()
                ))
            },

            TokenType::Float => {
                Ok(Expression::Float(
                    self.lexer.current_token_content().parse::<f64>().unwrap()
                ))
            },

            TokenType::Text => {
                Ok(Expression::Text(
                    self.lexer.current_token_content()
                ))
            },

            TokenType::True => {
                Ok(Expression::True)
            },

            TokenType::False => {
                Ok(Expression::False)
            },

            _ => {
                Ok(Expression::False)
            }
        }
    }
}