use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy)]
pub enum Topology {
    #[value(alias = "s")]
    Series,
    #[value(alias = "p")]
    Parallel,
}
