#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Integer,
    Float,
    Text,
    Identifier,
    Assign,
    Function,
    Lambda,
    Return,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Colon,
    Comma,
    Period,
    False,
    True,
    If,
    Else,
    BinOp,
    Semicolon,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Plus,
    Minus,
    Equal,
    NotEqual,
    Lt,
    Gt,
    LtEqual,
    GtEqual,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content:    String,
    pub row:        u32,
    pub col:        u32,
}

impl Token {
    pub fn new(token_type: TokenType, content: String, row: u32, col: u32) -> Token {
        Token {
            token_type: token_type,
            content:    content,
            row:        row,
            col:        col,
        }
    }
}