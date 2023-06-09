use std::cell::{RefCell, RefMut};

use error::ParseResult;
use pipeline::parser::PipelineParser;
use script::Script;

pub mod command;
pub mod error;
pub mod pipeline;
mod script;

pub const C_ESCAPE: char = '\\';
pub const C_QUOTE: char = '"';
pub const C_LIT_QUOTE: char = '\'';
pub const C_EXE_BLOCK: char = '`';
pub const C_SPACE: char = ' ';
pub const C_NEWLINE: char = '\n';
pub const C_SUPER: char = '$';
pub const C_END_CMD: char = ';';
pub const C_PIPE: char = '|';

trait ParseTools {
    fn chars(&self) -> &[char];

    fn index_refcell(&self) -> &RefCell<usize>;

    fn idx_val(&self) -> usize;

    fn idx_mut(&self) -> RefMut<usize>;

    fn increment(&self) {
        *self.idx_mut() += 1;
    }

    fn decrement(&self) {
        *self.idx_mut() -= 1;
    }

    fn eof(&self) -> bool {
        return self.chars().len() <= self.idx_val();
    }

    fn cur_char(&self) -> char {
        let i = self.idx_val();
        return self.chars()[i];
    }
}

pub struct Parser {
    chars: Vec<char>,
    index: RefCell<usize>,
    script: Script,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            chars: Vec::new(),
            index: RefCell::new(0),
            script: Default::default(),
        };
    }

    pub fn with_text(text: &str) -> Parser {
        let mut p = Parser::new();
        p.feed(text);
        return p;
    }

    pub fn feed(&mut self, text: &str) {
        self.chars.extend(text.chars());
    }

    pub fn reset(&mut self) {
        *self.index.borrow_mut() = 0;
    }

    pub fn parse(mut self) -> ParseResult<Script>{
        while !self.eof() {
            let c = self.cur_char();

            match c {
                _ => {
                    let new_pipeline = PipelineParser::from(&self).parse()?;
                    self.script.push(new_pipeline);
                }
            }
        }

        return Ok(self.script);
    }
}

impl ParseTools for Parser {
    fn chars(&self) -> &[char] {
        return &self.chars;
    }

    fn idx_val(&self) -> usize {
        return *self.index.borrow();
    }

    fn idx_mut(&self) -> RefMut<usize> {
        return self.index.borrow_mut();
    }

    fn index_refcell(&self) -> &RefCell<usize> {
        return &self.index;
    }
}
