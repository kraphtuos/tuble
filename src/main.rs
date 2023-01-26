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

fn num_guesses_required(possible_stations: &[Station]) -> usize {
    if possible_stations.len() == 1 {
        return 0;
    }
    possible_stations
        .iter()
        .map(|station| {
            get_possible_states(station, possible_stations)
                .iter()
                .map(|s| num_guesses_required(s.1))
                .max()
                .unwrap()
                + 1
        })
        .min()
        .unwrap()
}

fn main() {
    let stations: Vec<Station> = (0..269).map(Station::from).collect();
    let total_guesses_required = num_guesses_required(&stations);
    println!("{}", total_guesses_required);
}
