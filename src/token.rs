use crate::token_type::TokenType;

pub trait Token {
    fn get_type(&self) -> &TokenType;
    fn get_text(&self) -> &str;
}
