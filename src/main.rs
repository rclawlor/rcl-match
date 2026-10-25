mod component;
mod error;
mod matcher;
mod parser;
mod series;

use clap::Parser;

use crate::{
    component::{Component, Topology},
    matcher::find_values,
    parser::parse,
    series::ESeries,
};

#[derive(Parser)]
struct Args {
    /// Target component value with units (e.g. 50pF, 5kOhm, 2uH)
    value: String,
    /// Topology of components
    #[arg(short = 't', long = "topology")]
    topology: Topology,
    /// E series values to choose from
    #[arg(short = 'e', long = "eseries")]
    eseries: Option<ESeries>,
    /// Percentage precision (default 5%)
    #[arg(short = 'p', long = "precision")]
    precision: Option<f32>,
}

fn main() {
    let args = Args::parse();
    let component = match parse(&args.value) {
        Ok(result) => result,
        Err(e) => {
            e.pprint();
            std::process::exit(1);
        }
    };

    let series = if let Some(s) = args.eseries {
        s
    } else {
        match component.component {
            Component::Resistor => ESeries::E24,
            Component::Capacitor | Component::Inductor => ESeries::E12,
        }
    };

    let precision = args.precision.unwrap_or(5.0);

    let values = find_values(&component, series, args.topology, precision);
    for value in values {
        println!("{}: {}, {}", value.0, value.1, value.2);
    }
}
