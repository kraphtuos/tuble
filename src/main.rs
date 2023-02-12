use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    let (best_guess, max_guesses) = minimax::optimise(&all_stations, &all_stations);
    print!("{}, {}", best_guess, max_guesses);
}

#[cfg(target_arch = "wasm32")]
use app::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    let all_stations = Station::all_stations();
    let possible_stations = all_stations.clone();
    let best_guess = minimax::optimise(&all_stations, &possible_stations);
    yew::Renderer::<App>::with_props(Props {
        all_stations,
        possible_stations,
        best_guess,
    })
    .render();
}
