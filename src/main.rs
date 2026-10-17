#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    cli::main();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<tuble::app::App>::new().render();
}
