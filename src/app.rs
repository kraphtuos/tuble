use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::*;

/// The optimisers shared by every step so their caches are reused.
#[derive(Clone)]
pub struct Optimisers(Rc<Vec<Box<dyn AnyOptimiser>>>);

impl Default for Optimisers {
    fn default() -> Self {
        Self(Rc::new(all_optimisers()))
    }
}

impl PartialEq for Optimisers {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[function_component]
pub fn App() -> Html {
    let optimisers = use_memo(|_| Optimisers::default(), ());
    html! {
        <div class="page">
            <header class="header">
                <div class="logo" aria-hidden="true"><span></span><span></span><span></span></div>
                <div>
                    <h1>{"Tuble solver"}</h1>
                    <p class="muted">
                        {"Suggested guesses for "}
                        <a href="https://tuble.co.uk/">{"tuble"}</a>
                        {", the London Underground guessing game."}
                    </p>
                </div>
            </header>
            <main class="line">
                <Step
                    possible_stations={StationSet::all()}
                    optimisers={(*optimisers).clone()}
                    number={1}
                />
            </main>
            <footer class="footer muted">
                <a href="https://github.com/kraphtuos/tuble">{"Source on GitHub"}</a>
            </footer>
            <datalist id="stations">
                { for Station::all().map(|station| html! { <option value={station.name()} /> }) }
            </datalist>
        </div>
    }
}

const ZONES: [(ZoneOutcome, &str); 4] = [
    (ZoneOutcome::Correct, "0 zones"),
    (ZoneOutcome::OneAway, "1 zone"),
    (ZoneOutcome::TwoAway, "2 zones"),
    (ZoneOutcome::MoreThanTwo, ">2 zones"),
];

/// The possible outcomes as stops by zones, shaded by how many stations
/// each leaves. Long tables are split in two so they aren't too tall on
/// wide screens.
fn outcome_table(
    parts: &BTreeMap<Outcome, StationSet>,
    picked: Option<Outcome>,
    pick: &Callback<Outcome>,
) -> Html {
    let stops: Vec<u8> = parts
        .keys()
        .map(|outcome| outcome.stops)
        .filter(|&stops| stops > 0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let most = parts.values().map(StationSet::len).max().unwrap_or(1);
    let half = if stops.len() > 8 {
        stops.len().div_ceil(2)
    } else {
        stops.len().max(1)
    };
    let tables = stops.chunks(half).map(|stops| {
        let rows = stops.iter().map(|&stops| {
            let cells = ZONES.iter().map(|&(zones, _)| {
                let outcome = Outcome { stops, zones };
                match parts.get(&outcome) {
                    Some(stations) => {
                        let class = classes!(
                            "cell",
                            (picked == Some(outcome)).then_some("selected"),
                        );
                        let style = format!("--heat: {:.2}", stations.len() as f64 / most as f64);
                        let title = format!("{outcome}: {} stations left", stations.len());
                        let onclick = pick.reform(move |_| outcome);
                        html! {
                            <button type="button" {class} {style} {title} {onclick}>
                                {stations.len()}
                            </button>
                        }
                    }
                    None => html! { <span class="cell empty"></span> },
                }
            });
            html! {
                <>
                    <span class="row-head">{stops}</span>
                    { for cells }
                </>
            }
        });
        html! {
            <div class="outcome-table">
                <span class="col-head">{"stops"}</span>
                { for ZONES.iter().map(|&(_, label)| html! { <span class="col-head">{label}</span> }) }
                { for rows }
            </div>
        }
    });
    html! {
        <>
            <p class="table-caption muted">
                {"Find the stops and zones tuble gave you. Each number is how many stations would be left."}
            </p>
            <div class="outcome-tables">{ for tables }</div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct StepProps {
    possible_stations: StationSet,
    optimisers: Optimisers,
    number: usize,
}

/// One guess: suggestions, the guess made and the outcome tuble gave.
#[function_component]
fn Step(props: &StepProps) -> Html {
    let StepProps {
        possible_stations,
        optimisers,
        number,
    } = props;
    let (possible_stations, number) = (*possible_stations, *number);
    // Hooks must run on every render, so they come before the early return.
    let guess_text = use_state_eq(String::new);
    // The outcome is kept with its guess so changing the guess discards it.
    let picked = use_state_eq(|| None::<(Station, Outcome)>);
    let guess = Station::find(&guess_text);
    let best_guesses = use_memo(
        |(possible_stations, optimisers)| {
            optimisers
                .0
                .iter()
                .map(|optimiser| optimiser.best_guess(*possible_stations))
                .collect::<Vec<_>>()
        },
        (possible_stations, optimisers.clone()),
    );
    let worst_outcomes = use_memo(
        |(guess, possible_stations, optimisers)| {
            guess.map(|guess| {
                optimisers
                    .0
                    .iter()
                    .map(|optimiser| optimiser.worst_outcome(guess, *possible_stations))
                    .collect::<Vec<_>>()
            })
        },
        (guess, possible_stations, optimisers.clone()),
    );

    let set_guess = {
        let guess_text = guess_text.clone();
        Callback::from(move |station: Station| guess_text.set(station.name().to_owned()))
    };

    if possible_stations.len() == 1 {
        let answer = possible_stations.first().unwrap();
        return html! {
            <section class="step done">
                <h2>{"It has to be"}</h2>
                <p class="answer">{answer.name()}</p>
                <p class="muted">{format!("Guess it to finish in {number} guesses.")}</p>
            </section>
        };
    }

    let candidates = (possible_stations.len() <= 12).then(|| {
        html! {
            <ul class="candidates">
                { for possible_stations.iter().map(|station| {
                    let onclick = set_guess.reform(move |_| station);
                    html! { <li><button type="button" {onclick}>{station.name()}</button></li> }
                }) }
            </ul>
        }
    });

    let suggestions = optimisers
        .0
        .iter()
        .zip(best_guesses.iter())
        .map(|(optimiser, &(station, ref cost))| {
            let class = classes!(
                "suggestion",
                format!("opt-{}", optimiser.name()),
                (guess == Some(station)).then_some("selected"),
            );
            let onclick = set_guess.reform(move |_| station);
            html! {
                <button type="button" {class} {onclick}>
                    <span class="suggestion-name">{optimiser.name()}</span>
                    <span class="suggestion-station">{station.name()}</span>
                    <span class="suggestion-cost">
                        <strong>{cost.clone()}</strong>{" "}{optimiser.cost_description()}
                    </span>
                </button>
            }
        })
        .collect::<Html>();

    let oninput = {
        let guess_text = guess_text.clone();
        Callback::from(move |e: InputEvent| {
            guess_text.set(e.target_unchecked_into::<HtmlInputElement>().value())
        })
    };
    let input_id = format!("guess-{number}");
    let unknown = (!guess_text.trim().is_empty() && guess.is_none())
        .then(|| html! { <p class="hint">{"Pick a station from the list."}</p> });

    let mut next = html! {};
    let outcomes = match (guess, worst_outcomes.as_ref()) {
        (Some(guess), Some(worst_outcomes)) => {
            let parts = partition(guess, possible_stations);
            let picked_outcome = picked
                .filter(|&(station, _)| station == guess)
                .map(|(_, outcome)| outcome);
            let pick = {
                let picked = picked.clone();
                Callback::from(move |outcome: Outcome| picked.set(Some((guess, outcome))))
            };
            let correct = parts
                .keys()
                .find(|outcome| outcome.stops == 0)
                .map(|&outcome| {
                    let class = classes!(
                        "correct",
                        (picked_outcome == Some(outcome)).then_some("selected"),
                    );
                    let onclick = pick.reform(move |_| outcome);
                    html! { <button type="button" {class} {onclick}>{"✓ Correct"}</button> }
                });
            let table = outcome_table(&parts, picked_outcome, &pick);
            let worst = optimisers
                .0
                .iter()
                .zip(worst_outcomes)
                .map(|(optimiser, &(outcome, ref cost))| {
                    let class = classes!("worst-item", format!("opt-{}", optimiser.name()));
                    let title = format!("{cost} {} after this", optimiser.cost_description());
                    let onclick = pick.reform(move |_| outcome);
                    html! {
                        <button type="button" {class} {title} {onclick}>
                            <span class="suggestion-name">{optimiser.name()}</span>
                            {outcome.to_string()}
                        </button>
                    }
                })
                .collect::<Html>();

            if let Some(outcome) = picked_outcome {
                next = if outcome.stops == 0 {
                    html! {
                        <section class="step done">
                            <h2>{"Solved"}</h2>
                            <p class="answer">{guess.name()}</p>
                            <p class="muted">
                                {format!("in {number} guess{}.", if number == 1 { "" } else { "es" })}
                            </p>
                        </section>
                    }
                } else {
                    // Key the next step by what led to it so a different
                    // guess or outcome starts it afresh.
                    let key = format!("{guess:?}/{outcome:?}");
                    html! {
                        <Step
                            {key}
                            possible_stations={parts[&outcome]}
                            optimisers={optimisers.clone()}
                            number={number + 1}
                        />
                    }
                };
            }

            html! {
                <>
                    <h3>{format!("What did tuble say for {}?", guess.name())}</h3>
                    {correct}
                    {table}
                    <div class="worst">
                        <span class="muted">{"Worst case for"}</span>
                        {worst}
                    </div>
                </>
            }
        }
        _ => html! {},
    };

    html! {
        <>
            <section class="step">
                <div class="step-head">
                    <h2>{format!("Guess {number}")}</h2>
                    <span class="count">
                        {format!("{} stations possible", possible_stations.len())}
                    </span>
                </div>
                {candidates}
                <h3>{"Suggestions"}</h3>
                <div class="suggestions">{suggestions}</div>
                <label class="field" for={input_id.clone()}>{"Your guess"}</label>
                <input
                    id={input_id}
                    type="text"
                    list="stations"
                    placeholder="Type a station or pick a suggestion"
                    autocomplete="off"
                    spellcheck="false"
                    value={(*guess_text).clone()}
                    {oninput}
                />
                {unknown}
                {outcomes}
            </section>
            {next}
        </>
    }
}
