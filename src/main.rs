mod component;
mod display;
mod error;
mod matcher;
mod math;
mod parser;
mod series;

use clap::Parser;

use crate::{
    component::{Component, ComponentKind, Topology},
    display::{Draw, ParallelCircuit, SeriesCircuit, Table},
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
    let (value, kind) = match parse(&args.value) {
        Ok(result) => result,
        Err(e) => {
            e.pprint();
            std::process::exit(1);
        }
    };

    // Different components have different default series
    let series = if let Some(s) = args.eseries {
        s
    } else {
        match kind {
            ComponentKind::Resistor => ESeries::E24,
            ComponentKind::Capacitor | ComponentKind::Inductor => ESeries::E12,
        }
    };

    let component = Component::new(value, kind, series);
    let precision = args.precision.unwrap_or(5.0);

    let values = find_values(&component, series, args.topology, precision);
    let data: Vec<Vec<String>> = values
        .iter()
        .map(|(v, a, b)| {
            vec![
                format!("{}", v.pprint_precise()),
                format!("{a}"),
                format!("{b}"),
            ]
        })
        .collect();

    let schematic = match args.topology {
        Topology::Series => {
            let circuit = SeriesCircuit::from_kind(component.kind, 2);
            circuit.draw()
        }
        Topology::Parallel => {
            let circuit = ParallelCircuit::from_kind(component.kind, 2);
            circuit.draw()
        }
    };

    let table = Table {
        headers: vec![
            "Value".to_string(),
            format!("{}1", component.kind.symbol()),
            format!("{}2", component.kind.symbol()),
        ],
        data,
    };

    // Print output to terminal
    println!();
    for row in schematic {
        println!(" {row}");
    }
    println!();
    for row in table.pprint() {
        println!(" {row}")
    }
    println!();
}
