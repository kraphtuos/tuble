mod dropdown;

use std::fmt;
use yew::prelude::*;

use crate::*;
use dropdown::*;

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
        return html! { <div class="container">{format!("answer: {}", possible_stations[0])}</div> };
    };
    let station_state = use_state(|| None::<Station>);
    let outcome_state = use_state(|| None::<Outcome>);
    let station_select = {
        let dropdown_props = {
            let station_state = station_state.clone();
            let outcome_state = outcome_state.clone();
            let name = "station".into();
            let choices = all_stations.clone();
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
        let button = html! { <DropdownComponent<Choice> ..dropdown_props /> };
        let child = if let Some(outcome) = *outcome_state {
            if let Some(possible_stations) = possible_outcomes.get(&outcome) {
                let choice = Choice {
                    outcome,
                    possible_stations: possible_stations.len(),
                };
                let best_guess = num_guesses_required(all_stations, possible_stations);
                let all_stations = all_stations.clone();
                let props = Props {
                    all_stations,
                    possible_stations: possible_stations.clone(),
                    best_guess,
                };
                html! { <>{choice}<App ..props /></> }
            } else {
                html! {}
            }
        } else {
            html! {}
        };
        html! { <>{station}{button}{child}</> }
    } else {
        html! {}
    };
    html! {
        <div class="container">
            {format!("best guess: {} - {}", best_guess.0, best_guess.1)}
            {station_select}
            {outcome_select}
        </div>
    }
}
