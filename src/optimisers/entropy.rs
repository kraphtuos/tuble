//! Optimise for maximal entropy.

use super::*;

pub fn optimise(all_stations: &[Station], possible_stations: &[Station]) -> (Station, f64) {
    let mut max_entropy = f64::MIN;
    let mut best_guess = None;
    let mut possible_station_picked = false;

    let n = possible_stations.len() as f64;

    for station in all_stations {
        let entropy = -get_possible_states(station, possible_stations)
            .into_iter()
            .map(|(_, possible_stations)| {
                #[cfg(target_arch = "wasm32")]
                if possible_stations.len() == 0 {
                    web_sys::console::log_1(&"zero length".into());
                }
                let p = possible_stations.len() as f64 / n;
                p * p.log2()
            })
            .sum::<f64>();

        if entropy > max_entropy {
            max_entropy = entropy;
            best_guess = Some(*station);
            possible_station_picked = possible_stations.contains(station);
        } else if (entropy == max_entropy)
            && !possible_station_picked
            && possible_stations.contains(station)
        {
            possible_station_picked = true;
            best_guess = Some(*station);
        }
    }

    (best_guess.unwrap(), max_entropy)
}
