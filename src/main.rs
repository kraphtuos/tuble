#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    cli::main();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use tuble::{app::*, *};

    yew::Renderer::<App>::with_props(Props {
        possible_stations: StationSet::all(),
        optimisers: Optimisers::default(),
    })
    .render();
}
