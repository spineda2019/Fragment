use std::path::PathBuf;

use argparse::{ArgumentParser, Collect};

fn main() {
    let mut files: Vec<PathBuf> = Vec::new();
    {
        let mut argument_parse: ArgumentParser = ArgumentParser::new();
        argument_parse.set_description("The Fragment Language Compiler");

        argument_parse
            .refer(&mut files)
            .add_argument("filename", Collect, "File to compile");

        argument_parse.parse_args_or_exit();
    }

    println!("Compiling files:");
    for file in files {
        print!("{:?} ", file);
    }
}
