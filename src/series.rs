use clap::ValueEnum;

use crate::component::Combine;

/// Round value to n decimal places
fn round(value: f32, n: usize) -> f32 {
    let scale: f32 = 10.0_f32.powf(n as f32);
    (value * scale).round() / scale
}

/// Select the decade window to search through
fn window(target: f32, combine: Combine) -> (i32, i32) {
    let d = target.log10().floor() as i32;
    match combine {
        Combine::Sum => (d - 2, d),
        Combine::Reciprocal => (d, d + 3),
    }
}

/// E24 series values.
///
/// Series from E3 to E24 do not follow the exact mathematical formula and must be stored.
const E24_BASE: [f32; 24] = [
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6,
    6.2, 6.8, 7.5, 8.2, 9.1,
];

#[derive(ValueEnum, Clone, Copy)]
#[clap(rename_all = "verbatim")]
pub enum ESeries {
    E3,
    E6,
    E12,
    E24,
    E48,
    E96,
    E192,
}

impl ESeries {
    pub fn values(&self) -> Vec<f32> {
        match self {
            ESeries::E3 => E24_BASE.iter().step_by(8).copied().collect(),
            ESeries::E6 => E24_BASE.iter().step_by(4).copied().collect(),
            ESeries::E12 => E24_BASE.iter().step_by(2).copied().collect(),
            ESeries::E24 => E24_BASE.to_vec(),
            _ => {
                let num_steps = self.steps();
                (0..num_steps)
                    .map(|n| {
                        round(
                            10.0_f32.powf((n as f32) / (num_steps as f32)),
                            self.decimal_places(),
                        )
                    })
                    .collect()
            }
        }
    }

    /// Find candidates to search through
    pub fn candidates(&self, target: f32, combine: Combine) -> Vec<f32> {
        let mut output = Vec::new();
        let (lo, hi) = window(target, combine);
        let base = self.values();
        for d in lo..=hi {
            for b in base.iter() {
                output.push(b * 10f32.powi(d));
            }
        }

        match combine {
            Combine::Sum => {
                // Values can't exceed target
                output
                    .into_iter()
                    .filter(|&x| x <= target * 1.001)
                    .collect()
            }
            Combine::Reciprocal => {
                // Values can't be less than target
                output
                    .into_iter()
                    .filter(|&x| x >= target * 0.999)
                    .collect()
            }
        }
    }

    fn steps(&self) -> usize {
        match self {
            ESeries::E3 => 3,
            ESeries::E6 => 6,
            ESeries::E12 => 12,
            ESeries::E24 => 24,
            ESeries::E48 => 48,
            ESeries::E96 => 96,
            ESeries::E192 => 192,
        }
    }

    /// Get number of decimal places for E series values between 1.0 to 9.9
    fn decimal_places(&self) -> usize {
        match self {
            ESeries::E3 | ESeries::E6 | ESeries::E12 | ESeries::E24 => 1,
            ESeries::E48 | ESeries::E96 | ESeries::E192 => 2,
        }
    }
}
