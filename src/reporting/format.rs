use std::io::{self, Write};

use ariadne::{Label, Report, ReportKind, Source};

use crate::{ast::Location, tokens::LexicalError};

use super::{Message, ParseError};

pub trait WriteDiagnostic {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str) -> io::Result<()>;
}

impl WriteDiagnostic for Message {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str) -> io::Result<()> {
        let report = match self {
            Message::LexerError(lexical_error, location) => {
                return (lexical_error.clone(), *location).write_diagnostic(w, code)
            }
            Message::ParseError(parse_error) => return parse_error.write_diagnostic(w, code),
            Message::UnknownVariable(variable_name, location) => {
                Report::build(ReportKind::Error, "myscript.toy", 0)
                    .with_label(
                        Label::new(("myscript.toy", location.as_range()))
                            .with_message("Unknown variable"),
                    )
                    .with_message(format!("Unknown variable {variable_name}"))
            }
        };

        report
            .finish()
            .write_for_stdout(("myscript.toy", Source::from(code)), w)
    }
}

impl WriteDiagnostic for ParseError {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str) -> io::Result<()> {
        let report = match self {
            ParseError::UnrecognizedEof { location, expected } => {
                Report::build(ReportKind::Error, "myscript.toy", 0)
                    .with_label(
                        Label::new(("myscript.toy", location.as_range()))
                            .with_message("End of file not expected here"),
                    )
                    .with_message("Unexpected end of file")
                    .with_note(format!(
                        "Note: expected one of tokens {}",
                        expected.join(", ")
                    ))
            }
            ParseError::UnrecognisedToken {
                location,
                token,
                expected,
            } => Report::build(ReportKind::Error, "myscript.toy", 0)
                .with_label(
                    Label::new(("myscript.toy", location.as_range()))
                        .with_message(format!("Unexpected token {token}")),
                )
                .with_message(format!(
                    "Unexpected token {token}, expected one of {}",
                    expected.join(", ")
                )),
            ParseError::ExtraToken { location, token } => {
                Report::build(ReportKind::Error, "myscript.toy", 0).with_label(
                    Label::new(("myscript.toy", location.as_range()))
                        .with_message(format!("Extra token {token}"))
                        .with_message(format!("Unexpected extra token {token}")),
                )
            }
        };

        report
            .finish()
            .write_for_stdout(("myscript.toy", Source::from(code)), w)
    }
}

impl WriteDiagnostic for (Box<LexicalError>, Location) {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str) -> io::Result<()> {
        let (lexical_error, location) = self;

        let report = match **lexical_error {
            LexicalError::InvaidInteger(ref parse_int_error) => {
                Report::build(ReportKind::Error, "myscript.toy", 0)
                    .with_label(
                        Label::new(("myscript.toy", location.as_range()))
                            .with_message("Invalid integer"),
                    )
                    .with_message(format!("{parse_int_error}"))
            }
            LexicalError::InvalidToken => Report::build(ReportKind::Error, "myscript.toy", 0)
                .with_label(
                    Label::new(("myscript.toy", location.as_range())).with_message("Invalid token"),
                ),
        };

        report
            .finish()
            .write_for_stdout(("myscript.toy", Source::from(code)), w)
    }
}
