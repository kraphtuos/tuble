pub mod data;
pub use data::*;

pub mod optimisers;
pub use optimisers::*;

#[cfg(target_arch = "wasm32")]
pub mod app;
