use crate::parse_rule::ParseRule;
use crate::parser::Parser;
use crate::precedence::Precedence;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // 1 character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 1 or 2 character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fn,
    If,
    Nil,
    Or,
    Puts,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Misc tokens
    Error(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, col: usize, len: usize) -> Token {
        Token {
            token_type,
            line,
            col,
            len,
        }
    }
}

impl TokenType {
    pub fn to_usize(self) -> usize {
        match self {
            TokenType::LeftParen => 0,
            TokenType::RightParen => 1,
            TokenType::LeftBrace => 2,
            TokenType::RightBrace => 3,
            TokenType::Comma => 4,
            TokenType::Dot => 5,
            TokenType::Minus => 6,
            TokenType::Plus => 7,
            TokenType::Semicolon => 8,
            TokenType::Slash => 9,
            TokenType::Star => 10,
            TokenType::Bang => 11,
            TokenType::BangEqual => 12,
            TokenType::Equal => 13,
            TokenType::EqualEqual => 14,
            TokenType::Greater => 15,
            TokenType::GreaterEqual => 16,
            TokenType::Less => 17,
            TokenType::LessEqual => 18,
            TokenType::Identifier(_) => 19,
            TokenType::String(_) => 20,
            TokenType::Number(_) => 21,
            TokenType::And => 22,
            TokenType::Class => 23,
            TokenType::Else => 24,
            TokenType::False => 25,
            TokenType::For => 26,
            TokenType::Fn => 27,
            TokenType::If => 28,
            TokenType::Nil => 29,
            TokenType::Or => 30,
            TokenType::Puts => 31,
            TokenType::Return => 32,
            TokenType::Super => 33,
            TokenType::This => 34,
            TokenType::True => 35,
            TokenType::Var => 36,
            TokenType::While => 37,
            TokenType::Error(_) => 38,
        }
    }

    pub fn rule(&self) -> &'static ParseRule {
        match self {
            TokenType::LeftParen => &ParseRule {
                prefix: Some(Parser::grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightParen => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LeftBrace => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightBrace => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Comma => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Dot => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => &ParseRule {
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => &ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Semicolon => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Slash => &ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Star => &ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Bang => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::BangEqual => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Equal => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EqualEqual => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Greater => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::GreaterEqual => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Less => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LessEqual => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Identifier(_) => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::String(_) => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Number(_) => &ParseRule {
                prefix: Some(Parser::number),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::And => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Class => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Else => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::False => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::For => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Fn => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::If => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Nil => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Or => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Puts => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Return => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Super => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::This => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::True => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Var => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::While => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Error(_) => &ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }
}
