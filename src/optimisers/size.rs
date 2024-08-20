//! Optimise for the size of the largest component

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Size(usize);

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Size {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Score for Size {}

pub struct SizeOptimiser;

impl Optimiser for SizeOptimiser {
    const NAME: &'static str = "size";

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Score> {
        let mut min_max_size = usize::MAX;
        let mut best_guess = None;
        let mut possible_station_picked = false;

        for station in all_stations {
            let max_size = get_possible_states(station, possible_stations)
                .iter()
                .map(|(_, possible_stations)| possible_stations.len())
                .max()
                .unwrap();

            if max_size < min_max_size {
                min_max_size = max_size;
                best_guess = Some(*station);
                possible_station_picked = possible_stations.contains(station);
            } else if (max_size == min_max_size)
                && !possible_station_picked
                && possible_stations.contains(station)
            {
                possible_station_picked = true;
                best_guess = Some(*station);
            }
        }

        Output {
            station: best_guess.unwrap(),
            score: Size(min_max_size),
        }
    }
}
