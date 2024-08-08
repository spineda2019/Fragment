use std::{fs::File, path::PathBuf};

use argparse::{ArgumentParser, Collect};
use common::error::CompilerError;
use lexer::lexer::Lexer;

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
        lexer.lex();
    } else {
        println!("Compiling files:");
        for file in files.iter() {
            println!("{:?} ", file);
            let file: File = match File::open(file) {
                Ok(f) => f,
                Err(e) => return Err(CompilerError::FileIOError(file.to_path_buf(), e)),
            };
            lexer.new_file(file);
            lexer.lex();
        }
    }

    Ok(())
}
