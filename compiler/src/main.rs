use std::{collections::VecDeque, path::PathBuf};

use argparse::{ArgumentParser, Collect};
use common::{error::CompilerError, token::Token};
use lexer::lexer::Lexer;
use parser::ast::Ast;

fn main() -> Result<(), CompilerError> {
    let mut files: Vec<PathBuf> = Vec::new();
    {
        let mut argument_parse: ArgumentParser = ArgumentParser::new();
        argument_parse.set_description("The Fragment Language Compiler");

        argument_parse
            .refer(&mut files)
            .add_argument("filename", Collect, "File to compile");

        argument_parse.parse_args_or_exit();
    }

    let mut lexer: Lexer = Lexer::init();

    if files.is_empty() {
        println!("Welcome to the Fragment REPL!");
        lexer.lex()?;
    } else {
        println!("Compiling files:");
        for file in files {
            println!("{:?} ", file);
            lexer.new_file(file)?;
            let tokens: VecDeque<Token> = lexer.lex()?;
            println!("We got tokens!!");
            for token in tokens.iter() {
                println!("\t{:#?}", token);
            }
            let ast: Ast = Ast::new(tokens);
        }
    }

    Ok(())
}
