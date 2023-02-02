use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropdownProps<T: PartialEq> {
    pub choices: Vec<T>,
    pub submit: Callback<Option<T>>,
}

#[function_component]
pub fn DropdownComponent<T: PartialEq + Display + Copy + 'static>(
    props: &DropdownProps<T>,
) -> Html {
    let DropdownProps { choices, submit } = props;

    let options = choices
        .iter()
        .map(|choice| {
            let choice = choice.clone();
            let onclick = submit.reform(move |_| Some(choice));
            html! {<li><a class="dropdown-item" {onclick}>{format!("{}", choice)}</a></li>}
        })
        .collect::<Html>();
    let onclick = submit.reform(|_| None::<T>);
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
}
