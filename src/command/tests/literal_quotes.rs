use crate::{Parser, command::{parser::CommandParser, TokenComponent}};

#[test]
fn test_basic_literals() {
    let test_string = "'Hello there how' 'are you doing'";
    let components_resul: Vec<&str> = vec!["Hello there how", "are you doing"];

    let rp = Parser::with_text(test_string);
    let command = CommandParser::from(&rp).parse().unwrap();

    assert_eq!(command.tokens.len(), components_resul.len());
    for (real, test) in components_resul.iter().zip(command.tokens.iter()) {
        assert_eq!(test.components.len(), 1);
        assert_eq!(test.components[0], TokenComponent::Literal(String::from(*real)))
    }
}