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

/// A clickable suggestion with its cost, whose meaning is shown on hover.
fn suggestion(
    optimiser: &dyn AnyOptimiser,
    text: String,
    cost: &str,
    onclick: Callback<MouseEvent>,
) -> Html {
    html! {
        <>
            <button type="button" class="btn btn-outline-primary btn-sm" {onclick}>
                {format!("{}: {}", optimiser.name(), text)}
            </button>
            {" "}
            <abbr class="small text-muted" title={optimiser.cost_description()}>{cost.to_owned()}</abbr>
        </>
    }
}

#[function_component]
pub fn App(props: &Props) -> Html {
    let Props {
        possible_stations,
        optimisers,
    } = props;
    let possible_stations = *possible_stations;
    // Hooks must run on every render, so they come before the early return.
    let station_state = use_state_eq(|| None::<Station>);
    let outcome_state = use_state_eq(|| None::<Outcome>);
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
        |(station, possible_stations, optimisers)| {
            station.map(|station| {
                optimisers
                    .0
                    .iter()
                    .map(|optimiser| optimiser.worst_outcome(station, *possible_stations))
                    .collect::<Vec<_>>()
            })
        },
        (*station_state, possible_stations, optimisers.clone()),
    );
    if possible_stations.len() == 1 {
        return html! { <div class="container"><p>{format!("answer: {}", possible_stations.first().unwrap())}</p></div> };
    };
    let mut child = html! {};
    let mut rows = vec![];
    rows.push(vec![html! {
        <span class="small text-muted">{format!("{} possible stations", possible_stations.len())}</span>
    }]);
    {
        // Best guess row
        let columns = optimisers
            .0
            .iter()
            .zip(best_guesses.iter())
            .map(|(optimiser, (station, cost))| {
                let station = *station;
                let station_state = station_state.clone();
                let outcome_state = outcome_state.clone();
                let onclick = Callback::from(move |_| {
                    station_state.set(Some(station));
                    outcome_state.set(None);
                });
                suggestion(optimiser.as_ref(), station.to_string(), cost, onclick)
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
    if let (Some(station), Some(worst_outcomes)) = (*station_state, worst_outcomes.as_ref()) {
        let possible_outcomes = partition(station, possible_stations);
        {
            // Worst outcome row
            let columns = optimisers
                .0
                .iter()
                .zip(worst_outcomes)
                .map(|(optimiser, (outcome, cost))| {
                    let outcome = *outcome;
                    let outcome_state = outcome_state.clone();
                    let onclick = Callback::from(move |_| outcome_state.set(Some(outcome)));
                    let text = format!("worst case {}", outcome);
                    suggestion(optimiser.as_ref(), text, cost, onclick)
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
                // Key the child by what led to it so a different guess or
                // outcome starts it afresh instead of keeping stale state.
                let key = format!("{station:?}/{outcome:?}");
                child = html! { <App {key} ..props /> };
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
