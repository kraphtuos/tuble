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

fn get_max_entropy(target_station: &Station, possible_stations: &[Station]) -> usize {
    let states = get_possible_states(target_station, possible_stations);
    states
        .iter()
        .max_by(|a, b| a.1.len().cmp(&b.1.len()))
        .map(|x| x.1.len())
        .unwrap()
}

fn best_guess_by_entropy(possible_stations: &[Station]) -> Station {
    possible_stations
        .iter()
        .min_by(|&a, &b| {
            get_max_entropy(a, possible_stations).cmp(&get_max_entropy(b, possible_stations))
        })
        .map(|x| *x)
        .unwrap()
}

fn main() {
    let stations: Vec<Station> = (0..269).map(Station::from).collect();
    let best_guess = best_guess_by_entropy(&stations);
    println!("{}", best_guess.get_name());
}
