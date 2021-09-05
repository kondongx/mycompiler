#![allow(dead_code, non_camel_case_types, unused_variables)]

use crate::{token::Token, token_reader::TokenReader, token_type::TokenType};

pub enum DfaState {
    Initial,

    If,
    Id_if1,
    Id_if2,
    Else,
    Id_else1,
    Id_else2,
    Id_else3,
    Id_else4,
    Int,
    Id_int1,
    Id_int2,
    Id_int3,
    Id,
    GT,
    GE,

    Assignment,

    Plus,
    Minus,
    Star,
    Slash,

    SemiColon,
    LeftParen,
    RightParen,

    IntLiteral,
}

impl Default for DfaState {
    fn default() -> Self {
        DfaState::Initial
    }
}

pub struct SimpleLexer {
    token_text: String,
    tokens: Vec<Box<dyn Token>>,
    token: SimpleToken,
    dfastate: DfaState,
}

impl Default for SimpleLexer {
    fn default() -> Self {
        Self {
            token_text: Default::default(),
            tokens: Default::default(),
            token: Default::default(),
            dfastate: Default::default(),
        }
    }
}

impl SimpleLexer {
    pub fn new(
        token_text: String,
        tokens: Vec<Box<dyn Token>>,
        token: SimpleToken,
        dfastate: DfaState,
    ) -> Self {
        Self {
            token_text,
            tokens,
            token,
            dfastate,
        }
    }

    pub fn dump(mut token_reader: SimpleTokenReader) {
        println!("text\t\ttype");
        while let Some(token) = token_reader.read() {
            println!("{}\t\t{:?}", token.get_text(), token.get_type())
        }
    }

    fn is_alpha(&self, ch: char) -> bool {
        ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z'
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_blank(&self, ch: char) -> bool {
        ch == ' ' || ch == '\t' || ch == '\n'
    }

    fn init_token(&mut self, ch: char) -> DfaState {
        if self.token_text.len() > 0 {
            self.token.text = self.token_text.clone();
            self.tokens.push(Box::new(self.token.clone()));

            self.token_text = String::new();
        }

        let new_state: DfaState;
        if self.is_alpha(ch) {
            // 第一个字符是字母
            if ch == 'i' {
                new_state = DfaState::Id_int1;
            } else {
                new_state = DfaState::Id // 进入 Id 状态
            }
            self.token.token_type = TokenType::Identifier;
            self.token_text.push(ch);
        } else if self.is_digit(ch) {
            new_state = DfaState::IntLiteral;
            self.token.token_type = TokenType::IntLiteral;
            self.token_text.push(ch);
        } else {
            match ch {
                '>' => {
                    new_state = DfaState::GT;
                    self.token.token_type = TokenType::GT;
                    self.token_text.push(ch);
                }
                '+' => {
                    new_state = DfaState::Plus;
                    self.token.token_type = TokenType::Plus;
                    self.token_text.push(ch);
                }
                '-' => {
                    new_state = DfaState::Minus;
                    self.token.token_type = TokenType::Minus;
                    self.token_text.push(ch);
                }
                '*' => {
                    new_state = DfaState::Star;
                    self.token.token_type = TokenType::Star;
                    self.token_text.push(ch);
                }
                '/' => {
                    new_state = DfaState::Slash;
                    self.token.token_type = TokenType::Slash;
                    self.token_text.push(ch);
                }
                ';' => {
                    new_state = DfaState::SemiColon;
                    self.token.token_type = TokenType::SemiColon;
                    self.token_text.push(ch);
                }
                '(' => {
                    new_state = DfaState::LeftParen;
                    self.token.token_type = TokenType::LeftParen;
                    self.token_text.push(ch);
                }
                ')' => {
                    new_state = DfaState::RightParen;
                    self.token.token_type = TokenType::RightParen;
                    self.token_text.push(ch);
                }
                '=' => {
                    new_state = DfaState::Assignment;
                    self.token.token_type = TokenType::Assignment;
                    self.token_text.push(ch);
                }
                _ => {
                    new_state = DfaState::Initial;
                }
            }
        }
        new_state
    }

    pub fn tokenizer(mut self, code: String) -> SimpleTokenReader {
        let mut state = DfaState::Initial;
        for ch in code.chars() {
            match state {
                DfaState::Initial => state = self.init_token(ch), // 重新确定后续状态
                DfaState::Id => {
                    if self.is_alpha(ch) || self.is_digit(ch) {
                        self.token_text.push(ch);
                    } else {
                        state = self.init_token(ch);
                    }
                }
                DfaState::GT => {
                    if ch == '=' {
                        self.token.token_type = TokenType::GE; // 转换为 GE 状态
                        state = DfaState::GE;
                        self.token_text.push(ch);
                    } else {
                        state = self.init_token(ch); // 退出 GT 状态, 并保存 Token
                    }
                }
                DfaState::GE => {}
                DfaState::Assignment => {
                    if self.is_blank(ch) {
                        state = self.init_token(ch);
                    } else {
                        self.token.token_type = TokenType::Assignment;
                        state = DfaState::Assignment;
                        self.token_text.push(ch);
                    }
                }
                DfaState::Plus => {}
                DfaState::Minus => {}
                DfaState::Star => {}
                DfaState::Slash => {}
                DfaState::SemiColon => {}
                DfaState::LeftParen => {}
                DfaState::RightParen => state = self.init_token(ch),
                DfaState::IntLiteral => {
                    if self.is_digit(ch) {
                        self.token_text.push(ch);
                    } else {
                        state = self.init_token(ch); // 退出当前状态,并保留 Token
                    }
                }
                DfaState::Id_int1 => {
                    if ch == 'n' {
                        state = DfaState::Id_int2;
                        self.token_text.push(ch);
                    } else if self.is_digit(ch) || self.is_alpha(ch) {
                        state = DfaState::Id;
                        self.token_text.push(ch);
                    }
                }
                DfaState::Id_int2 => {
                    if ch == 't' {
                        state = DfaState::Id_int3;
                        self.token_text.push(ch);
                    } else if self.is_digit(ch) || self.is_alpha(ch) {
                        state = DfaState::Id;
                        self.token_text.push(ch);
                    }
                }
                DfaState::Id_int3 => {
                    if self.is_blank(ch) {
                        self.token.token_type = TokenType::Int;
                        state = self.init_token(ch);
                    }
                }
                _ => {}
            }
        }
        SimpleTokenReader {
            tokens: self.tokens,
            pos: 0,
        }
    }
}

#[derive(Clone)]
pub struct SimpleToken {
    pub token_type: TokenType,
    pub text: String,
}

impl Default for SimpleToken {
    fn default() -> Self {
        Self {
            token_type: Default::default(),
            text: Default::default(),
        }
    }
}

impl Token for SimpleToken {
    fn get_type(&self) -> &TokenType {
        &self.token_type
    }
    fn get_text(&self) -> &str {
        &self.text
    }
}

pub struct SimpleTokenReader {
    pub tokens: Vec<Box<dyn Token>>,
    pub pos: usize,
}

impl TokenReader for SimpleTokenReader {
    fn read<'a>(&'a mut self) -> Option<&'a Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            let pos = self.pos;
            self.pos += 1;
            return Some(self.tokens.get(pos).unwrap());
        }
        None
    }

    fn peek<'a>(&'a self) -> Option<&'a Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            return Some(self.tokens.get(self.pos).unwrap());
        }
        None
    }

    fn unread(&mut self) -> () {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn get_position(&self) -> usize {
        return self.pos;
    }

    fn set_position(&mut self, pos: usize) -> () {
        if pos < self.tokens.len() {
            self.pos = pos
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleLexer;

    #[test]
    fn test_simple_lexer() {
        let lexer = SimpleLexer::default();
        let script = String::from("int age = 45;");

        println!("parse: {}", script);
        let token_reader = lexer.tokenizer(script);
        SimpleLexer::dump(token_reader);
    }

    #[test]
    fn test_chars() {
        let code = String::from("int age = 45;");
        for ch in code.chars() {
            if ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' {
                println!("ch is alpha {}", ch);
            }
            if ch >= '0' && ch <= '9' {
                println!("ch is num {}", ch)
            }
        }
    }
}
