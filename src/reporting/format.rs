use std::{
    io::{self, Write},
    ops::Range,
};

use ariadne::{Config, Label, Report, ReportKind, Source};

use crate::{ast::Location, tokens::LexicalError};

use super::{Message, ParseError};

pub trait WriteDiagnostic {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str, include_colour: bool) -> io::Result<()>;
}

trait BuildDiagnostic {
    fn build_diagnostic<'a>(&self) -> ariadne::ReportBuilder<'static, (&'a str, Range<usize>)>;
}

impl WriteDiagnostic for Message {
    fn write_diagnostic<W: Write>(&self, w: W, code: &str, include_colour: bool) -> io::Result<()> {
        let report = match self {
            Message::LexerError(lexical_error, location) => {
                (lexical_error.clone(), *location).build_diagnostic()
            }
            Message::ParseError(parse_error) => parse_error.build_diagnostic(),
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
            .with_config(Config::default().with_color(include_colour))
            .finish()
            .write_for_stdout(("myscript.toy", Source::from(code)), w)
    }
}

impl BuildDiagnostic for ParseError {
    fn build_diagnostic<'a>(&self) -> ariadne::ReportBuilder<'static, (&'a str, Range<usize>)> {
        match self {
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
        }
    }
}

impl BuildDiagnostic for (Box<LexicalError>, Location) {
    fn build_diagnostic<'a>(&self) -> ariadne::ReportBuilder<'static, (&'a str, Range<usize>)> {
        let (lexical_error, location) = self;

        match **lexical_error {
            LexicalError::InvaidInteger(ref parse_int_error) => {
                Report::build(ReportKind::Error, "myscript.toy", 0)
                    .with_label(
                        Label::new(("myscript.toy", location.as_range()))
                            .with_message("Invalid integer"),
                    )
                    .with_message(format!("{parse_int_error}"))
                    .with_note(match parse_int_error.kind() {
                        std::num::IntErrorKind::PosOverflow => {
                            format!("Larger than maximum positive number which is {}", i64::MAX)
                        }
                        std::num::IntErrorKind::NegOverflow => {
                            format!("Smaller than minimum negative number which is {}", i64::MIN)
                        }
                        _ => String::default(),
                    })
            }
            LexicalError::InvalidToken => Report::build(ReportKind::Error, "myscript.toy", 0)
                .with_label(
                    Label::new(("myscript.toy", location.as_range())).with_message("Invalid token"),
                ),
        }
    }
}
