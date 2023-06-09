use std::ops::{Deref, DerefMut};

use crate::pipeline::Pipeline;

pub struct Script {
    commands: Vec<Pipeline>
}

impl Default for Script {
    fn default() -> Self {
        Self { commands: Default::default() }
    }
}

impl Deref for Script {
    type Target = Vec<Pipeline>;

    fn deref(&self) -> &Self::Target {
        return &self.commands;
    }
}

impl DerefMut for Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.commands;
    }
}