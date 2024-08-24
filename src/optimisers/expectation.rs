//! Optimise for maximal expectation.

use super::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Expectation(f64);

impl std::fmt::Display for Expectation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for Expectation {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Expectation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Cost for Expectation {}

pub struct ExpectationOptimiser;

impl Optimiser for ExpectationOptimiser {
    const NAME: &'static str = "expectation";

    fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> Output<impl Cost> {
        fn helper(
            all_stations: &[Station],
            possible_stations: &[Station],
            cache: &mut HashMap<Vec<Station>, (Station, f64)>,
        ) -> (Station, f64) {
            if possible_stations.len() == 1 {
                return (possible_stations[0], 1.);
            }
            if possible_stations.len() == 2 {
                return (possible_stations[0], 2.);
            }
            if let Some(res) = cache.get(possible_stations) {
                return *res;
            }

            let mut min_expectation = f64::MAX;
            let mut best_guess = None;
            let mut possible_station_picked = false;

            for station in all_stations {
                let possible_states = get_possible_states(station, possible_stations);
                if possible_states.len() == 1 {
                    continue;
                }
                let expectation = possible_states
                    .into_iter()
                    .map(|(_outcome, possible_stations)| {
                        (helper(all_stations, &possible_stations, cache).1 + 1.)
                            * possible_stations.len() as f64
                    })
                    .sum::<f64>()
                    / possible_stations.len() as f64;

                if expectation < min_expectation {
                    min_expectation = expectation;
                    best_guess = Some(*station);
                    possible_station_picked = possible_stations.contains(station);
                } else if (expectation == min_expectation)
                    && !possible_station_picked
                    && possible_stations.contains(station)
                {
                    possible_station_picked = true;
                    best_guess = Some(*station);
                }
            }
            let res = (best_guess.unwrap(), min_expectation);
            cache.insert(possible_stations.to_vec(), res);
            res
        }

        let (station, expectation) = helper(all_stations, possible_stations, &mut HashMap::new());
        Output {
            station,
            cost: Expectation(expectation),
        }
    }
}
