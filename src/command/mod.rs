mod tests;
pub (crate) mod parser;

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Command {
    pub tokens: TokenSequence,
}

impl Default for Command {
    fn default() -> Self {
        return Command { tokens: Default::default()};
    }
}

pub trait TokenRender {
    fn render(&self) -> String;
}

#[derive(Debug)]
pub struct TokenSequence {
    pub tokens: Vec<Token>,
}

impl Default for TokenSequence {
    fn default() -> Self {
        return TokenSequence { tokens: Vec::new() };
    }
}

impl Deref for TokenSequence {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        return &self.tokens;
    }
}

impl DerefMut for TokenSequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.tokens;
    }
}

impl TokenRender for TokenSequence {
    fn render(&self) -> String {
        return self
            .tokens
            .iter()
            .fold(String::new(), |s, t| format!("{}{}", s, t.render()));
    }
}

#[derive(Debug)]
pub struct Token {
    pub components: Vec<TokenComponent>,
}

impl TokenRender for Token {
    fn render(&self) -> String {
        return self
            .components
            .iter()
            .fold(String::new(), |s, t| format!("{}{}", s, t.render()));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenComponent {
    String(String),
    Literal(String),
    Quote(QuoteComponenet),
}

impl TokenRender for TokenComponent {
    fn render(&self) -> String {
        return match self {
            TokenComponent::String(s) => s.clone(),
            TokenComponent::Literal(s) => format!("'{}'", s),
            TokenComponent::Quote(c) => c.render(),
        };
    }
}

#[derive(Debug)]
pub struct QuoteComponenet {
    tokens: TokenSequence,
}

impl Default for QuoteComponenet {
    fn default() -> Self {
        return QuoteComponenet { tokens: Default::default() };
    }
}

impl Deref for QuoteComponenet {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        return self.tokens.deref();
    }
}

impl DerefMut for QuoteComponenet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.tokens.deref_mut();
    }
}

impl TokenRender for QuoteComponenet {
    fn render(&self) -> String {
        return format!("\"{}\"", self.tokens.render());
    }
}

impl PartialEq for QuoteComponenet {
    fn eq(&self, other: &Self) -> bool {
        self.tokens.render() == other.tokens.render()
    }
}

impl Eq for QuoteComponenet {}
