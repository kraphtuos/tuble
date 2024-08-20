mod select;

use std::fmt;
use yew::prelude::*;

use crate::*;
use select::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub all_stations: Vec<Station>,
    pub possible_stations: Vec<Station>,
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
    } = props;
    if possible_stations.len() == 1 {
        return html! { <div class="container"><p>{format!("answer: {}", possible_stations[0])}</p></div> };
    };
    let mut child = html! {};
    let station_state = use_state(|| None::<Station>);
    let choice_state = use_state(|| None::<Choice>);
    let mut rows = vec![];
    {
        // Best guess row
        let mut columns = vec![];
        fn add_col<O: Optimiser>(
            columns: &mut Vec<Html>,
            all_stations: &[Station],
            possible_stations: &[Station],
            station_state: &UseStateHandle<Option<Station>>,
        ) {
            let Output { station, score } = O::optimise(&all_stations, &possible_stations);
            let station_state = station_state.clone();
            let onclick = Callback::from(move |_| station_state.set(Some(station)));
            let text = format!("{} best guess: {} - {}", O::NAME, station, score);
            let column = html! { <label class="col-form-label" {onclick}>{text}</label> };
            columns.push(column)
        }
        add_col::<MinimaxOptimiser>(
            &mut columns,
            all_stations,
            possible_stations,
            &station_state,
        );
        add_col::<SizeOptimiser>(
            &mut columns,
            all_stations,
            possible_stations,
            &station_state,
        );
        add_col::<EntropyOptimiser>(
            &mut columns,
            all_stations,
            possible_stations,
            &station_state,
        );
        rows.push(columns);
    }
    {
        // Station selector
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
        rows.push(vec![html! { <SelectComponent<Station> ..select_props /> }]);
    }
    if let Some(station) = *station_state {
        let possible_outcomes = get_possible_states(&station, possible_stations);
        {
            // Worse outcome
            let mut columns = vec![];
            fn add_col<O: Optimiser>(
                columns: &mut Vec<Html>,
                all_stations: &[Station],
                possible_outcomes: &std::collections::BTreeMap<Outcome, Vec<Station>>,
            ) {
                let worse_case = possible_outcomes
                    .iter()
                    .map(|(outcome, possible_stations)| {
                        (outcome, O::optimise(all_stations, possible_stations).score)
                    })
                    .max_by_key(|x| x.1)
                    .unwrap();
                let text = format!(
                    "{} worse case: {} - {}",
                    O::NAME,
                    worse_case.0,
                    worse_case.1
                );
                let column = html! { <label class="col-form-label">{text}</label> };
                columns.push(column);
            }
            add_col::<MinimaxOptimiser>(&mut columns, all_stations, &possible_outcomes);
            add_col::<SizeOptimiser>(&mut columns, all_stations, &possible_outcomes);
            add_col::<EntropyOptimiser>(&mut columns, all_stations, &possible_outcomes);
            rows.push(columns);
        };
        {
            // Outcome selector
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
            rows.push(vec![html! { <SelectComponent<Choice> ..select_props /> }]);
        }
        if let Some(choice) = *choice_state {
            let outcome = choice.outcome;
            if let Some(possible_stations) = possible_outcomes.get(&outcome) {
                let props = Props {
                    all_stations: all_stations.clone(),
                    possible_stations: possible_stations.clone(),
                };
                child = html! { <App ..props /> };
            }
        }
    }
    html! {
        <div class="container">
            <div class="row mb-3">
                {
                    for rows.into_iter().map(|columns|
                        html! {
                            <div class="row">
                                { for columns.into_iter().map(|col| html! { <div class="col-auto">{col}</div> }) }
                            </div>
                        }
                    )
                 }
            </div>
            {child}
        </div>
    }
}
