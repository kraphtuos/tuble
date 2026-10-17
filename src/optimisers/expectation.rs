//! Optimise for minimal expected number of guesses by searching recursively.

use super::*;

/// Expected number of guesses, stored exactly as the total number of guesses
/// needed over `count` equally likely targets.
#[derive(Clone, Copy)]
pub struct Expectation {
    pub total: usize,
    pub count: usize,
}

impl std::fmt::Display for Expectation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}", self.total as f64 / self.count as f64)
    }
}

impl PartialEq for Expectation {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Expectation {}

impl PartialOrd for Expectation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expectation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.total * other.count).cmp(&(other.total * self.count))
    }
}

impl Cost for Expectation {}

#[derive(Default)]
pub struct ExpectationOptimiser {
    cache: RefCell<HashMap<StationSet, Output<Expectation>>>,
}

impl Optimiser for ExpectationOptimiser {
    const NAME: &'static str = "expectation";
    const COST: &'static str = "expected number of guesses";

    type Cost = Expectation;

    fn optimise(&self, possible_stations: StationSet) -> Output<Expectation> {
        let count = possible_stations.len();
        if count <= 2 {
            return Output {
                station: possible_stations.first().unwrap(),
                cost: Expectation {
                    total: count * (count + 1) / 2,
                    count,
                },
            };
        }
        if let Some(&res) = self.cache.borrow().get(&possible_stations) {
            return res;
        }
        let res = best_guess(possible_stations, |guess, best| {
            let possible = possible_stations.contains(guess);
            let mut rest = possible_stations;
            rest.remove(guess);
            let parts = partition(guess, rest);
            if !possible && parts.len() == 1 {
                return None;
            }
            // Every target costs this guess and every target it misses at
            // least one more, refined below with the exact cost of each part.
            let mut total = count + rest.len();
            for part in parts.into_values() {
                total += self.optimise(part).cost.total - part.len();
                if !best.could_beat(Expectation { total, count }, possible) {
                    return None;
                }
            }
            Some(Expectation { total, count })
        });
        self.cache.borrow_mut().insert(possible_stations, res);
        res
    }
}
