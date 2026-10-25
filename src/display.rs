use crate::component::ComponentKind;

///  Methods for locating a node relative to others, alongside
///  rendering the node
pub trait Draw {
    /// The number of lines from this element's top to where the entering,
    /// connecting path is drawn.
    fn entry_height(&self) -> usize;

    /// This primitives's total height.
    fn height(&self) -> usize;

    /// Draw this element.
    fn draw(&self) -> Vec<String>;
}

pub struct SeriesCircuit {
    pub components: Vec<Box<dyn Draw>>,
}

impl SeriesCircuit {
    pub fn from_kind(kind: ComponentKind, num: usize) -> Self {
        let components = match kind {
            ComponentKind::Resistor => (0..num)
                .map(|idx| Box::new(ResistorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
            ComponentKind::Capacitor => (0..num)
                .map(|idx| Box::new(CapacitorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
            ComponentKind::Inductor => (0..num)
                .map(|idx| Box::new(InductorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
        };

        SeriesCircuit { components }
    }
}

impl Draw for SeriesCircuit {
    fn entry_height(&self) -> usize {
        self.components
            .first()
            .map(|c| c.entry_height())
            .unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.components.first().map(|c| c.height()).unwrap_or(1)
    }

    fn draw(&self) -> Vec<String> {
        let mut output: Vec<Vec<String>> = (0..self.height()).map(|_| Vec::new()).collect();
        for c in &self.components {
            let out = c.draw();
            for (idx, s) in out.into_iter().enumerate() {
                output[idx].push(s);
            }
        }

        output.iter().map(|x| x.join("")).collect()
    }
}

pub struct ParallelCircuit {
    pub components: Vec<Box<dyn Draw>>,
}

impl ParallelCircuit {
    /// True if the circuit contains an even number of components
    fn is_even(&self) -> bool {
        self.components.len().is_multiple_of(2)
    }

    pub fn from_kind(kind: ComponentKind, num: usize) -> Self {
        let components = match kind {
            ComponentKind::Resistor => (0..num)
                .map(|idx| Box::new(ResistorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
            ComponentKind::Capacitor => (0..num)
                .map(|idx| Box::new(CapacitorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
            ComponentKind::Inductor => (0..num)
                .map(|idx| Box::new(InductorSymbol { id: idx + 1 }) as Box<dyn Draw>)
                .collect(),
        };

        ParallelCircuit { components }
    }
}

impl Draw for ParallelCircuit {
    fn entry_height(&self) -> usize {
        let mid = self.components.len() / 2;
        if self.is_even() {
            self.components.iter().take(mid).map(|c| c.height()).sum()
        } else {
            self.components
                .iter()
                .take(mid)
                .map(|c| c.height())
                .sum::<usize>()
                + self.components[mid].entry_height()
        }
    }

    fn height(&self) -> usize {
        self.components.iter().map(|c| c.height()).sum()
    }

    fn draw(&self) -> Vec<String> {
        let mut output = Vec::new();
        let mid = self.components.len() / 2;
        for (idx, c) in self.components.iter().enumerate() {
            let mut render = c.draw();
            match idx {
                0 => {
                    // Top component - add tracks downwards
                    render[c.entry_height()] = format!("┌{}┐", render[c.entry_height()]);
                }
                n if n == self.components.len() - 1 => {
                    // Bottom component - add tracks upwards
                    render[c.entry_height()] = format!("└{}┘", render[c.entry_height()]);
                }
                n if n == mid && !self.is_even() => {
                    // Middle component - add tracks each way
                    render[c.entry_height()] = format!("┼{}┼", render[c.entry_height()]);
                }
                _ => {
                    render[c.entry_height()] = format!("├{}┤", render[c.entry_height()]);
                }
            }

            for (row, s) in render.iter_mut().enumerate() {
                if self.is_even() && output.len() == self.entry_height() {
                    // Add connection points if not aligned with components
                    *s = format!("┤{s}├");
                } else if (row > c.entry_height() && idx == 0)
                    || (row < c.entry_height() && idx == self.components.len() - 1)
                    || (row != c.entry_height() && idx != 0 && idx != self.components.len() - 1)
                {
                    // Pad lines for bottom of top component, top of bottom component, all others
                    // that aren't entry points
                    *s = format!("│{s}│");
                } else if row != c.entry_height() {
                    // Pad spaced for top and bottom components
                    *s = format!(" {s} ");
                }

                output.push(s.to_string());
            }
        }

        // Add extra tracking to entry/exit points
        output
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                if idx == self.entry_height() {
                    format!("─{s}─")
                } else {
                    format!(" {s} ")
                }
            })
            .collect()
    }
}

/// The schematic symbol for a capacitor
///
///    CX
///    ╷╷
/// ───┤├───
///    ╵╵
///
pub struct CapacitorSymbol {
    pub id: usize,
}

impl Draw for CapacitorSymbol {
    fn entry_height(&self) -> usize {
        2
    }

    fn height(&self) -> usize {
        4
    }

    fn draw(&self) -> Vec<String> {
        vec![
            format!("   C{:<4}", self.id),
            "   ╷╷   ".to_string(),
            "───┤├───".to_string(),
            "   ╵╵   ".to_string(),
        ]
    }
}

/// The schematic symbol for a resistor
///
///      RX
/// ──╱╲╱╲╱╲╱──
///
struct ResistorSymbol {
    id: usize,
}

impl Draw for ResistorSymbol {
    fn entry_height(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        2
    }

    fn draw(&self) -> Vec<String> {
        vec![format!("     R{:<5}", self.id), "──╱╲╱╲╱╲╱──".to_string()]
    }
}

/// The schematic symbol for an inductor
///
///      LX
/// ──◠ ◠ ◠ ◠ ──
///
pub struct InductorSymbol {
    pub id: usize,
}

impl Draw for InductorSymbol {
    fn entry_height(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        2
    }

    fn draw(&self) -> Vec<String> {
        vec![format!("     L{:<6}", self.id), "──◠ ◠ ◠ ◠ ──".to_string()]
    }
}
