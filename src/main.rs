mod app;
mod dropdown;

use std::collections::BTreeMap;
use tuble::*;

fn get_possible_states(
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

fn num_guesses_required(possible_stations: &[Station]) -> (Station, usize) {
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

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    let (best_guess, max_guesses) = num_guesses_required(&all_stations);
    print!("{}, {}", best_guess, max_guesses);
}

#[cfg(target_arch = "wasm32")]
use crate::app::{Component, Props};

#[cfg(target_arch = "wasm32")]
fn main() {
    let possible_stations = Station::all_stations();
    let (best_guess, max_guesses) = num_guesses_required(&possible_stations);
    yew::Renderer::<Component>::with_props(Props {
        possible_stations,
        best_guess,
        max_guesses,
    })
    .render();
}
