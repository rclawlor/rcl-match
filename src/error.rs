use colored::Colorize;

#[derive(Debug)]
pub enum Error {
    MissingValue,
    InvalidNumber(String),
    InvalidPrefix(String),
    InvalidUnit(String),
}

impl Error {
    pub fn pprint(&self) {
        println!("{} {}", "Error:".red(), self.message());
    }

    fn message(&self) -> String {
        match self {
            Error::MissingValue => "Missing value".to_string(),
            Error::InvalidNumber(s) => format!("Invalid number '{}'", s),
            Error::InvalidPrefix(s) => format!("Invalid prefix '{s}'"),
            Error::InvalidUnit(s) => format!("Invalid unit '{s}'"),
        }
    }
}
