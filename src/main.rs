use std::io;

use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
use reporting::WriteDiagnostic;

mod ast;
mod exec;
mod lexer;
mod reporting;
mod tokens;

#[cfg(test)]
mod snapshot_tests;

lalrpop_mod!(pub grammar);

fn main() -> io::Result<()> {
    let source_code = std::fs::read_to_string("myscript.toy").unwrap();
    let lexer = Lexer::new(&source_code);
    let parser = grammar::ScriptParser::new();

    let ast = parser.parse(lexer);

    match ast {
        Ok(ast) => {
            if let Err(e) = exec::execute(ast) {
                e.write_diagnostic(io::stdout().lock(), &source_code, true)?;
            }
        }
        Err(ast_err) => {
            reporting::Message::from_lalrpop(ast_err, None).write_diagnostic(
                io::stdout().lock(),
                &source_code,
                true,
            )?;
        }
    }

    Ok(())
}
