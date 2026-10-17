//! Optimise for the size of the largest component.

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size(pub usize);

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Cost for Size {}

pub struct SizeOptimiser;

impl Optimiser for SizeOptimiser {
    const NAME: &'static str = "size";
    const COST: &'static str = "most stations left after the guess";

    type Cost = Size;

    fn optimise(&self, possible_stations: StationSet) -> Output<Size> {
        best_guess(possible_stations, |guess, _| {
            partition(guess, possible_stations)
                .values()
                .map(|part| Size(part.len()))
                .max()
        })
    }
}
