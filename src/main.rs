use ariadne::{Label, Source};
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

mod ast;
mod exec;
mod lexer;
mod tokens;

lalrpop_mod!(pub grammar);

fn main() {
    let source_code = std::fs::read_to_string("myscript.toy").unwrap();
    let lexer = Lexer::new(&source_code);
    let parser = grammar::ScriptParser::new();

    let mut errors = Vec::new();

    let ast = parser.parse(&mut errors, lexer);

    let any_errors = !errors.is_empty();

    for error in errors {
        error_report(error.error, &source_code);
    }

    match ast {
        Ok(ast) => {
            if !any_errors {
                if let Err(e) = exec::execute(ast) {
                    let mut error_report =
                        ariadne::Report::build(ariadne::ReportKind::Error, "myscript.toy", 0);
                    error_report = error_report.with_label(
                        Label::new(("myscript.toy", e.location().as_range()))
                            .with_message(format!("{e}")),
                    );

                    error_report
                        .finish()
                        .eprint(("myscript.toy", Source::from(source_code)))
                        .unwrap();
                }
            }
        }
        Err(ast_err) => error_report(ast_err, &source_code),
    }
}

fn error_report(
    error: lalrpop_util::ParseError<usize, tokens::Token, tokens::LexicalError>,
    source_code: &str,
) {
    let mut error_report = ariadne::Report::build(ariadne::ReportKind::Error, "myscript.toy", 20);

    match error {
        lalrpop_util::ParseError::InvalidToken { location } => {
            error_report = error_report.with_label(
                Label::new(("myscript.toy", location..location + 1)).with_message("Invalid token"),
            );
        }
        lalrpop_util::ParseError::UnrecognizedEof { expected, .. } => {
            error_report = error_report.with_label(
                Label::new(("myscript.toy", source_code.len() - 1..source_code.len()))
                    .with_message(format!(
                        "Unexpected eof, expected one of {expected}",
                        expected = expected.join(", ")
                    )),
            );
        }
        lalrpop_util::ParseError::UnrecognizedToken {
            token: (start, _token, end),
            expected,
        } => {
            error_report = error_report.with_label(
                Label::new(("myscript.toy", start..end)).with_message(format!(
                    "Unrecognised token, expected {expected}",
                    expected = expected.join(", ")
                )),
            );
        }
        lalrpop_util::ParseError::ExtraToken {
            token: (start, _token, end),
        } => {
            error_report = error_report.with_label(
                Label::new(("myscript.toy", start..end)).with_message("Unexpected extra token"),
            );
        }
        lalrpop_util::ParseError::User { error } => {
            error_report = error_report.with_message(format!("{error}"));
        }
    }

    error_report
        .finish()
        .eprint(("myscript.toy", Source::from(source_code)))
        .unwrap();
}
