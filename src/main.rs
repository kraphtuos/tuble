use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    let (best_guess, max_guesses) = minimax::optimise(&all_stations, &all_stations);
    println!("minimax: {}, {}", best_guess, max_guesses);
    let (best_guess, max_size) = size::optimise(&all_stations, &all_stations);
    println!("max size: {}, {}", best_guess, max_size);
    let (best_guess, min_entropy) = entropy::optimise(&all_stations, &all_stations);
    println!("min entropy: {}, {}", best_guess, min_entropy);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use app::*;

    let all_stations = Station::all_stations();
    let possible_stations = all_stations.clone();
    yew::Renderer::<App>::with_props(Props {
        all_stations,
        possible_stations,
    })
    .render();
}
