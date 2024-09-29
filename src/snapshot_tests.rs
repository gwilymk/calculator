use std::fs;

use insta::{assert_ron_snapshot, assert_snapshot, glob};

use crate::{
    grammar::ScriptParser,
    lexer::Lexer,
    reporting::{self, WriteDiagnostic},
};

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

#[test]
fn reporting_tests() {
    glob!("snapshot_tests", "reporting/*.toy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&input);
        let parser = ScriptParser::new();

        let mut errors = vec![];

        match parser.parse(lexer) {
            Ok(output) => {
                for statement in output {
                    statement.append_errors(&mut errors);
                }
            }
            Err(err) => errors.push(reporting::Message::from_lalrpop(err, None)),
        };

        let mut output = vec![];
        for error in errors {
            error.write_diagnostic(&mut output, &input, false).unwrap();
        }

        assert_snapshot!(String::from_utf8(output).unwrap());
    });
}
