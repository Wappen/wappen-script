use crate::parser::parse;
use crate::runner::{Context, Runner};
use crate::token::Token;
use crate::tokenizer::tokenize;
use clap::Parser;
use std::path::Path;

mod node;
mod parser;
mod runner;
mod token;
mod tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a .ws file
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if path.is_file() {
        let value = Runner::run_file(path, &mut Context::default())
            .expect("Nothing returned :(")
            .unwrap();
        println!("Program returned: {}", value);
    } else {
        println!("File does not exist!")
    }
}
