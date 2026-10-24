use clap::ValueEnum;

pub enum Component {
    Resistor,
    Capacitor,
    Inductor,
}

impl Component {
    pub fn combine_for(&self, topology: Topology) -> Combine {
        match (self, topology) {
            (Component::Resistor | Component::Inductor, Topology::Series) => Combine::Sum,
            (Component::Resistor | Component::Inductor, Topology::Parallel) => Combine::Reciprocal,
            (Component::Capacitor, Topology::Series) => Combine::Reciprocal,
            (Component::Capacitor, Topology::Parallel) => Combine::Sum,
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            Component::Resistor => "Ohms",
            Component::Capacitor => "F",
            Component::Inductor => "H",
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
