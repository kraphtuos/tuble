use crate::*;
use std::collections::{BTreeMap, HashMap};

mod entropy;
mod minimax;
mod size;

pub use entropy::EntropyOptimiser;
pub use minimax::MinimaxOptimiser;
pub use size::SizeOptimiser;

pub fn get_possible_states(
    target_station: &Station,
    possible_stations: &[Station],
) -> BTreeMap<Outcome, Vec<Station>> {
    let mut map: BTreeMap<Outcome, Vec<Station>> = BTreeMap::new();
    for &station in possible_stations {
        map.entry(target_station.get_outcome(&station))
            .or_default()
            .push(station);
    }
    map
}

pub trait Score: Copy + std::fmt::Display + Ord {} // Higher scores are better

pub struct Output<S: Score> {
    pub station: Station,
    pub score: S,
}

pub trait Optimiser {
    const NAME: &'static str;

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Score>;
}

pub fn format_output<O: Optimiser>(station: &Station, score: &impl Score, text: &str) -> String {
    format!("{} {}: {} - {}", O::NAME, text, station, score)
}

pub fn optimise_and_output<O: Optimiser>(
    all_stations: &[Station],
    possible_stations: &[Station],
    text: &str,
) -> String {
    let Output { station, score } = O::optimise(all_stations, possible_stations);
    format_output::<O>(&station, &score, text)
}
