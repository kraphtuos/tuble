//! Optimise for the worst case number of guesses by searching recursively.

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Steps(pub usize);

impl std::fmt::Display for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Cost for Steps {}

#[derive(Default)]
pub struct MinimaxOptimiser {
    cache: RefCell<HashMap<StationSet, Output<Steps>>>,
}

impl Optimiser for MinimaxOptimiser {
    const NAME: &'static str = "minimax";
    const COST: &'static str = "worst case number of guesses";

    type Cost = Steps;

    fn optimise(&self, possible_stations: StationSet) -> Output<Steps> {
        if possible_stations.len() <= 2 {
            return Output {
                station: possible_stations.first().unwrap(),
                cost: Steps(possible_stations.len()),
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
            let mut worst = Steps(1);
            for part in parts.into_values() {
                worst = worst.max(Steps(self.optimise(part).cost.0 + 1));
                if !best.could_beat(worst, possible) {
                    return None;
                }
            }
            Some(worst)
        });
        self.cache.borrow_mut().insert(possible_stations, res);
        res
    }
}
