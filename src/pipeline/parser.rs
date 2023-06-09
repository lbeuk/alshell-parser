use super::Pipeline;
use crate::command::Command;
use crate::error::{ParseError::UnterminatedCommand, ParseResult};
use crate::{command::parser::CommandParser, ParseTools, C_SPACE};
use crate::{C_END_CMD, C_PIPE};
use std::cell::{RefCell, RefMut};

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

impl<'a, T: ParseTools> From<&'a T> for PipelineParser<'a> {
    fn from(value: &'a T) -> Self {
        return PipelineParser {
            chars: value.chars(),
            idx: value.index_refcell(),
            pipeline: Default::default(),
        };
    }
}

enum PipelineStatus {
    StartCmd,
    FinishedCmd(Command),
}

impl<'a> PipelineParser<'a> {
    pub fn parse(mut self) -> ParseResult<Pipeline> {
        let mut pipe_status = PipelineStatus::StartCmd;

        while !self.eof() {
            let c = self.cur_char();
            println!("{} {}", c, self.idx_val());

            match (&pipe_status, c) {
                (PipelineStatus::FinishedCmd(c), C_PIPE) => {
                    self.pipeline.push(c.clone());
                    pipe_status = PipelineStatus::StartCmd;
                },
                (PipelineStatus::FinishedCmd(_), C_END_CMD) => break,
                (PipelineStatus::FinishedCmd(_), _) => return Err(UnterminatedCommand(self.idx_val())),
                (PipelineStatus::StartCmd, C_SPACE) => {},
                (PipelineStatus::StartCmd, _) => pipe_status = PipelineStatus::FinishedCmd(CommandParser::from(&self).parse()?),
            }

            self.increment();
        }

        match pipe_status {
            PipelineStatus::StartCmd => {},
            PipelineStatus::FinishedCmd(c) => self.pipeline.push(c),
        }

        return Ok(self.pipeline);
    }
}
