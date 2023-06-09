use core::fmt;
use std::error::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacter(char, usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexptected token: {} at {}.", c, pos)
            }
        }
    }
}

impl Error for ParseError {}
