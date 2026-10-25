use clap::ValueEnum;

use crate::{math::round, parser::Prefix, series::ESeries};

pub struct Component {
    pub value: f32,
    pub kind: ComponentKind,
    series: ESeries,
}

impl Component {
    pub fn new(value: f32, kind: ComponentKind, series: ESeries) -> Self {
        Component {
            value,
            kind,
            series,
        }
    }

    /// Print value to 3 decimal places
    pub fn pprint_precise(&self) -> String {
        let (scale, prefix) = Prefix::from_number(self.value);
        let rounded = round(scale, 3);

        format!("{}{}{}", rounded, prefix, self.kind.unit())
    }

    /// Print value to number of S.F. based on E series
    fn pprint(&self) -> String {
        let (scale, prefix) = Prefix::from_number(self.value);
        if scale == 0.0 || !scale.is_finite() {
            return format!("{}{}{}", scale, prefix, self.kind.unit());
        }

        let sf = self.series.significant_figures();

        // Scientific notation with `sf - 1` digits after the point does the rounding
        let sci = format!("{:.*e}", sf - 1, scale);
        let (_, exp) = sci.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        let rounded: f64 = sci.parse().unwrap();

        // Decimal places needed to show exactly `sig` significant digits
        let decimals = (sf as i32 - 1 - exp).max(0) as usize;
        format!("{:.*}{}{}", decimals, rounded, prefix, self.kind.unit())
    }
}

impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pprint())
    }
}

#[derive(Clone, Copy)]
pub enum ComponentKind {
    Resistor,
    Capacitor,
    Inductor,
}

impl ComponentKind {
    pub fn combine_for(&self, topology: Topology) -> Combine {
        match (self, topology) {
            (ComponentKind::Resistor | ComponentKind::Inductor, Topology::Series) => Combine::Sum,
            (ComponentKind::Resistor | ComponentKind::Inductor, Topology::Parallel) => {
                Combine::Reciprocal
            }
            (ComponentKind::Capacitor, Topology::Series) => Combine::Reciprocal,
            (ComponentKind::Capacitor, Topology::Parallel) => Combine::Sum,
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            ComponentKind::Resistor => "Ohms",
            ComponentKind::Capacitor => "F",
            ComponentKind::Inductor => "H",
        }
    }
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Topology {
    #[value(alias = "s")]
    Series,
    #[value(alias = "p")]
    Parallel,
}

#[derive(Clone, Copy)]
pub enum Combine {
    Sum,
    Reciprocal,
}

impl Combine {
    /// Apply combination method to values
    pub fn apply(&self, a: &f32, b: &f32) -> f32 {
        match self {
            Self::Sum => a + b,
            Self::Reciprocal => (a.recip() + b.recip()).recip(),
        }
    }
}
