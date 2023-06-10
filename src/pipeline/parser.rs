use super::Pipeline;
use crate::command::Command;
use crate::error::{ParseError::{UnterminatedCommand, UnsucceededPipe}, ParseResult};
use crate::{command::parser::CommandParser, ParseTools};
use crate::chars::*;
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
    AwaitingCmd,
    FinishedCmd(Command),
}

impl<'a> PipelineParser<'a> {
    pub fn parse(mut self) -> ParseResult<Pipeline> {
        use PipelineStatus::*;
        let mut pipe_status = PipelineStatus::StartCmd;

        while !self.eof() {
            let c = self.cur_char();

            match (&pipe_status, c) {
                (PipelineStatus::FinishedCmd(c), C_PIPE) => {
                    self.pipeline.push(c.clone());
                    pipe_status = PipelineStatus::AwaitingCmd;
                },
                (FinishedCmd(_), C_END_CMD | C_NEWLINE) => break,
                (FinishedCmd(_), _) => return Err(UnterminatedCommand(self.idx_val())),
                (StartCmd | AwaitingCmd, C_SPACE) | (AwaitingCmd, C_NEWLINE) => {},
                (StartCmd | AwaitingCmd, _) => pipe_status = PipelineStatus::FinishedCmd(CommandParser::from(&self).parse()?),
            }

            self.increment();
        }

        match pipe_status {
            StartCmd => {},
            FinishedCmd(c) => self.pipeline.push(c),
            AwaitingCmd => return Err(UnsucceededPipe(self.idx_val()))
        }

        return Ok(self.pipeline);
    }
}
