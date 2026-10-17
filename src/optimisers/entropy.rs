//! Optimise for maximal information gained, i.e. minimal expected entropy left.

use super::*;

#[derive(Clone, Copy)]
pub struct Entropy(pub f64);

impl std::fmt::Display for Entropy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

impl PartialEq for Entropy {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Entropy {}

impl PartialOrd for Entropy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entropy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Cost for Entropy {}

pub struct EntropyOptimiser;

impl Optimiser for EntropyOptimiser {
    const NAME: &'static str = "entropy";
    const COST: &'static str = "expected bits of entropy left after the guess";

    type Cost = Entropy;

    fn optimise(&self, possible_stations: StationSet) -> Output<Entropy> {
        let n = possible_stations.len() as f64;
        best_guess(possible_stations, |guess, _| {
            // Sum in a fixed order so equally good guesses tie exactly.
            let mut sizes: Vec<_> = partition(guess, possible_stations)
                .values()
                .map(|part| part.len())
                .collect();
            sizes.sort_unstable();
            let entropy = sizes
                .into_iter()
                .map(|l| {
                    let l = l as f64;
                    l / n * l.log2()
                })
                .sum::<f64>();
            Some(Entropy(entropy))
        })
    }
}
