use std::borrow::Cow;

pub struct Identifier<'a> {
    name: Cow<'a, str>,
}