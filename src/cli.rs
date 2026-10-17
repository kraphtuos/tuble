use std::io::{self, BufRead, Write};
use tuble::*;

const USAGE: &str = "usage: tuble [best | eval | play]
  best  print each optimiser's first guess (default)
  eval  play every optimiser against every station and compare them
  play  suggest guesses while you play a game";

pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] | ["best"] => best(),
        ["eval"] => eval(),
        ["play"] => play(),
        _ => {
            eprintln!("{USAGE}");
            std::process::exit(2);
        }
    }
}

fn best() {
    let possible_stations = StationSet::all();
    for optimiser in all_optimisers() {
        let (station, cost) = optimiser.best_guess(possible_stations);
        println!(
            "{}: {} - {} ({})",
            optimiser.name(),
            station,
            cost,
            optimiser.cost_description()
        );
    }
}

fn eval() {
    println!(
        "{:<12} {:>6} {:>4}  guesses: stations",
        "optimiser", "mean", "max"
    );
    for optimiser in all_optimisers() {
        let guesses: Vec<usize> = Station::all()
            .map(|target| guesses_to_find(optimiser.as_ref(), target))
            .collect();
        let max = *guesses.iter().max().unwrap();
        let mean = guesses.iter().sum::<usize>() as f64 / guesses.len() as f64;
        let histogram = (1..=max)
            .map(|n| format!("{n}: {}", guesses.iter().filter(|&&g| g == n).count()))
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "{:<12} {:>6.4} {:>4}  {}",
            optimiser.name(),
            mean,
            max,
            histogram
        );
    }
}

fn play() {
    let optimisers = all_optimisers();
    let mut lines = io::stdin().lock().lines();
    let mut possible_stations = StationSet::all();
    for guesses in 1.. {
        if possible_stations.len() == 1 {
            println!("The answer is {}", possible_stations.first().unwrap());
            return;
        }
        println!("\n{} possible stations", possible_stations.len());
        if possible_stations.len() <= 10 {
            let names: Vec<_> = possible_stations.iter().map(|s| s.name()).collect();
            println!("  {}", names.join(", "));
        }
        let suggestions: Vec<Station> = optimisers
            .iter()
            .enumerate()
            .map(|(i, optimiser)| {
                let (station, cost) = optimiser.best_guess(possible_stations);
                println!(
                    "  {}. {}: {} - {} ({})",
                    i + 1,
                    optimiser.name(),
                    station,
                    cost,
                    optimiser.cost_description()
                );
                station
            })
            .collect();

        let Some(guess) = prompt(
            &mut lines,
            "Guess (station or suggestion number): ",
            |input| {
                let suggestion = input.parse::<usize>().ok().and_then(|i| {
                    let i = i.checked_sub(1)?;
                    suggestions.get(i).copied()
                });
                suggestion
                    .or_else(|| Station::find(input))
                    .ok_or_else(|| format!("no station called {input:?}"))
            },
        ) else {
            return;
        };

        let parts = partition(guess, possible_stations);
        let Some(outcome) = prompt(&mut lines, "Outcome (stops then zones): ", |input| {
            let outcome: Outcome = input.parse()?;
            if parts.contains_key(&outcome) {
                Ok(outcome)
            } else {
                Err(format!("no possible station gives {outcome} for {guess}"))
            }
        }) else {
            return;
        };
        if outcome.stops == 0 {
            println!("Found {guess} in {guesses} guesses");
            return;
        }
        possible_stations = parts[&outcome];
    }
}

/// Asks until `parse` accepts the input, or returns `None` at the end of input.
fn prompt<T>(
    lines: &mut impl Iterator<Item = io::Result<String>>,
    message: &str,
    mut parse: impl FnMut(&str) -> Result<T, String>,
) -> Option<T> {
    loop {
        print!("{message}");
        io::stdout().flush().ok();
        let line = lines.next()?.ok()?;
        match parse(line.trim()) {
            Ok(value) => return Some(value),
            Err(error) => println!("{error}"),
        }
    }
}
