# Tuble optimiser

A solver for [tuble](https://tuble.co.uk/).

## How to run this

Install [trunk](https://trunkrs.dev/) and run

```
trunk serve --release
```

## Development

### VSCode

Install the rust-analyzer extension.
It might be useful to add the following targets to rustup where rust-analyzer would attempt to compile.

```
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
```

## TODOs

- Add max entropy reducing optimiser for minimal expected number of guesses
- Improve command line UI
- Add cross evaluation of different optimisers