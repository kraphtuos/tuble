mod select;

use std::fmt;
use std::rc::Rc;
use yew::prelude::*;

use crate::*;
use select::*;

/// The optimisers shared by every level of the app so their caches are reused.
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

#[derive(PartialEq, Properties)]
pub struct Props {
    pub possible_stations: StationSet,
    #[prop_or_default]
    pub optimisers: Optimisers,
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
        possible_stations,
        optimisers,
    } = props;
    let possible_stations = *possible_stations;
    if possible_stations.len() == 1 {
        return html! { <div class="container"><p>{format!("answer: {}", possible_stations.first().unwrap())}</p></div> };
    };
    let mut child = html! {};
    let station_state = use_state_eq(|| None::<Station>);
    let outcome_state = use_state_eq(|| None::<Outcome>);
    let mut rows = vec![];
    {
        // Best guess row
        let columns = optimisers
            .0
            .iter()
            .map(|optimiser| {
                let (station, cost) = optimiser.best_guess(possible_stations);
                let station_state = station_state.clone();
                let outcome_state = outcome_state.clone();
                let onclick = Callback::from(move |_| {
                    station_state.set(Some(station));
                    outcome_state.set(None);
                });
                let text = format!("{} best guess: {} - {}", optimiser.name(), station, cost);
                html! { <label class="col-form-label" {onclick}>{text}</label> }
            })
            .collect();
        rows.push(columns);
    }
    {
        // Station selector
        let select_props = {
            let station_state = station_state.clone();
            let outcome_state = outcome_state.clone();
            let name = "station".into();
            let placeholder = "select station".into();
            let choices = Station::all().map(|station| (station, station)).collect();
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
        let possible_outcomes = partition(station, possible_stations);
        {
            // Worse outcome
            let columns = optimisers
                .0
                .iter()
                .map(|optimiser| {
                    let (outcome, cost) = optimiser.worst_outcome(station, possible_stations);
                    let outcome_state = outcome_state.clone();
                    let onclick = Callback::from(move |_| outcome_state.set(Some(outcome)));
                    let text = format!("{} worse case: {} - {}", optimiser.name(), outcome, cost);
                    html! { <label class="col-form-label" {onclick}>{text}</label> }
                })
                .collect();
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
                            outcome,
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
            if let Some(&possible_stations) = possible_outcomes.get(&outcome) {
                let props = Props {
                    possible_stations,
                    optimisers: optimisers.clone(),
                };
                child = html! { <App ..props /> };
            }
        }
    }
    html! {
        <div class="container">
            <div class="row mb-3">
                {
                    for rows.into_iter().map(|columns: Vec<Html>|
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
