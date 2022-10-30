use crate::parser::parse;
use crate::runner::Runner;
use crate::token::Token;
use crate::tokenizer::tokenize;
use clap::Parser;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

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
        let tokens = tokenize(format!("( # {} )", path.to_str().expect("Invalid path!")).as_str());
        for token in &tokens {
            println!("{}", token);
        }
        let ast = parse(tokens);
        let value = Runner::run(Rc::new(RefCell::new(ast))).expect("Nothing returned :(");
        println!("Program returned: {}", value);
    } else {
        println!("File does not exist!")
    }
}
