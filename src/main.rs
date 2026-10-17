use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let possible_stations = StationSet::all();
    for optimiser in all_optimisers() {
        let (station, cost) = optimiser.best_guess(possible_stations);
        println!("{}: {} - {}", optimiser.name(), station, cost);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use app::*;

    yew::Renderer::<App>::with_props(Props {
        possible_stations: StationSet::all(),
        optimisers: Optimisers::default(),
    })
    .render();
}
