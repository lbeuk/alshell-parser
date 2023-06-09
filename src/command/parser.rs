
use std::cell::{RefMut, RefCell};

use crate::error::ParseResult;
use crate::{C_QUOTE, C_LIT_QUOTE, C_EXE_BLOCK, C_SPACE, C_NEWLINE, C_SUPER, C_END_CMD, C_ESCAPE, C_PIPE};
use crate::ParseTools;

use super::{QuoteComponenet, TokenComponent, Token, Command};

pub (crate) struct CommandParser<'a> {
    chars: &'a [char],
    idx: &'a RefCell<usize>,
    command: Command,
}

impl<'a> ParseTools for CommandParser<'a> {
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

impl <'a, T: ParseTools> From<&'a T> for CommandParser<'a> {
    fn from(value: &'a T) -> Self {
        return CommandParser {
            chars: value.chars(),
            idx: value.index_refcell(),
            command: Default::default(),
        }
    }
}

impl<'a> CommandParser<'a> {

    pub fn parse(mut self) -> ParseResult<Command>{
        let mut components = Vec::new();

        while !self.eof() {
            let c = self.cur_char();
            
            match c {
                C_SPACE => {
                    self.command.tokens.push(Token { components });
                    components = Vec::new();
                }
                C_LIT_QUOTE => components.push(self.parse_lit_quoted()),
                C_QUOTE => components.push(self.parse_quoted()),
                C_END_CMD | C_PIPE => break,
                _ => components.push(self.parse_string()),
            };

            self.increment();
        }

        if components.len() > 0 {
            self.command.tokens.push(Token { components });
        }

        self.decrement();

        return Ok(self.command);
    }

    fn parse_string(&mut self) -> TokenComponent {
        let mut escaped = false;
        let mut set_escaped = false;
        let mut rendered = String::new();

        while !self.eof() {
            let c = self.cur_char();
            println!("{} {}", escaped, self.idx_val());

            match (escaped, c) {
                (false, C_ESCAPE) => set_escaped = true,
                (
                    false,
                    C_QUOTE | C_LIT_QUOTE | C_EXE_BLOCK | C_SPACE | C_NEWLINE | C_SUPER | C_END_CMD | C_PIPE,
                ) => break,
                (_, _) => rendered.push(c),
            };

            escaped = set_escaped;
            set_escaped = false;

            self.increment();
        }

        self.decrement();
        return TokenComponent::String(rendered);
    }

    fn parse_lit_quoted(&mut self) -> TokenComponent {
        let mut rendered = String::new();

        // Skips leading quote
        self.increment();

        while !self.eof() {
            let c = self.cur_char();

            match c {
                C_LIT_QUOTE => break,
                _ => rendered.push(c),
            };

            self.increment();
        }

        // No backstep so that skips trailing quote
        return TokenComponent::Literal(rendered);
    }

    fn parse_quoted(&mut self) -> TokenComponent {
        let mut components = Vec::new();
        let mut quote = QuoteComponenet::default();

        // Skips leading quote
        self.increment();

        while !self.eof() {
            let c = self.cur_char();
            
            match c {
                C_SPACE => {
                    quote.push(Token { components });
                    components = Vec::new();
                }
                C_QUOTE => break,
                _ => components.push(self.parse_string()),
            };

            self.increment();
        }

        if components.len() > 0 {
            quote.push(Token { components });
        }

        // No backstep so that skips trailing quote
        return TokenComponent::Quote(quote);
    }
}