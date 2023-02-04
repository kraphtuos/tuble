use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    let (best_guess, max_guesses) = num_guesses_required(&all_stations);
    print!("{}, {}", best_guess, max_guesses);
}

#[cfg(target_arch = "wasm32")]
use app::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    let possible_stations = Station::all_stations();
    let best_guess = num_guesses_required(&possible_stations);
    yew::Renderer::<App>::with_props(Props {
        possible_stations,
        best_guess,
    })
    .render();
}
