# Tuble optimiser

A solver for [tuble](https://tuble.co.uk/).

After each guess tuble tells you how many stops and zones away the answer is.
The solver groups the stations that are still possible by the outcome each
guess would give, and suggests a guess using one of four optimisers:

| Optimiser | Picks the guess that minimises | Kind |
|---|---|---|
| minimax | the worst case number of guesses | exact search |
| size | the most stations left after the guess | one step heuristic |
| entropy | the expected bits of entropy left after the guess | one step heuristic |
| expectation | the expected number of guesses | exact search |

Ties go to a guess that could be the answer.
Playing every optimiser against every station (`tuble eval`) gives:

```
optimiser      mean  max  guesses: stations
minimax      2.9740    4  1: 1, 2: 55, 3: 163, 4: 50
size         2.8699    5  1: 1, 2: 72, 3: 158, 4: 37, 5: 1
entropy      2.7844    5  1: 1, 2: 85, 3: 155, 4: 27, 5: 1
expectation  2.7807    5  1: 1, 2: 89, 3: 149, 4: 28, 5: 2
```

## How to run this

### Web app

Install [trunk](https://trunkrs.dev/) and run

```
trunk serve --release
```

Pushes to `main` also deploy it to GitHub Pages.

### Command line

```
cargo run --release -- [best | eval | play]
```

- `best` prints each optimiser's first guess (the default).
- `eval` plays every optimiser against every station and compares them.
- `play` suggests guesses while you play, narrowing down the stations from
  the outcomes you enter, e.g. `3 1` for 3 stops and 1 zone or `7 >2`.

## Data

- `data/stations.json` lists the station names.
- `data/distances.json` holds `[stops, zones]` between every pair of
  stations, indexed in the same order.

Both are JSON arrays that are also valid Rust array literals, so they are
compiled in with `include!`. The tests check the distances are symmetric and
zero only between a station and itself.

## Development

```
cargo test
cargo clippy --all-targets
cargo clippy --target wasm32-unknown-unknown
```

### VSCode

Install the rust-analyzer extension.
It might be useful to add the following targets to rustup where rust-analyzer would attempt to compile.

```
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
```
