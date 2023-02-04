pub mod data;

#[cfg(target_arch = "wasm32")]
pub mod app;

pub use self::data::*;

use std::collections::BTreeMap;

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
    mem: &mut std::collections::HashMap<Vec<Station>, (Station, usize)>,
) -> (Station, usize) {
    if let Some(value) = mem.get(possible_stations) {
        return *value;
    }
    if possible_stations.len() == 1 {
        return (possible_stations[0], 1);
    }
    let res = all_stations
        .iter()
        .filter_map(|station| {
            let possible_states = get_possible_states(station, possible_stations);
            if possible_states.len() == 1 {
                return None;
            };
            possible_states
                .iter()
                .map(|x| num_guesses_required_helper(all_stations, x.1, mem).1)
                .max()
                .map(|x| (*station, x + 1))
        })
        .min_by_key(|x| x.1)
        .unwrap();
    mem.insert(possible_stations.to_owned(), res);
    res
}

pub fn num_guesses_required(
    all_stations: &[Station],
    possible_stations: &[Station],
) -> (Station, usize) {
    num_guesses_required_helper(
        all_stations,
        possible_stations,
        &mut std::collections::HashMap::new(),
    )
}
