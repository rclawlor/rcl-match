use crate::{component::Component, error::Error};
use std::str::CharIndices;

pub struct ComponentValue {
    pub value: f32,
    pub prefix: Prefix,
    pub component: Component,
}

pub fn parse(source: &str) -> Result<ComponentValue, Error> {
    let mut parser = Parser::new(source);
    parser.parse()
}

pub enum Prefix {
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
    /// No prefix
    None,
    /// k
    Kilo,
    /// M
    Mega,
}

impl Prefix {
    pub fn multiplier(&self) -> f32 {
        match self {
            Prefix::Femto => 1e-15,
            Prefix::Pico => 1e-12,
            Prefix::Nano => 1e-9,
            Prefix::Micro => 1e-6,
            Prefix::Milli => 1e-3,
            Prefix::None => 1.0,
            Prefix::Kilo => 1e3,
            Prefix::Mega => 1e6,
        }
    }

    pub fn from_number(n: f32) -> (f32, Prefix) {
        // Find prefix
        let exp: i32 = ((n.log10() / 3.0).floor() * 3.0) as i32;
        let prefix = match exp {
            -15 => Prefix::Femto,
            -12 => Prefix::Pico,
            -9 => Prefix::Nano,
            -6 => Prefix::Micro,
            -3 => Prefix::Milli,
            0 => Prefix::None,
            3 => Prefix::Kilo,
            6 => Prefix::Mega,
            n if n < -15 => Prefix::Femto,
            n if n > 6 => Prefix::Mega,
            _ => unreachable!("All numbers rounded to nearest multiple of 3"),
        };

        (n / prefix.multiplier(), prefix)
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Prefix::Femto => 'f',
            Prefix::Pico => 'p',
            Prefix::Nano => 'n',
            Prefix::Micro => 'u',
            Prefix::Milli => 'm',
            Prefix::None => ' ',
            Prefix::Kilo => 'k',
            Prefix::Mega => 'M',
        };
        write!(f, "{}", s)
    }
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

    fn parse(&mut self) -> Result<ComponentValue, Error> {
        let value = self.parse_number()?;
        let prefix = self.parse_prefix()?;
        let component = self.parse_component()?;

        Ok(ComponentValue {
            value,
            prefix,
            component,
        })
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

    fn parse_prefix(&mut self) -> Result<Prefix, Error> {
        if self.at_end() {
            Ok(Prefix::None)
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
            Ok(prefix)
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
