//! Precomputes the first guesses of the recursive optimisers. Searching from
//! every station is most of the work of showing the first step, and slow on
//! phones, so it is done once here and embedded in the crate.

// Compile the crate's own data and optimisers, laid out as they are in src.
#[allow(dead_code)]
#[path = "src"]
mod tuble {
    pub mod data;
    pub mod optimisers;
}

pub use tuble::{data::*, optimisers::*};

fn main() {
    for path in ["data", "src/data.rs", "src/optimisers.rs", "src/optimisers"] {
        println!("cargo::rerun-if-changed={path}");
    }

    let all = StationSet::all();
    let minimax = MinimaxOptimiser::default().optimise(all);
    let expectation = ExpectationOptimiser::default().optimise(all);
    let code = format!(
        "pub const MINIMAX: (&str, usize) = ({:?}, {});\n\
         pub const EXPECTATION: (&str, usize) = ({:?}, {});\n",
        minimax.station.name(),
        minimax.cost.0,
        expectation.station.name(),
        expectation.cost.total,
    );
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{out_dir}/first_guesses.rs"), code).unwrap();
}
