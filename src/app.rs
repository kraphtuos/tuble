use std::fmt;

use crate::dropdown::*;
use tuble::*;
use yew::prelude::*;

use crate::{get_possible_states, num_guesses_required};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub possible_stations: Vec<Station>,
    pub best_guess: Station,
    pub max_guesses: usize,
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
        possible_stations,
        best_guess,
        max_guesses,
    } = props;
    if possible_stations.len() == 1 {
        return html! { <div class="container">{format!("answer: {}", possible_stations[0])}</div> };
    };
    let station_state = use_state(|| None::<Station>);
    let outcome_state = use_state(|| None::<Outcome>);
    let station_select = {
        let dropdown_props = {
            let station_state = station_state.clone();
            let outcome_state = outcome_state.clone();
            let name = "station".into();
            let choices = possible_stations.clone();
            let submit = Callback::from(move |choice: Option<Station>| {
                station_state.set(choice);
                outcome_state.set(None);
            });
            DropdownProps::<Station> {
                name,
                choices,
                submit,
            }
        };
        html! { <DropdownComponent<Station> ..dropdown_props /> }
    };
    let outcome_select = if let Some(station) = *station_state {
        let possible_outcomes = get_possible_states(&station, possible_stations);
        let dropdown_props = {
            let outcome_state = outcome_state.clone();
            let name = "outcome".into();
            let choices: Vec<_> = possible_outcomes
                .iter()
                .map(|(outcome, possible_stations)| Choice {
                    outcome: outcome.to_owned(),
                    possible_stations: possible_stations.len(),
                })
                .collect();
            let submit = Callback::from(move |choice: Option<Choice>| {
                outcome_state.set(choice.map(|choice| choice.outcome))
            });
            DropdownProps::<Choice> {
                name,
                choices,
                submit,
            }
        };
        let child = if let Some(outcome) = *outcome_state {
            if let Some(possible_stations) = possible_outcomes.get(&outcome) {
                let (best_guess, max_guesses) = num_guesses_required(possible_stations);
                let props = Props {
                    possible_stations: possible_stations.clone(),
                    best_guess,
                    max_guesses,
                };
                html! {
                    <Component ..props />
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };
        html! { <><DropdownComponent<Choice> ..dropdown_props />{child}</> }
    } else {
        html! {}
    };
    html! {
        <div class="container">
            <div class="container">
                {format!("best guess: {}", best_guess)}
            </div>
            <div class="container">
                {format!("max guesses: {}", max_guesses)}
            </div>
            <div class="container">
                {station_select}
            </div>
            <div class="container">
                {outcome_select}
            </div>
        </div>
    }
}
