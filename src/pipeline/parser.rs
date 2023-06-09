use std::cell::{RefMut, RefCell};
use super::Pipeline;
use crate::error::{ParseResult, ParseError::UnexpectedCharacter};
use crate::{C_PIPE, C_END_CMD};
use crate::{command::parser::CommandParser, ParseTools, C_SPACE};

pub(crate) struct PipelineParser<'a> {
    pub(crate) chars: &'a [char],
    pub(crate) idx: &'a RefCell<usize>,
    pipeline: Pipeline,
}

impl<'a> ParseTools for PipelineParser<'a> {
    fn chars(&self) -> &[char] {
        return self.chars;
    }

    fn idx_val(&self) -> usize {
        return *self.idx.borrow();
    }

    fn idx_mut(&self) -> RefMut<usize> {
        return self.idx.borrow_mut();
    }

    fn index_refcell(&self) -> &RefCell<usize> {
        return self.idx;
    }
}

impl <'a, T: ParseTools> From<&'a T> for PipelineParser<'a> {
    fn from(value: &'a T) -> Self {
        return PipelineParser {
            chars: value.chars(),
            idx: value.index_refcell(),
            pipeline: Default::default(),
        }
    }
}

impl<'a> PipelineParser<'a> {

    pub fn parse(mut self) -> ParseResult<Pipeline> {
        while !self.eof() {
            let c = self.cur_char();

            match c {
                C_SPACE => {},
                C_PIPE => panic!(),
                _ => {
                    let new_cmd = CommandParser::from(&self).parse()?;
                    self.pipeline.push(new_cmd);

                    let c = self.cur_char();
                    let i = self.idx_val();
                    match c {
                        C_PIPE => {},
                        C_END_CMD => break,
                        _ => return Err(UnexpectedCharacter(c, i))
                    }
                }
            }

            self.increment();
        }
        return Ok(self.pipeline);
    }

}
