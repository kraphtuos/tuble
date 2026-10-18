pub mod data;
pub use data::*;

pub mod optimisers;
pub use optimisers::*;

#[cfg(target_arch = "wasm32")]
pub mod app;

/// The recursive optimisers' first guesses, precomputed by build.rs.
mod first_guesses {
    include!(concat!(env!("OUT_DIR"), "/first_guesses.rs"));
}

/// Every optimiser, with the slow first guesses already known.
pub fn all_optimisers() -> Vec<Box<dyn AnyOptimiser>> {
    let all = StationSet::all();
    let count = all.len();
    let station = |name| Station::find(name).expect("precomputed station exists");

    let minimax = MinimaxOptimiser::default();
    let (name, steps) = first_guesses::MINIMAX;
    minimax.remember(
        all,
        Output {
            station: station(name),
            cost: Steps(steps),
        },
    );

    let expectation = ExpectationOptimiser::default();
    let (name, total) = first_guesses::EXPECTATION;
    expectation.remember(
        all,
        Output {
            station: station(name),
            cost: Expectation { total, count },
        },
    );

    vec![
        Box::new(minimax),
        Box::new(SizeOptimiser),
        Box::new(EntropyOptimiser),
        Box::new(expectation),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precomputed_first_guesses_match_a_fresh_search() {
        let all = StationSet::all();
        let fresh: Vec<Box<dyn AnyOptimiser>> = vec![
            Box::new(MinimaxOptimiser::default()),
            Box::new(ExpectationOptimiser::default()),
        ];
        for optimiser in fresh {
            let precomputed = all_optimisers()
                .into_iter()
                .find(|o| o.name() == optimiser.name())
                .unwrap();
            assert_eq!(precomputed.best_guess(all), optimiser.best_guess(all));
        }
    }
}
