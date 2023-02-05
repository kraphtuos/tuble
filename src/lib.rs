pub mod data;

#[cfg(target_arch = "wasm32")]
pub mod app;

pub use self::data::*;

use std::collections::{BTreeMap, HashMap};

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

fn num_guesses_required_helper(
    all_stations: &[Station],
    possible_stations: &[Station],
    cache: &mut HashMap<Vec<Station>, (Station, usize)>,
) -> (Station, usize) {
    if possible_stations.len() == 1 {
        return (possible_stations[0], 1);
    }
    if possible_stations.len() == 2 {
        return (possible_stations[0], 2);
    }
    if let Some(res) = cache.get(possible_stations) {
        return *res;
    }
    let mut min_max_guess = usize::MAX;
    let mut best_guess = None;
    let mut possible_station_picked = false;
    'outer: for station in all_stations {
        let possible_states = get_possible_states(station, possible_stations);
        if possible_states.len() == 1 {
            continue;
        }
        let mut max_guess = 0;
        for (_outcome, possible_stations) in possible_states {
            let num_guesses =
                num_guesses_required_helper(all_stations, &possible_stations, cache).1 + 1;
            if (num_guesses > min_max_guess)
                || (num_guesses == min_max_guess && possible_station_picked)
            {
                continue 'outer;
            }
            max_guess = max_guess.max(num_guesses);
        }

        if max_guess < min_max_guess {
            min_max_guess = max_guess;
            best_guess = Some(*station);
            if possible_stations.contains(station) {
                possible_station_picked = true;
            } else {
                possible_station_picked = false;
            }
        } else if (max_guess == min_max_guess)
            && !possible_station_picked
            && possible_stations.contains(station)
        {
            possible_station_picked = true;
            best_guess = Some(*station);
        }
    }
    let res = (best_guess.unwrap(), min_max_guess);
    cache.insert(possible_stations.to_vec(), res);
    res
}

pub fn num_guesses_required(
    all_stations: &[Station],
    possible_stations: &[Station],
) -> (Station, usize) {
    num_guesses_required_helper(all_stations, possible_stations, &mut HashMap::new())
}
