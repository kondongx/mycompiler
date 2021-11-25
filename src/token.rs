use std::fmt::Debug;

use crate::token_type::TokenType;

pub trait Token: TokenClone + Debug {
    fn get_type(&self) -> &TokenType;
    fn get_text(&self) -> &str;
}

pub trait TokenClone {
    fn clone_box(&self) -> Box<dyn Token>;
}

impl<T> TokenClone for T
where
    T: 'static + Token + Clone,
{
    fn clone_box(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Token> {
    fn clone(&self) -> Box<dyn Token> {
        self.clone_box()
    }
}
