use std::collections::HashMap;
use tuble::*;

fn get_possible_states(
    target_station: &Station,
    possible_stations: &[Station],
) -> HashMap<Outcome, Vec<Station>> {
    let mut map: HashMap<Outcome, Vec<Station>> = HashMap::new();
    for &station in possible_stations {
        map.entry(target_station.get_outcome(&station))
            .or_default()
            .push(station);
    }
    map
}

fn num_guesses_required(possible_stations: &[Station]) -> (Station, usize) {
    if possible_stations.len() == 1 {
        return (possible_stations[0], 0);
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

fn main() {
    let all_stations = Station::all_stations();
    let (best_station, total_guesses_required) = num_guesses_required(&all_stations);
    println!("{} {}", best_station.get_name(), total_guesses_required);
}
