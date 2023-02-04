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

pub fn num_guesses_required(possible_stations: &[Station]) -> (Station, usize) {
    if possible_stations.len() == 1 {
        return (possible_stations[0], 1);
    }
    possible_stations
        .iter()
        .map(|station| {
            get_possible_states(station, possible_stations)
                .iter()
                .map(|x| {
                    if x.1.len() == possible_stations.len() {
                        return usize::MAX - 1;
                    }
                    num_guesses_required(x.1).1
                })
                .max()
                .map(|x| (*station, x + 1))
                .unwrap()
        })
        .min_by_key(|x| x.1)
        .unwrap()
}
