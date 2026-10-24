use crate::error::Error;
use std::str::CharIndices;

pub fn parse(source: &str) -> Result<(f32, Component), Error> {
    let mut parser = Parser::new(source);
    parser.parse()
}

enum Prefix {
    /// f
    Femto,
    /// p
    Pico,
    /// n
    Nano,
    /// u
    Micro,
    /// m
    Milli,
    /// k
    Kilo,
    /// M
    Mega,
}

impl Prefix {
    fn multiplier(&self) -> f32 {
        match self {
            Prefix::Femto => 1e-15,
            Prefix::Pico => 1e-12,
            Prefix::Nano => 1e-9,
            Prefix::Micro => 1e-6,
            Prefix::Milli => 1e-3,
            Prefix::Kilo => 1e3,
            Prefix::Mega => 1e6,
        }
    }
}

pub enum Component {
    Resistor,
    Capacitor,
    Inductor,
}

struct Parser<'source> {
    source: &'source str,
    iter: CharIndices<'source>,
    c: char,
    pos: usize,
}

impl<'source> Parser<'source> {
    fn new(source: &'source str) -> Self {
        let mut parser = Parser {
            source,
            iter: source.char_indices(),
            c: '\0',
            pos: 0,
        };
        parser.advance();
        parser
    }

    fn parse(&mut self) -> Result<(f32, Component), Error> {
        let mut value = self.parse_number()?;
        if let Some(prefix) = self.parse_prefix()? {
            value *= prefix.multiplier();
        }
        let component = self.parse_component()?;

        Ok((value, component))
    }

    fn parse_number(&mut self) -> Result<f32, Error> {
        if self.peek_char().is_ascii_digit() {
            let mut chars = String::new();
            while self.peek_char().is_ascii_digit() || self.peek_char() == '.' {
                chars.push(self.next_char());
            }

            chars
                .parse()
                .map_err(|_| Error::InvalidNumber(chars.to_string()))
        } else {
            Err(Error::MissingValue)
        }
    }

    fn parse_prefix(&mut self) -> Result<Option<Prefix>, Error> {
        if self.at_end() {
            Ok(None)
        } else {
            let prefix = match self.next_char() {
                'f' => Prefix::Femto,
                'p' => Prefix::Pico,
                'n' => Prefix::Nano,
                'u' => Prefix::Micro,
                'm' => Prefix::Milli,
                'k' => Prefix::Kilo,
                'M' => Prefix::Mega,
                other => return Err(Error::InvalidPrefix(other.to_string())),
            };
            Ok(Some(prefix))
        }
    }

    fn parse_component(&mut self) -> Result<Component, Error> {
        if self.at_end() {
            Ok(Component::Resistor)
        } else {
            let unit = self.flush();
            let component = match unit.as_str() {
                "R" | "Ohm" | "Ohms" | "ohm" | "ohms" => Component::Resistor,
                "F" => Component::Capacitor,
                "H" => Component::Inductor,
                other => return Err(Error::InvalidUnit(other.to_string())),
            };
            self.advance();
            Ok(component)
        }
    }

    /// Consume and move to next char
    fn next_char(&mut self) -> char {
        let c = self.c;
        self.advance();
        c
    }

    /// Advance to next character in input
    fn advance(&mut self) {
        if let Some((pos, c)) = self.iter.next() {
            self.pos = pos;
            self.c = c;
        } else {
            self.pos = self.source.len();
            self.c = '\0';
        }
    }

    /// Check next char
    fn peek_char(&self) -> char {
        self.c
    }

    /// Check if source finished
    fn at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Flush all remaining chars
    fn flush(&mut self) -> String {
        let mut end = String::new();
        while !self.at_end() {
            end.push(self.next_char());
        }

        end
    }
}
