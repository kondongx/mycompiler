use crate::token::Token;

pub trait TokenReader {
    fn read<'a>(&'a mut self) -> Option<&'a Box<dyn Token>>;
    fn peek<'a>(&'a self) -> Option<&'a Box<dyn Token>>;
    fn unread(&mut self) -> ();
    fn get_position(&self) -> usize;
    fn set_position(&mut self, pos: usize) -> ();
}
