use crate::{
    component::{Component, Topology},
    series::ESeries,
};

pub fn find_values(
    component: &Component,
    series: ESeries,
    topology: Topology,
    precision: f32,
) -> Vec<(Component, Component, Component)> {
    let mut matcher = Matcher::new();
    let matches = matcher.find_matches(component, series, topology, precision);
    matches
        .iter()
        .map(|m| {
            (
                Component::new(m.value, component.kind, series),
                Component::new(m.a, component.kind, series),
                Component::new(m.b, component.kind, series),
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
            n_matches: 5,
        }
    }

    /// Find component matches
    fn find_matches(
        &mut self,
        c: &Component,
        series: ESeries,
        topology: Topology,
        precision: f32,
    ) -> Vec<CandidateMatch> {
        let target = c.value;
        let combine = c.kind.combine_for(topology);
        let candidates = series.candidates(target, combine);
        for (idx, a) in candidates.iter().enumerate() {
            for b in &candidates[idx..] {
                let result = combine.apply(a, b);
                let error = (result - target).abs();
                if 100.0 * (error / target) <= precision {
                    self.check_result(&CandidateMatch {
                        value: result,
                        error: (result - target).abs(),
                        a: *a,
                        b: *b,
                    });
                }
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
