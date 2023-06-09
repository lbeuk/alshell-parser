#![cfg(test)]

mod basic_strings;
mod literal_quotes;

// #[test]
// fn test_input() {
//     use crate::{Parser, command::parser::CommandParser};

//     let line = "hello there; what";
//     let rp = Parser::with_text(line);

//     let p = CommandParser::from(&rp);
//     let command = p.parse();

//     println!("INPUT: {}", line);
//     println!("COMMAND: {:?}", command);
// }