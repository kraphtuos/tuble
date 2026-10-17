//! Optimise for minimal expected number of guesses by searching recursively.

use super::*;

/// Expected number of guesses, stored exactly as the total number of guesses
/// needed over `count` equally likely targets.
#[derive(Clone, Copy)]
struct Expectation {
    total: usize,
    count: usize,
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

pub struct ExpectationOptimiser;

/// Returns the best guess and the total number of guesses needed to find
/// every station in `possible_stations`.
fn helper(
    all_stations: &[Station],
    possible_stations: &[Station],
    cache: &mut HashMap<Vec<Station>, (Station, usize)>,
) -> (Station, usize) {
    if possible_stations.len() == 1 {
        return (possible_stations[0], 1);
    }
    if possible_stations.len() == 2 {
        return (possible_stations[0], 3);
    }
    if let Some(res) = cache.get(possible_stations) {
        return *res;
    }

    let mut min_total = usize::MAX;
    let mut best_guess = None;
    let mut possible_station_picked = false;

    for station in all_stations {
        let possible_states = get_possible_states(station, possible_stations);
        if possible_states.len() == 1 {
            continue;
        }
        // Every target costs this guess, and the ones it doesn't hit cost the rest.
        let total = possible_stations.len()
            + possible_states
                .into_values()
                .filter(|possible_stations| possible_stations[..] != [*station])
                .map(|possible_stations| helper(all_stations, &possible_stations, cache).1)
                .sum::<usize>();

        if total < min_total {
            min_total = total;
            best_guess = Some(*station);
            possible_station_picked = possible_stations.contains(station);
        } else if (total == min_total)
            && !possible_station_picked
            && possible_stations.contains(station)
        {
            possible_station_picked = true;
            best_guess = Some(*station);
        }
    }
    let res = (best_guess.unwrap(), min_total);
    cache.insert(possible_stations.to_vec(), res);
    res
}

impl Optimiser for ExpectationOptimiser {
    const NAME: &'static str = "expectation";

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Cost> {
        let (station, total) = helper(all_stations, possible_stations, &mut HashMap::new());
        Output {
            station,
            cost: Expectation {
                total,
                count: possible_stations.len(),
            },
        }
    }
}
