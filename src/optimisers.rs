use crate::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

mod entropy;
mod expectation;
mod minimax;
mod size;

pub use entropy::EntropyOptimiser;
pub use expectation::ExpectationOptimiser;
pub use minimax::MinimaxOptimiser;
pub use size::SizeOptimiser;

/// Groups `possible_stations` by the outcome they would give for `guess`.
pub fn partition(guess: Station, possible_stations: StationSet) -> BTreeMap<Outcome, StationSet> {
    let mut map: BTreeMap<Outcome, StationSet> = BTreeMap::new();
    for station in possible_stations.iter() {
        map.entry(guess.get_outcome(&station))
            .or_default()
            .insert(station);
    }
    map
}

pub trait Cost: Copy + std::fmt::Display + Ord {}

#[derive(Clone, Copy)]
pub struct Output<C: Cost> {
    pub station: Station,
    pub cost: C,
}

pub trait Optimiser {
    const NAME: &'static str;
    /// What the cost means, for display.
    const COST: &'static str;

    type Cost: Cost;

    /// Picks the best guess when the answer is one of `possible_stations`,
    /// which must not be empty.
    fn optimise(&self, possible_stations: StationSet) -> Output<Self::Cost>;
}

/// The lowest cost guess seen so far, preferring guesses that could be the
/// answer on ties.
struct Best<C> {
    best: Option<(Station, C, bool)>,
}

impl<C: Cost> Best<C> {
    /// Whether a guess with this cost would replace the current best.
    fn could_beat(&self, cost: C, possible: bool) -> bool {
        match self.best {
            None => true,
            Some((_, best_cost, best_possible)) => {
                cost < best_cost || (cost == best_cost && possible && !best_possible)
            }
        }
    }
}

/// Picks the guess out of all stations with the lowest cost. `cost` may
/// return `None` to skip a guess, e.g. once it can no longer beat the best.
fn best_guess<C: Cost>(
    possible_stations: StationSet,
    mut cost: impl FnMut(Station, &Best<C>) -> Option<C>,
) -> Output<C> {
    let mut best = Best { best: None };
    for station in Station::all() {
        let Some(c) = cost(station, &best) else {
            continue;
        };
        let possible = possible_stations.contains(station);
        if best.could_beat(c, possible) {
            best.best = Some((station, c, possible));
        }
    }
    let (station, cost, _) = best.best.expect("no informative guess");
    Output { station, cost }
}

/// An object safe view of an [`Optimiser`] with its costs formatted.
pub trait AnyOptimiser {
    fn name(&self) -> &'static str;
    fn cost_description(&self) -> &'static str;
    fn best_guess(&self, possible_stations: StationSet) -> (Station, String);
    /// The outcome of `guess` that leaves the costliest stations to solve.
    fn worst_outcome(&self, guess: Station, possible_stations: StationSet) -> (Outcome, String);
}

impl<O: Optimiser> AnyOptimiser for O {
    fn name(&self) -> &'static str {
        O::NAME
    }

    fn cost_description(&self) -> &'static str {
        O::COST
    }

    fn best_guess(&self, possible_stations: StationSet) -> (Station, String) {
        let Output { station, cost } = self.optimise(possible_stations);
        (station, cost.to_string())
    }

    fn worst_outcome(&self, guess: Station, possible_stations: StationSet) -> (Outcome, String) {
        let (outcome, cost) = partition(guess, possible_stations)
            .into_iter()
            .map(|(outcome, stations)| (outcome, self.optimise(stations).cost))
            .max_by_key(|&(_, cost)| cost)
            .unwrap();
        (outcome, cost.to_string())
    }
}

pub fn all_optimisers() -> Vec<Box<dyn AnyOptimiser>> {
    vec![
        Box::new(MinimaxOptimiser::default()),
        Box::new(SizeOptimiser),
        Box::new(EntropyOptimiser),
        Box::new(ExpectationOptimiser::default()),
    ]
}

/// The number of guesses `optimiser` takes to find `target`.
pub fn guesses_to_find(optimiser: &dyn AnyOptimiser, target: Station) -> usize {
    let mut possible_stations = StationSet::all();
    for guesses in 1.. {
        let (guess, _) = optimiser.best_guess(possible_stations);
        if guess == target {
            return guesses;
        }
        possible_stations = partition(guess, possible_stations)[&guess.get_outcome(&target)];
    }
    unreachable!()
}
