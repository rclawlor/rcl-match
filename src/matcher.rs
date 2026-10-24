use crate::{component::Topology, parser::ComponentValue, series::ESeries};

pub fn find_values(
    component: &ComponentValue,
    series: ESeries,
    topology: Topology,
) -> (f32, (f32, f32)) {
    let mut matcher = Matcher::new();
    matcher.find_matches(component, series, topology)
}

struct Matcher {
    best: f32,
    values: (f32, f32),
}

impl Matcher {
    fn new() -> Self {
        Matcher {
            best: 0.0,
            values: (0.0, 0.0),
        }
    }

    /// Find component matches
    fn find_matches(
        &mut self,
        c: &ComponentValue,
        series: ESeries,
        topology: Topology,
    ) -> (f32, (f32, f32)) {
        let target = c.value * c.prefix.multiplier();
        let combine = c.component.combine_for(topology);
        let candidates = series.candidates(target, combine);
        for (idx, a) in candidates.iter().enumerate() {
            for b in &candidates[idx..] {
                let result = combine.apply(a, b);
                if (result - target).abs() < (self.best - target).abs() {
                    self.best = result;
                    self.values = (*a, *b);
                }
            }
        }

        (self.best, self.values)
    }
}
