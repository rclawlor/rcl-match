mod error;
mod matcher;
mod parser;
mod series;

use clap::Parser;

use crate::{matcher::Topology, parser::parse, series::ESeries};

#[derive(Parser)]
struct Args {
    /// Target component value
    value: String,
    /// Topology of components
    #[arg(short = 't', long = "topology")]
    topology: Topology,
    /// E series values to choose from
    #[arg(short = 'e', long = "eseries")]
    eseries: Option<ESeries>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args.eseries.unwrap().values());
    let (_value, _component) = match parse(&args.value) {
        Ok(result) => result,
        Err(e) => {
            e.pprint();
            std::process::exit(1);
        }
    };
}
