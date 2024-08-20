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
    let outcome_state = use_state(|| None::<Outcome>);
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
            let Output { station, cost } = O::optimise(&all_stations, &possible_stations);
            let station_state = station_state.clone();
            let onclick = Callback::from(move |_| station_state.set(Some(station)));
            let text = format!("{} best guess: {} - {}", O::NAME, station, cost);
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
            let outcome_state = outcome_state.clone();
            let name = "station".into();
            let placeholder = "select station".into();
            let choices = all_stations
                .iter()
                .map(|&station| (station, station))
                .collect();
            let selected = *station_state;
            let submit = Callback::from(move |choice: Option<Station>| {
                station_state.set(choice);
                outcome_state.set(None);
            });
            SelectProps::<Station, Station> {
                name,
                placeholder,
                choices,
                selected,
                submit,
            }
        };
        rows.push(vec![
            html! { <SelectComponent<Station, Station> ..select_props /> },
        ]);
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
                outcome_state: &UseStateHandle<Option<Outcome>>,
            ) {
                let (&outcome, cost) = possible_outcomes
                    .iter()
                    .map(|(outcome, possible_stations)| {
                        (outcome, O::optimise(all_stations, possible_stations).cost)
                    })
                    .max_by_key(|x| x.1)
                    .unwrap();
                let outcome_state = outcome_state.clone();
                let onclick = Callback::from(move |_| outcome_state.set(Some(outcome)));
                let text = format!("{} worse case: {} - {}", O::NAME, outcome, cost);
                let column = html! { <label class="col-form-label" {onclick}>{text}</label> };
                columns.push(column);
            }
            add_col::<MinimaxOptimiser>(
                &mut columns,
                all_stations,
                &possible_outcomes,
                &outcome_state,
            );
            add_col::<SizeOptimiser>(
                &mut columns,
                all_stations,
                &possible_outcomes,
                &outcome_state,
            );
            add_col::<EntropyOptimiser>(
                &mut columns,
                all_stations,
                &possible_outcomes,
                &outcome_state,
            );
            rows.push(columns);
        };
        {
            // Outcome selector
            let select_props = {
                let outcome_state = outcome_state.clone();
                let name = "outcome".into();
                let placeholder = "select outcome".into();
                let choices: Vec<_> = possible_outcomes
                    .iter()
                    .map(|(&outcome, possible_stations)| {
                        let choice = Choice {
                            outcome: outcome,
                            possible_stations: possible_stations.len(),
                        };
                        (outcome, choice)
                    })
                    .collect();
                let selected = *outcome_state;
                let submit = Callback::from(move |outcome: Option<Outcome>| {
                    // CR: The child component should be re-rendered here.
                    outcome_state.set(outcome);
                });
                SelectProps::<Outcome, Choice> {
                    name,
                    placeholder,
                    choices,
                    selected,
                    submit,
                }
            };
            rows.push(vec![
                html! { <SelectComponent<Outcome, Choice> ..select_props /> },
            ]);
        }
        if let Some(outcome) = *outcome_state {
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
