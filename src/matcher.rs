use crate::{
    component::Topology,
    parser::{ComponentValue, Prefix},
    series::ESeries,
};

pub fn find_values(
    component: &ComponentValue,
    series: ESeries,
    topology: Topology,
) -> Vec<(String, String, String)> {
    let mut matcher = Matcher::new();
    let matches = matcher.find_matches(component, series, topology);
    let unit = component.component.unit();
    matches
        .iter()
        .map(|m| {
            let (v_scale, v_prefix) = Prefix::from_number(m.value);
            let (a_scale, a_prefix) = Prefix::from_number(m.a);
            let (b_scale, b_prefix) = Prefix::from_number(m.b);
            (
                format!("{v_scale}{v_prefix}{unit}"),
                format!("{a_scale}{a_prefix}{unit}"),
                format!("{b_scale}{b_prefix}{unit}"),
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
struct CandidateMatch {
    value: f32,
    error: f32,
    a: f32,
    b: f32,
}

struct Matcher {
    best: Vec<CandidateMatch>,
    n_matches: usize,
}

impl Matcher {
    fn new() -> Self {
        Matcher {
            best: Vec::new(),
            n_matches: 2,
        }
    }

    /// Find component matches
    fn find_matches(
        &mut self,
        c: &ComponentValue,
        series: ESeries,
        topology: Topology,
    ) -> Vec<CandidateMatch> {
        let target = c.value * c.prefix.multiplier();
        let combine = c.component.combine_for(topology);
        let candidates = series.candidates(target, combine);
        for (idx, a) in candidates.iter().enumerate() {
            for b in &candidates[idx..] {
                let result = combine.apply(a, b);
                self.check_result(&CandidateMatch {
                    value: result,
                    error: (result - target).abs(),
                    a: *a,
                    b: *b,
                });
            }
        }

        self.best.clone()
    }

    fn check_result(&mut self, result: &CandidateMatch) {
        if self.n_matches == 0 {
            return;
        }

        if self.best.len() == self.n_matches
            && let Some(worst) = self.best.last()
            && result.error.total_cmp(&worst.error).is_ge()
        {
            return;
        }

        // Results with equal error keep their insertion order.
        let idx = self
            .best
            .partition_point(|p| p.error.total_cmp(&result.error).is_le());

        self.best.insert(idx, *result);
        self.best.truncate(self.n_matches);
    }
}
