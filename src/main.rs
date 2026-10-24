mod error;
mod parser;

use clap::Parser;

use crate::parser::parse;

#[derive(Parser)]
struct Args {
    value: String,
}

fn main() {
    let args = Args::parse();
    let (_value, _component) = match parse(&args.value) {
        Ok(result) => result,
        Err(e) => {
            e.pprint();
            std::process::exit(1);
        }
    };
}
