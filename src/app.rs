use std::{collections::BTreeMap, fmt};

use crate::dropdown::*;
use tuble::*;
use yew::prelude::*;

use crate::{get_possible_states, num_guesses_required};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub best_guess: Station,
    pub max_guesses: usize,
    pub possible_outcomes: BTreeMap<Outcome, Vec<Station>>,
}

#[derive(Clone, Copy, PartialEq, Properties)]
struct Choice {
    outcome: Outcome,
    possible_stations: usize,
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.outcome, self.possible_stations)
    }
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
        html! { <div class="container">{format!("answer: {}", stations[0])}</div> }
    } else {
        let dropdown_props = {
            let choices: Vec<_> = possible_outcomes
                .iter()
                .map(|(outcome, possible_stations)| Choice {
                    outcome: outcome.to_owned(),
                    possible_stations: possible_stations.len(),
                })
                .collect();
            let selection = selection.clone();
            let submit = Callback::from(move |choice: Option<Choice>| {
                selection.set(choice.map(|choice| choice.outcome))
            });
            DropdownProps::<Choice> { choices, submit }
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
                    {format!("best guess: {}", best_guess)}
                </div>
                <div class="container">
                    {format!("max guesses: {}", max_guesses)}
                </div>
                <div class="container">
                    <DropdownComponent<Choice> ..dropdown_props />
                </div>
                {child}
            </div>
        }
    }
}
