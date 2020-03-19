use crate::lexer::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    input: &'a str,
}
impl<'a> Lexer<'a> {

}
pub struct Cursor<'a> {
    chars: CharIndices<'a>
}
impl<'a> Cursor<'a> {

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next().map(|c| c.1)
    }
    fn next_char(&mut self) -> Option<char> {
        self.chars.next().map(|c| c.1)
    }
    fn take_while(&mut self, f: impl FnMut(char) -> bool) -> Option<&'a str> {
        let start = self.chars.clone();
        let mut i = self.chars.clone();
        while i.next().map(|c| f(c.1)).unwrap_or(false) {
            self.chars = i;
        }
        Some(start.as_str()[..self.chars.clone().next])
    }
}