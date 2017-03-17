use syntax::tokens::{
    TokenType,
    Token,
    BinOp,
};

use syntax::lexer::Lexer;

#[derive(Debug, Clone)]
pub enum Statement {
    If(Box<Expression>, Box<Statement>),
    IfElse(Box<Expression>, Box<Statement>, Box<Statement>),
    Variable(String, Box<Expression>),
    Block(Box<Vec<Statement>>),
    Expression(Box<Expression>),
    Assignment(String, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Call(Box<Expression>, Box<Vec<Expression>>),
    Assignment(Box<Expression>, Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    Index(String, Box<Expression>),
    Array(Box<Vec<Expression>>),
    Identifier(String),
    Operation(Box<Expression>, BinOp, Box<Expression>),
    Definition(Option<String>, Box<Vec<String>>, Box<Vec<Statement>>, Option<String>),
    Return(Box<Expression>),
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

    pub fn parse_from_tokens(tokens: Vec<Token>) -> Result<Vec<Statement>, String> {
        let mut parser = Parser::from(Lexer::from(tokens));

        parser.parse_full()
    }

    fn parse_bin_op(&mut self, expr: Expression) -> Result<Expression, String> {
        let mut expr_list = vec!(expr);
        let mut oper_list: Vec<(BinOp, u8)> = Vec::new();

        oper_list.push(Lexer::bin_op(
            &self.lexer.current_token_content()[..]
        ).unwrap());

        self.lexer.next_token();

        expr_list.push(try!(self.parse_word()));

        let mut done = false;

        while expr_list.len() > 1 {

            if !done && self.lexer.next_token() {
                if self.lexer.current_token().token_type != TokenType::BinOp {
                    self.lexer.previous_token();
                    done = true;

                    continue
                }

                let (op, prec) = Lexer::bin_op(&self.lexer.current_token_content()[..]).unwrap();

                if prec > oper_list.last().unwrap().1 {

                    let left = expr_list.pop().unwrap();
                    let right = expr_list.pop().unwrap();

                    expr_list.push(Expression::Operation(
                        Box::new(left),
                        oper_list.pop().unwrap().0,
                        Box::new(right),
                    ));

                    self.lexer.next_token();

                    expr_list.push(try!(self.parse_word()));
                    oper_list.push((op, prec));

                    continue
                }
                self.lexer.next_token();

                expr_list.push(try!(self.parse_word()));
                oper_list.push((op, prec));
            }

            let left = expr_list.pop().unwrap();
            let right = expr_list.pop().unwrap();

            let oper = Expression::Operation(
                Box::new(left),
                oper_list.pop().unwrap().0,
                Box::new(right),
            );

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

            TokenType::LParen => {
                self.lexer.next_token();

                let expr = try!(self.parse_expression());

                try!(self.lexer.match_current_token(TokenType::RParen));

                self.lexer.next_token();

                if self.lexer.current_token().token_type == TokenType::LParen {
                    return self.parse_caller(expr)
                }

                self.lexer.previous_token();

                Ok(expr)
            },

            TokenType::Identifier => {
                let id = Expression::Identifier(self.lexer.current_token_content());

                if self.lexer.next_token() {
                    return match self.lexer.current_token().token_type {
                        TokenType::BinOp => {
                            self.parse_bin_op(id)
                        },
                        
                        TokenType::LParen => {
                            self.parse_caller(id)
                        },
                        
                        _ => {
                            self.lexer.previous_token();
                            Ok(id)
                        }
                    }
                }

                Ok(id)
            },

            TokenType::Definition => {
                self.lexer.next_token();

                let name: Option<String>;

                if self.lexer.current_token().token_type == TokenType::Identifier {
                    name = Some(self.lexer.current_token_content());
                    self.lexer.next_token();
                } else {
                    name = None;
                }

                try!(self.lexer.match_current_token(TokenType::LParen));

                self.lexer.next_token();

                let mut arg_stack = Vec::new();

                while self.lexer.current_token().token_type == TokenType::Identifier {
                    arg_stack.push(self.lexer.current_token_content());
                    
                    self.lexer.next_token();

                    if self.lexer.current_token().token_type == TokenType::Comma {
                        self.lexer.next_token();
                    }
                }

                try!(self.lexer.match_current_token(TokenType::RParen));

                self.lexer.next_token();

                let ret_type: Option<String>;

                if self.lexer.current_token().token_type == TokenType::Arrow {
                    self.lexer.next_token();

                    if self.lexer.current_token().token_type == TokenType::Identifier {
                        ret_type = Some(self.lexer.current_token_content());
                    } else {
                        ret_type = None;
                    }

                } else {
                    ret_type = None;
                }

                self.lexer.next_token();

                let block_body = try!(self.parse_block());

                Ok(Expression::Definition(
                    name,
                    Box::new(arg_stack),
                    Box::new(block_body),
                    ret_type,
                ))
            },

            TokenType::Return => {
                self.lexer.next_token();

                let expr = try!(self.parse_expression());

                Ok(Expression::Return(Box::new(expr)))
            },

            _ => {
                Err(String::from("fucked expression"))
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.lexer.current_token().token_type {
            TokenType::Identifier => {
                let id = self.lexer.current_token_content();

                self.lexer.next_token();

                if self.lexer.current_token().token_type != TokenType::Assign {
                    self.lexer.previous_token();

                    let expr = try!(self.parse_expression());

                    return Ok(Statement::Expression(Box::new(expr)))
                }

                self.lexer.next_token();

                let expr = try!(self.parse_expression());

                Ok(Statement::Assignment(id, Box::new(expr)))
            },

            TokenType::If => {
                self.lexer.next_token();

                let condition = try!(self.parse_expression());

                self.lexer.next_token();

                let body = try!(self.parse_block());

                self.lexer.next_token();

                if self.lexer.current_token().token_type == TokenType::Else {
                    self.lexer.next_token();

                    let else_body = try!(self.parse_block());

                    return Ok(Statement::IfElse(
                        Box::new(condition),
                        Box::new(Statement::Block(Box::new(body))),
                        Box::new(Statement::Block(Box::new(else_body))),
                    ))
                }

                Ok(Statement::If(
                    Box::new(condition),
                    Box::new(Statement::Block(Box::new(body))),
                ))
            },

            _ => {
                let expr = try!(self.parse_expression());
                Ok(Statement::Expression(Box::new(expr)))
            }
        }
    }

    pub fn parse_full(&mut self) -> Result<Vec<Statement>, String> {
        let mut statement_stack = Vec::new();

        loop {
            if self.lexer.tokens_remaining() < 1 {
                break
            }

            statement_stack.push(try!(self.parse_statement()));

            self.lexer.next_token();
        }

        Ok(statement_stack)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        try!(self.lexer.match_current_token(TokenType::Do));

        let mut block_tokens = Vec::new();
        let mut opened_dos   = 1;

        while self.lexer.next_token() {
            if self.lexer.current_token().token_type == TokenType::Do {
                opened_dos += 1;     
            } else if self.lexer.current_token().token_type == TokenType::End {
                opened_dos -= 1;     
            }

            if opened_dos == 0 {
                break
            }

            block_tokens.push(self.lexer.current_token().clone());
        }

        try!(self.lexer.match_current_token(TokenType::End));

        Parser::parse_from_tokens(block_tokens)
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

    // Invoked when LParen is popped
    fn parse_caller(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut stack = Vec::new();

        self.lexer.next_token();

        while self.lexer.current_token().token_type != TokenType::RParen {
            stack.push(try!(self.parse_expression()));
            
            self.lexer.next_token();

            if self.lexer.current_token().token_type == TokenType::Comma {
                self.lexer.next_token();
            }
        }

        Ok(Expression::Call(Box::new(callee), Box::new(stack)))
    }
}