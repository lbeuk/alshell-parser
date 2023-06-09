use core::fmt;
use std::error::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacter(char, usize),
    UnterminatedCommand(usize),
    UnsucceededPipe(usize)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexptected token: {} at {}.", c, pos)
            },
            Self::UnterminatedCommand(pos) => {
                write!(f, "Unterminated command at {}. THIS ERROR SHOULD NEVER HAPPEN, PLEASE REPORT THIS ERROR", pos)
            },
            Self::UnsucceededPipe(pos) => {
                write!(f, "Pipeline that wasn't followed by a command at {}.", pos)
            }
        }
    }
}

impl Error for ParseError {}
