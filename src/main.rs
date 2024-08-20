use tuble::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let all_stations = Station::all_stations();
    fn print<O: Optimiser>(all_stations: &[Station]) {
        let Output { station, score } = O::optimise(&all_stations, &all_stations);
        println!("{}: {} - {}", O::NAME, station, score);
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
