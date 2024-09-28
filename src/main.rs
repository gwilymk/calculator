use std::ops::Range;

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

    let ast = parser.parse(lexer);

    match ast {
        Ok(ast) => {
            if let Err(e) = exec::execute(ast) {
                if let Some(err_message) = e.message() {
                    let mut error_report =
                        ariadne::Report::build(ariadne::ReportKind::Error, "myscript.toy", 0);
                    error_report = error_report.with_label(
                        Label::new(("myscript.toy", e.location().as_range()))
                            .with_message(err_message),
                    );

                    error_report
                        .finish()
                        .eprint(("myscript.toy", Source::from(source_code)))
                        .unwrap();
                } else if let Some(error_recovery) = e.error_recovery() {
                    error_report(
                        error_recovery.error,
                        Some(e.location().as_range()),
                        &source_code,
                    );
                }
            }
        }
        Err(ast_err) => error_report(ast_err, None, &source_code),
    }
}

fn error_report(
    error: lalrpop_util::ParseError<usize, tokens::Token, tokens::LexicalError>,
    err_location: Option<Range<usize>>,
    source_code: &str,
) {
    let mut error_report = ariadne::Report::build(ariadne::ReportKind::Error, "myscript.toy", 20);

    match error {
        lalrpop_util::ParseError::InvalidToken { location } => {
            error_report = error_report.with_label(
                Label::new((
                    "myscript.toy",
                    err_location.unwrap_or(location..location + 1),
                ))
                .with_message("Invalid token"),
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
                Label::new(("myscript.toy", err_location.unwrap_or(start..end)))
                    .with_message("Unexpected extra token"),
            );
        }
        lalrpop_util::ParseError::User { error } => {
            if let Some(location) = err_location {
                error_report = error_report.with_label(
                    Label::new(("myscript.toy", location)).with_message(format!("{error}")),
                );
            } else {
                error_report = error_report.with_message(format!("{error}"));
            }
        }
    }

    error_report
        .finish()
        .eprint(("myscript.toy", Source::from(source_code)))
        .unwrap();
}
