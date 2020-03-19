use crate::lexer::text::Text;
use crate::lexer::number::{BigInt, Number};
use std::str::FromStr;
use crate::lexer::identifier::Identifier;
use crate::lexer::punctuator::Punctuator;
use crate::lexer::keyword::Keyword;

pub enum Token<'a> {
    Null,
    Text(Text<'a>),
    Identifier(Identifier<'a>),
    Number(Number),
    BigInt(BigInt),
    Boolean(bool),
    Keyword(Keyword),
    Punctuator(Punctuator)
}
impl Token {

}
impl<'a> FromStr for Token<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "null" => Ok(Token::Null),
            "false" => Ok(Token::Boolean(false)),
            "true" => Ok(Token::Boolean(true)),
            _ => {
                if let Ok(punctuator) = Punctuator::from_str(s) {
                    return Ok(Token::Punctuator(punctuator))
                }
                if let Ok(keyword) = Keyword::from_str(s) {
                    return Ok(Token::Keyword(keyword))
                }
            }
        }
    }
}