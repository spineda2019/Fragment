use std::path::PathBuf;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut file_path: PathBuf = PathBuf::new();
    {
        let mut argument_parse: ArgumentParser = ArgumentParser::new();
        argument_parse.set_description("The Fragment Language Compiler");

        argument_parse
            .refer(&mut file_path)
            .add_argument("filename", Store, "File to compile");

        argument_parse.parse_args_or_exit();
    }

    println!("Compiling: {:?}", file_path);
}
