use crate::opcode::Opcode;
use crate::precedence::Precedence;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::InterpretError;
use crate::Chunk;
use crate::Scanner;

pub struct Parser {
    current: Token,
    previous: Token,
    scanner: Scanner,
    chunk: Chunk,
    had_error: bool,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            current: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            previous: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
            had_error: false,
        }
    }

    pub fn compile(mut self) -> Result<Chunk, InterpretError> {
        self.advance();
        self.declaration();
        self.emit_op(Opcode::Return);
        Ok(self.chunk)
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn declaration(&mut self) {
        if let TokenType::Identifier(name) = self.current.token_type.clone() {
            self.advance();
            if self.matches(TokenType::Equal) {
                self.variable_declaration(name);
                return;
            }
        }

        self.statement();
    }

    fn statement(&mut self) {
        match self.current.token_type.clone() {
            TokenType::Print => {
                self.advance();
                self.print_statement();
            }
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.emit_op(Opcode::Pop);
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        if let Some(tok) = self.scanner.next() {
            self.current = tok;
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if !(self.current.token_type == token_type) {
            self.error_at_current(msg);
        }

        self.advance();
    }

    pub fn string(&mut self) {
        match &self.previous.token_type {
            TokenType::String(s) => self.emit_constant(Value::String(s.to_string())),
            _ => unreachable!("No string"),
        }
    }

    fn error(&mut self, msg: &str) {
        self.error_at(self.previous.clone(), msg);
    }

    fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.current.clone(), msg);
    }

    fn error_at(&mut self, tok: Token, msg: &str) {
        println!("[line {}] Error: {}", tok.line, msg);
        self.had_error = true;
    }

    pub fn number(&mut self) {
        if let TokenType::Number(num) = self.previous.token_type {
            self.emit_constant(Value::Number(num));
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, a: u8, b: u8) {
        self.emit_byte(a);
        self.emit_byte(b);
    }

    fn emit_op(&mut self, op: Opcode) {
        self.emit_byte(op as u8);
    }

    fn emit_ops(&mut self, op1: Opcode, op2: Opcode) {
        self.emit_byte(op1 as u8);
        self.emit_byte(op2 as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value) as u8;
        self.emit_bytes(Opcode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = self.chunk.add_constant(value);
        if constant > std::u8::MAX as usize {
            self.error("Too many constants in this chunk");
            0
        } else {
            constant
        }
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression");
    }

    pub fn unary(&mut self) {
        let operator = self.previous.token_type.clone();
        self.parse_precedence(Precedence::Unary);

        match operator {
            TokenType::Minus => self.emit_op(Opcode::Negate),
            TokenType::Bang => self.emit_op(Opcode::Not),
            _ => unreachable!("Impossible unary operator"),
        }
    }

    pub fn binary(&mut self) {
        let operator = self.previous.token_type.clone();
        let rule = operator.rule();
        let precedence = Precedence::from(rule.precedence as usize + 1);
        self.parse_precedence(precedence);

        match operator {
            TokenType::Plus => self.emit_op(Opcode::Add),
            TokenType::Minus => self.emit_op(Opcode::Subtract),
            TokenType::Star => self.emit_op(Opcode::Multiply),
            TokenType::Slash => self.emit_op(Opcode::Divide),
            TokenType::BangEqual => self.emit_ops(Opcode::Equal, Opcode::Not),
            TokenType::EqualEqual => self.emit_op(Opcode::Equal),
            TokenType::Greater => self.emit_op(Opcode::Greater),
            TokenType::GreaterEqual => self.emit_ops(Opcode::Less, Opcode::Not),
            TokenType::Less => self.emit_op(Opcode::Less),
            TokenType::LessEqual => self.emit_ops(Opcode::Greater, Opcode::Not),
            TokenType::BitwiseAnd => self.emit_op(Opcode::BitwiseAnd),
            TokenType::BitwiseOr => self.emit_op(Opcode::BitwiseOr),
            _ => (),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let rule = self.previous.token_type.rule();

        if let Some(prefix_rule) = rule.prefix {
            prefix_rule(self);

            let prec_u8 = precedence as u8;
            while prec_u8 <= self.current.token_type.rule().precedence as u8 {
                self.advance();
                if let Some(infix_rule) = self.previous.token_type.rule().infix {
                    infix_rule(self);
                }
            }

            return;
        }

        self.error("Expected expression");
    }

    pub fn literal(&mut self) {
        let token_type = self.previous.token_type.clone();
        match token_type {
            TokenType::False => self.emit_op(Opcode::False),
            TokenType::Nil => self.emit_op(Opcode::Nil),
            TokenType::True => self.emit_op(Opcode::True),
            _ => unreachable!("Impossible TokenType in literal"),
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.emit_op(Opcode::Print);
        println!();
    }

    fn matches(&mut self, token_type: TokenType) -> bool {
        if self.current.token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn variable(&mut self) {
        if let TokenType::Identifier(name) = self.previous.token_type.clone() {
            let var = self.make_constant(Value::String(name));
            self.emit_op(Opcode::GetGlobal);
            self.emit_byte(var as u8);
        }
    }

    fn variable_declaration(&mut self, name: String) {
        self.expression();
        let global = self.make_constant(Value::String(name));
        self.define_global(global);
    }

    fn define_global(&mut self, global: usize) {
        self.emit_op(Opcode::DefineGlobal);
        self.emit_byte(global as u8);
    }
}
