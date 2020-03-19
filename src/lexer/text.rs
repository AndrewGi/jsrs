use std::borrow::Cow;
pub struct Text<'a> {
    content: Cow<'a, str>
}
pub struct Char {
    c: char
}