mod parser;

use clap::Parser;

#[derive(Parser)]
struct Args {
    value: String,
}

fn main() {
    let _args = Args::parse();
}
