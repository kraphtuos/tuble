use std::collections::BTreeMap;

use tuble::*;
use yew::prelude::*;

use crate::{get_possible_states, num_guesses_required};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub best_guess: Station,
    pub max_guesses: usize,
    pub possible_outcomes: BTreeMap<Outcome, Vec<Station>>,
}

#[function_component]
pub fn Component(props: &Props) -> Html {
    let Props {
        best_guess,
        max_guesses,
        possible_outcomes,
    } = props;
    let selection = use_state(|| None::<Outcome>);
    let stations: Vec<_> = possible_outcomes.values().flatten().collect();
    if stations.len() == 1 {
        html! { <div class="container">{format!("answer: {}", stations[0].get_name())}</div> }
    } else {
        let select = {
            let options = possible_outcomes
                .keys()
                .map(|outcome| {
                    let s = selection.clone();
                    let outcome = outcome.to_owned();
                    let onclick = Callback::from(move |_| s.set(Some(outcome)));
                    html! {<li><a class="dropdown-item" {onclick}>{outcome.to_string()}</a></li>}
                })
                .collect::<Html>();
            let onclick = {
                let s = selection.clone();
                Callback::from(move |_| s.set(None))
            };
            html! {
                <div class="dropdown">
                    <button class="btn btn-secondary dropdown-toggle" data-bs-toggle="dropdown" {onclick}>
                        {"select outcome"}
                    </button>
                    <ul class="dropdown-menu">
                        {options}
                    </ul>
                </div>
            }
        };
        let child = (*selection)
            .as_ref()
            .and_then(|outcome| {
                possible_outcomes.get(&outcome).map(|possible_stations| {
                    let (best_guess, max_guesses) = num_guesses_required(possible_stations);
                    let possible_outcomes = get_possible_states(&best_guess, possible_stations);
                    let props = Props {
                        best_guess,
                        max_guesses,
                        possible_outcomes,
                    };
                    html! { <Component ..props /> }
                })
            })
            .unwrap_or(html! {});
        html! {
            <div class="container">
                <div class="container">
                    {format!("best guess: {}", best_guess.get_name())}
                </div>
                <div class="container">
                    {format!("max guesses: {}", max_guesses)}
                </div>
                <div class="container">
                    {select}
                </div>
                <div class="container">
                    {child}
                </div>
            </div>
        }
    }
}
