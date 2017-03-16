use syntax::tokens::{
    TokenType,
    BinOp,
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
    Bool(bool),
    Call(String, Box<Vec<Expression>>),
    Assignment(Box<Expression>, Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    Index(String, Box<Expression>),
    Array(Box<Vec<Expression>>),
    Identifier(String),
    Operation(Box<Expression>, BinOp, Box<Expression>),
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

    pub fn from(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
        }
    }

    fn parse_bin_op(&mut self, expr: Expression) -> Result<Expression, String> {
        let mut expr_list = vec!(expr);
        let mut oper_list: Vec<(BinOp, u8)> = Vec::new();

        oper_list.push(Lexer::bin_op(&self.lexer.current_token_content()[..]).unwrap());

        self.lexer.next_token();

        expr_list.push(try!(self.parse_word()));

        let mut done = false;

        while expr_list.len() > 1 {

            let left = expr_list.pop().unwrap();
            let right = expr_list.pop().unwrap();

            let oper = Expression::Operation(
                Box::new(left),
                oper_list.pop().unwrap().0,
                Box::new(right),
            );


            if !done && self.lexer.next_token() {
                if self.lexer.current_token().token_type != TokenType::BinOp {
                    self.lexer.previous_token();
                    done = false;

                    continue
                }

                let (op, prec) = Lexer::bin_op(&self.lexer.current_token_content()[..]).unwrap();

                if prec > oper_list.last().unwrap().1 {
                    expr_list.push(oper);

                    self.lexer.next_token();

                    expr_list.push(try!(self.parse_word()));
                    oper_list.push((op, prec));

                    continue
                }
                self.lexer.next_token();

                expr_list.push(try!(self.parse_word()));
                oper_list.push((op, prec));
            }

            expr_list.push(oper);
        }

        Ok(expr_list.pop().unwrap())
    }

    fn parse_word(&mut self) -> Result<Expression, String> {
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
                Ok(Expression::Bool(true))
            },

            TokenType::False => {
                Ok(Expression::Bool(false))
            },

            TokenType::Identifier => {
                let id = Expression::Identifier(self.lexer.current_token_content());

                if self.lexer.next_token() {
                    return match self.lexer.current_token().token_type {
                        TokenType::BinOp => {
                            self.parse_bin_op(id)
                        },
                        _ => {
                            self.lexer.previous_token();
                            Ok(id)
                        }
                    }
                }

                Ok(id)
            },

            _ => {
                Ok(Expression::Bool(false))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let expr = try!(self.parse_word());

        self.lexer.next_token();

        if self.lexer.tokens_remaining() > 0 {
            if self.lexer.current_token().token_type == TokenType::BinOp {
                return self.parse_bin_op(expr);
            }

            self.lexer.previous_token();
        }

        Ok(expr)
    }
}