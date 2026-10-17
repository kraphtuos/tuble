//! Optimise for maximal entropy.

use super::*;

#[derive(Clone, Copy)]
struct Entropy(f64);

impl std::fmt::Display for Entropy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Cost> {
        let mut min_entropy = f64::MAX;
        let mut best_guess = None;
        let mut possible_station_picked = false;

        let n = possible_stations.len() as f64;

        for station in all_stations {
            let entropy = get_possible_states(station, possible_stations)
                .into_values()
                .map(|possible_stations| {
                    let l = possible_stations.len() as f64;
                    l / n * l.log2()
                })
                .sum::<f64>();

            if entropy < min_entropy {
                min_entropy = entropy;
                best_guess = Some(*station);
                possible_station_picked = possible_stations.contains(station);
            } else if (entropy == min_entropy)
                && !possible_station_picked
                && possible_stations.contains(station)
            {
                possible_station_picked = true;
                best_guess = Some(*station);
            }
        }

        Output {
            station: best_guess.unwrap(),
            cost: Entropy(min_entropy),
        }
    }
}
