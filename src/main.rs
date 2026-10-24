mod component;
mod error;
mod matcher;
mod parser;
mod series;

use clap::Parser;

use crate::{
    component::{Component, Topology},
    matcher::find_values,
    parser::{Prefix, parse},
    series::ESeries,
};

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

    let (value, (a, b)) = find_values(&component, series, args.topology);
    let (a_scale, a_prefix) = Prefix::from_number(a);
    let (b_scale, b_prefix) = Prefix::from_number(b);
    println!(
        "{value}: {a_scale}{a_prefix}{}, {b_scale}{b_prefix}{}",
        component.component.unit(),
        component.component.unit()
    );
}
