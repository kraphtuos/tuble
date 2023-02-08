use std::fmt::Display;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SelectProps<T: PartialEq> {
    pub name: AttrValue,
    pub choices: Vec<T>,
    pub selected: Option<T>,
    pub submit: Callback<Option<T>>,
}

#[function_component]
pub fn SelectComponent<T: PartialEq + Display + Copy + 'static>(props: &SelectProps<T>) -> Html {
    let SelectProps {
        name,
        choices,
        selected,
        submit,
    } = props;

    let select_ref = use_node_ref();

    let options = {
        choices.iter().map(move |choice| {
            let selected = selected.as_ref() == Some(choice);
            html! {<option {selected}>{format!("{}", choice)}</option>}
        })
    };
    let onchange = {
        let select_ref = select_ref.clone();
        let choices = choices.clone();
        submit.reform(move |_| {
            select_ref
                .cast::<HtmlSelectElement>()
                .and_then(|select| choices.get(select.selected_index() as usize - 1))
                .copied()
        })
    };
    html! {
        <select class="form-select" aria-label={format!("select {}", name)} {onchange} ref={select_ref}>
            <option disabled=true selected={selected.is_none()}>{"-"}</option>
            {for options}
        </select>
    }
}
