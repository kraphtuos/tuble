mod select;

use std::fmt;
use yew::prelude::*;

use crate::*;
use select::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub all_stations: Vec<Station>,
    pub possible_stations: Vec<Station>,
    pub best_guess_minimax: (Station, usize),
    pub best_guess_size: (Station, usize),
    pub best_guess_entropy: (Station, f64),
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
        best_guess_minimax,
        best_guess_size,
        best_guess_entropy,
    } = props;
    if possible_stations.len() == 1 {
        return html! { <div class="container"><p>{format!("answer: {}", possible_stations[0])}</p></div> };
    };
    let station_state = use_state(|| None::<Station>);
    let choice_state = use_state(|| None::<Choice>);
    let mut columns = vec![
        html! { <label class="col-form-label">{format!("minimax best guess: {} - {}", best_guess_minimax.0, best_guess_minimax.1)}</label> },
        html! { <label class="col-form-label">{format!("size best guess: {} - {}", best_guess_size.0, best_guess_size.1)}</label> },
        html! { <label class="col-form-label">{format!("entropy best guess: {} - {}", best_guess_entropy.0, best_guess_entropy.1)}</label> },
    ];
    let mut child = html! {};
    {
        let select_props = {
            let station_state = station_state.clone();
            let choice_state = choice_state.clone();
            let name = "station".into();
            let placeholder = "select station".into();
            let choices = all_stations.clone();
            let selected = *station_state;
            let submit = Callback::from(move |choice: Option<Station>| {
                station_state.set(choice);
                choice_state.set(None);
            });
            SelectProps::<Station> {
                name,
                placeholder,
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
            let worst_guess_minimax = possible_outcomes
                .iter()
                .map(|(outcome, possible_stations)| {
                    (
                        outcome,
                        minimax::optimise(all_stations, possible_stations).1,
                    )
                })
                .max_by_key(|x| x.1)
                .unwrap();
            let worst_guess_size = possible_outcomes
                .iter()
                .map(|(outcome, possible_stations)| {
                    (outcome, size::optimise(all_stations, possible_stations).1)
                })
                .max_by_key(|x| x.1)
                .unwrap();
            let worst_guess_entropy = possible_outcomes
                .iter()
                .map(|(outcome, possible_stations)| {
                    (
                        outcome,
                        entropy::optimise(all_stations, possible_stations).1,
                    )
                })
                .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                .unwrap();
            columns.push(
                html! { <label class="col-form-label">{format!("minimax worst guess: {} - {}", worst_guess_minimax.0, worst_guess_minimax.1)}</label> },
            );
            columns.push(
                html! { <label class="col-form-label">{format!("size worst guess: {} - {}", worst_guess_size.0, worst_guess_size.1)}</label> }
            );
            columns.push(
                html! { <label class="col-form-label">{format!("entropy worst guess: {} - {}", worst_guess_entropy.0, worst_guess_entropy.1)}</label> }
            );
        };
        {
            let select_props = {
                let choice_state = choice_state.clone();
                let name = "outcome".into();
                let placeholder = "select outcome".into();
                let choices: Vec<_> = possible_outcomes
                    .iter()
                    .map(|(outcome, possible_stations)| Choice {
                        outcome: outcome.to_owned(),
                        possible_stations: possible_stations.len(),
                    })
                    .collect();
                let selected = *choice_state;
                let submit = Callback::from(move |choice: Option<Choice>| {
                    // CR: The child component should be re-rendered here.
                    choice_state.set(choice);
                });
                SelectProps::<Choice> {
                    name,
                    placeholder,
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
                let best_guess_minimax = minimax::optimise(all_stations, possible_stations);
                let best_guess_size = size::optimise(all_stations, possible_stations);
                let best_guess_entropy = entropy::optimise(all_stations, possible_stations);
                let all_stations = all_stations.clone();
                let props = Props {
                    all_stations,
                    possible_stations: possible_stations.clone(),
                    best_guess_minimax,
                    best_guess_size,
                    best_guess_entropy,
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
