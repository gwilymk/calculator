use std::fs;

use insta::{assert_ron_snapshot, glob};

use crate::lexer::Lexer;

#[test]
fn lexer_tests() {
    glob!("snapshot_tests", "lexer/*.toy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&input);
        let output = lexer
            .map(|token| match token {
                Ok((_, token, _)) => Ok(token),
                Err(e) => Err(format!("{e}")),
            })
            .collect::<Vec<_>>();

        assert_ron_snapshot!(output);
    });
}
