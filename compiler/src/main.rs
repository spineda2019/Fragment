use std::path::PathBuf;

use argparse::{ArgumentParser, Collect, StoreTrue};
use common::error::CompilerError;
use lexer::lexer::Lexer;
use parser::ast::Ast;

fn main() -> Result<(), CompilerError> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut lex_only: bool = false;
    let mut verbose: bool = false;

    {
        let mut argument_parse: ArgumentParser = ArgumentParser::new();
        argument_parse.set_description("The Fragment Language Compiler");

        argument_parse
            .refer(&mut files)
            .add_argument("filename", Collect, "File to compile");

        argument_parse.refer(&mut lex_only).add_option(
            &["-l", "--lex"],
            StoreTrue,
            "Indicate to only lex and display tokens",
        );

        argument_parse.refer(&mut verbose).add_option(
            &["--verbose"],
            StoreTrue,
            "Increase printing info",
        );

        argument_parse.parse_args_or_exit();
    }

    let mut lexer: Lexer = Lexer::init();

    if lex_only {
        println!("Only lexing files...");

        for file in files {
            lexer.new_file(file)?;
            let tokens = lexer.lex()?;
            for token in tokens {
                println!("{:?}", token);
            }
        }
    } else if files.is_empty() {
        println!("Welcome to the Fragment REPL!");
        lexer.lex()?;
    } else {
        println!("Compiling files: {:?}\n", &files);
        for file in files {
            lexer.new_file(file)?;
            let mut ast: Ast = Ast::new(&mut lexer, verbose);
            ast.parse_tokens()?;
        }
    }

    Ok(())
}
