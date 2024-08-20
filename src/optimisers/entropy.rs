//! Optimise for maximal entropy.

use super::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Entropy(f64);

impl std::fmt::Display for Entropy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for Entropy {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Entropy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Cost for Entropy {}

pub struct EntropyOptimiser;

impl Optimiser for EntropyOptimiser {
    const NAME: &'static str = "entropy";

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Cost> {
        let mut max_entropy = f64::MIN;
        let mut best_guess = None;
        let mut possible_station_picked = false;

        let n = possible_stations.len() as f64;

        for station in all_stations {
            let entropy = -get_possible_states(station, possible_stations)
                .into_iter()
                .map(|(_, possible_stations)| {
                    let p = possible_stations.len() as f64 / n;
                    p * p.log2()
                })
                .sum::<f64>();

            if entropy > max_entropy {
                max_entropy = entropy;
                best_guess = Some(*station);
                possible_station_picked = possible_stations.contains(station);
            } else if (entropy == max_entropy)
                && !possible_station_picked
                && possible_stations.contains(station)
            {
                possible_station_picked = true;
                best_guess = Some(*station);
            }
        }

        Output {
            station: best_guess.unwrap(),
            cost: Entropy(max_entropy),
        }
    }
}
