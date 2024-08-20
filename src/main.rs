use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    fn print<O: Optimiser>(all_stations: &[Station]) {
        println!(
            "{}",
            optimise_and_output::<O>(all_stations, all_stations, "")
        )
    }
    print::<MinimaxOptimiser>(&all_stations);
    print::<SizeOptimiser>(&all_stations);
    print::<EntropyOptimiser>(&all_stations);
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
