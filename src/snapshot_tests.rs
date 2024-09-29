use std::fs;

use insta::{assert_ron_snapshot, glob};

use crate::{grammar::ScriptParser, lexer::Lexer};

#[test]
fn lexer_tests() {
    glob!("snapshot_tests", "lexer/*.toy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&input);
        let output = lexer
            .map(|token| token.map(|(_, token, _)| token))
            .collect::<Vec<_>>();

        assert_ron_snapshot!(output);
    });
}

#[test]
fn parser_tests() {
    glob!("snapshot_tests", "parser/*.toy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&input);
        let parser = ScriptParser::new();

        let output = parser.parse(lexer).unwrap();

        assert_ron_snapshot!(output);
    });
}
