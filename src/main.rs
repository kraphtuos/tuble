use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    let (best_guess, max_guesses) = minimax::optimise(&all_stations, &all_stations);
    println!("minimax: {}, {}", best_guess, max_guesses);
    let (best_guess, max_size) = size::optimise(&all_stations, &all_stations);
    println!("entropy: {}, {}", best_guess, max_size);
}

#[cfg(target_arch = "wasm32")]
use app::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    let all_stations = Station::all_stations();
    let possible_stations = all_stations.clone();
    let best_guess_minimax = minimax::optimise(&all_stations, &possible_stations);
    let best_guess_size = size::optimise(&all_stations, &possible_stations);
    yew::Renderer::<App>::with_props(Props {
        all_stations,
        possible_stations,
        best_guess_minimax,
        best_guess_size,
    })
    .render();
}
