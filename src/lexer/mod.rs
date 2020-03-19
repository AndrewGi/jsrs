pub mod token;
pub mod literal;
pub mod text;
pub mod number;
pub mod keyword;
pub mod error;
pub mod punctuator;
pub mod lexer;
pub mod identifier;
pub mod regex;

pub use lexer::Lexer;
pub trait Peekable: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}
impl<T: Iterator> Peekable for std::iter::Peekable<T> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}