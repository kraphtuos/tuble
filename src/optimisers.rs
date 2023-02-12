use crate::*;
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

pub mod minimax;
pub mod size;
