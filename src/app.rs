mod select;

use std::fmt;
use yew::prelude::*;

use crate::*;
use select::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub all_stations: Vec<Station>,
    pub possible_stations: Vec<Station>,
    pub best_guess: (Station, usize),
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
pub fn App(props: &Props) -> Html {
    let Props {
        all_stations,
        possible_stations,
        best_guess,
    } = props;
    if possible_stations.len() == 1 {
        return html! { <div class="container"><p>{format!("answer: {}", possible_stations[0])}</p></div> };
    };
    let station_state = use_state(|| None::<Station>);
    let choice_state = use_state(|| None::<Choice>);
    let mut columns = vec![
        html! { <label class="col-form-label">{format!("best guess: {} - {}", best_guess.0, best_guess.1)}</label> },
    ];
    let mut child = html! {};
    {
        let select_props = {
            let station_state = station_state.clone();
            let choice_state = choice_state.clone();
            let name = "station".into();
            let choices = all_stations.clone();
            let selected = *station_state;
            let submit = Callback::from(move |choice: Option<Station>| {
                station_state.set(choice);
                choice_state.set(None);
            });
            SelectProps::<Station> {
                name,
                choices,
                selected,
                submit,
            }
        };
        columns.push(html! { <SelectComponent<Station> ..select_props /> });
    }
    if let Some(station) = *station_state {
        let possible_outcomes = get_possible_states(&station, possible_stations);
        {
            let select_props = {
                let choice_state = choice_state.clone();
                let name = "outcome".into();
                let choices: Vec<_> = possible_outcomes
                    .iter()
                    .map(|(outcome, possible_stations)| Choice {
                        outcome: outcome.to_owned(),
                        possible_stations: possible_stations.len(),
                    })
                    .collect();
                let selected = *choice_state;
                let submit = Callback::from(move |choice: Option<Choice>| choice_state.set(choice));
                SelectProps::<Choice> {
                    name,
                    choices,
                    selected,
                    submit,
                }
            };
            columns.push(html! { <SelectComponent<Choice> ..select_props /> });
        }
        if let Some(choice) = *choice_state {
            let outcome = choice.outcome;
            if let Some(possible_stations) = possible_outcomes.get(&outcome) {
                let best_guess = num_guesses_required(all_stations, possible_stations);
                let all_stations = all_stations.clone();
                let props = Props {
                    all_stations,
                    possible_stations: possible_stations.clone(),
                    best_guess,
                };
                child = html! { <App ..props /> };
            }
        }
    }
    html! {
        <div class="container">
            <div class="row mb-3">
                { for columns.into_iter().map(|col| html! { <div class="col-auto">{col}</div> }) }
            </div>
            {child}
        </div>
    }
}
