use syntax::tokens::{
    TokenType,
    Token,
    BinOp,
};

/* A sadly OOP approach on a lexer.
 * Potentially improved using a peekable iterator.
 */

pub struct Lexer {
    tokens:       Vec<Token>,
    lines:        u32,
    start:        usize,
    pos:          usize,
    top:          usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            lines:  0,
            start:  0,
            pos:    0,
            top:    0,
        }
    }

    pub fn from(tokens: Vec<Token>) -> Lexer {
        Lexer {
            tokens: tokens,
            lines:  0,
            start:  0,
            pos:    0,
            top:    0,
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn reset(&mut self) {
        self.tokens = Vec::new();
        self.lines = 0;
        self.start = 0;
        self.pos   = 0;
        self.top   = 0;
    }

    fn push_token(&mut self, token_type: TokenType, line: &str) {
        self.tokens.push(Token::new(
            token_type,

            String::from(&line[self.start .. self.pos]),
            self.lines,
            self.pos as u32,
        ));

        self.start = self.pos;
    }

    fn look(&self, line: &str, offset: usize) -> char {
        match line.chars().nth(self.pos + offset) {
            Some(v) => v,
            None    => ' ',
        }
    }

    fn skip_whitespace(&mut self, line: &str) {
        while self.look(line, 0) == ' ' && self.pos < line.len() - 1 {
            self.pos   += 1;
            self.start += 1;
        }
    }

    pub fn bin_op(v: &str) -> Option<(BinOp, u8)> {
        match v {
            "*"  => Some((BinOp::Mul, 1)),
            "/"  => Some((BinOp::Div, 1)),
            "+"  => Some((BinOp::Plus, 2)),
            "-"  => Some((BinOp::Minus, 2)),
            "=="  => Some((BinOp::Equal, 4)),
            "~=" => Some((BinOp::NotEqual, 4)),
            "<"  => Some((BinOp::Lt, 4)),
            ">"  => Some((BinOp::Gt, 4)),
            "<=" => Some((BinOp::GtEqual, 4)),
            ">=" => Some((BinOp::LtEqual, 4)),
            _    => None,
        }
    }

    fn keyword(&mut self, line: &str) -> Option<TokenType> {
        match &line[self.start .. self.pos] {
            "define" => Some(TokenType::Definition),
            "lambda" => Some(TokenType::Lambda),
            "if"     => Some(TokenType::If),
            "else"   => Some(TokenType::Else),
            "return" => Some(TokenType::Return),
            "true"   => Some(TokenType::True),
            "false"  => Some(TokenType::False),
            "do"     => Some(TokenType::Do),
            "end"    => Some(TokenType::End),
            _        => None
        }
    }

    fn is_bin_op(&mut self, line: &str) -> bool {
        let mut is_bin_op = false;
        
        let mut offset = 2;
        while self.pos + offset >= line.len() {
            offset -= 1;
        }

        while offset > 0 && !is_bin_op {
            match Lexer::bin_op(&line[self.start .. self.pos + offset]) {
                Some(_) => is_bin_op = true,
                None => ()
            }
            offset -= 1;
        }

        self.pos += offset;
        is_bin_op
    }
    
    pub fn next_token(&mut self) -> bool {
        if self.top < self.tokens.len() {
            self.top += 1;
            return true
        }
        false
    }

    pub fn previous_token(&mut self) -> bool {
        if self.top != 0 {
            self.top -= 1;
            return true
        }
        false
    }

    pub fn tokens_remaining(&self) -> usize {
        self.tokens.len() - self.top
    }

    pub fn current_token(&self) -> &Token {
        if self.top > self.tokens.len() - 1 {
            return &self.tokens[self.tokens.len() - 1]
        }
        &self.tokens[self.top]
    }

    pub fn current_token_content(&self) -> String {
        self.current_token().content.clone()
    }

    pub fn match_current_token(&self, t: TokenType) -> Result<&Token, String> {
        match self.current_token().token_type == t {
            true  => Ok(self.current_token()),
            false => Err(format!(
                "expected {:?} but found {:?}", t, self.current_token()
            ))
        }
    }

    fn push_move(&mut self, t: TokenType, line: &str) {
        self.pos += 1;
        self.push_token(t, line);
    }

    pub fn tokenize(&mut self, source: String) -> Result<(), String> {

        fn identifier_valid(c: char) -> bool {
            c.is_alphabetic() || c == '_' 
                              || c == '?'
                              || c == '!'
                              || c.is_digit(10)
        }

        for line in source.lines() {
            self.lines += 1;
            self.start  = 0;
            self.pos    = 0;

            while self.pos < line.len() {
                self.skip_whitespace(line);

                let chr  = self.look(line, 0);

                if chr == '"' || chr == '\'' {
                    let del = chr;

                    self.start += 1;
                    self.pos   += 1;

                    while self.look(line, 0) != del {
                        self.pos += 1;
                    }

                    self.push_token(TokenType::Text, line);

                    self.start += 1;
                    self.pos   += 1;

                    continue
                }

                if chr.is_alphabetic() {
                    while identifier_valid(self.look(line, 0)) {
                        self.pos += 1;
                    }

                    match self.keyword(line) {
                        Some(t) => self.push_token(t, line),
                        None    => self.push_token(TokenType::Identifier, line),
                    }

                    continue
                }

                let peek = self.look(line, 1);

                if chr.is_digit(10) ||
                   chr == '.' && peek.is_digit(10) ||
                   chr == '-' && peek.is_digit(10) {

                    if chr == '-' {
                        self.pos += 1;
                    }

                    while self.look(line, 0).is_digit(10) {
                        self.pos += 1;
                    }

                    if self.look(line, 0) == '.' && self.look(line, 1).is_digit(10) {
                        self.pos += 1;
                        while self.look(line, 0).is_digit(10) {
                            self.pos += 1;
                        }
                        self.push_token(TokenType::Float, line);
                        continue;
                    }
                    self.push_token(TokenType::Integer, line);
                    continue;
                }

                if chr == '-' && self.look(line, 1) == '>'  {
                    self.pos += 2;
                    self.push_token(TokenType::Arrow, line);
 
                    continue
                }

                if self.is_bin_op(line) {
                    self.pos += 1;
                    self.push_token(TokenType::BinOp, line);

                    continue
                }

                match chr {
                    '=' => {
                        self.push_move(TokenType::Assign, line);
                        continue
                    }

                    '(' => {
                        self.push_move(TokenType::LParen, line);
                        continue
                    }

                    ')' => {
                        self.push_move(TokenType::RParen, line);
                        continue
                    }

                    '[' => {
                        self.push_move(TokenType::LBracket, line);
                        continue
                    }

                    ']' => {
                        self.push_move(TokenType::RBracket, line);
                        continue
                    }

                    '{' => {
                        self.push_move(TokenType::LBrace, line);
                        continue
                    }

                    '}' => {
                        self.push_move(TokenType::RBrace, line);
                        continue
                    }

                    ':' => {
                        self.push_move(TokenType::Colon, line);
                        continue
                    }

                    ',' => {
                        self.push_move(TokenType::Comma, line);
                        continue
                    }

                    '.' => {
                        self.push_move(TokenType::Period, line);
                        continue
                    }

                    ';' => {
                        self.push_move(TokenType::Semicolon, line);
                        continue
                    }

                    ' '  => break,
                    '\0' => break,
                    '\n' => break,

                    _   => {
                        panic!("fucked symbol: {}, line: {} col: {}",
                                &line[self.start .. line.len()],
                                self.lines, self.start)
                    },
                }
            }
        }
    
        Ok(())
    }
}