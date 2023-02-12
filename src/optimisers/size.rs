//! Optimise for the size of the largest component

use super::*;

pub fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> (Station, usize) {
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

    (best_guess.unwrap(), min_max_size)
}
