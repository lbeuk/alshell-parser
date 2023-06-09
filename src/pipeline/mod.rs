use std::ops::{Deref, DerefMut};

use crate::command::Command;

mod tests;
pub (crate) mod parser;

#[derive(Clone, Debug)]
pub struct Pipeline {
    commands: Vec<Command>
}

impl Default for Pipeline {
    fn default() -> Self {
        Self { commands: Default::default() }
    }
}

impl Deref for Pipeline {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        return &self.commands;
    }
}

impl DerefMut for Pipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.commands;
    }
}