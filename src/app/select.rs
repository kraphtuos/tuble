use std::fmt::Display;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SelectProps<K: PartialEq, T: PartialEq> {
    pub name: AttrValue,
    pub placeholder: AttrValue,
    pub choices: Vec<(K, T)>,
    pub selected: Option<K>,
    pub submit: Callback<Option<K>>,
}

#[function_component]
pub fn SelectComponent<K: PartialEq + Copy + 'static, T: PartialEq + Display + Copy + 'static>(
    props: &SelectProps<K, T>,
) -> Html {
    let SelectProps {
        name,
        placeholder,
        choices,
        selected,
        submit,
    } = props;

    let select_ref = use_node_ref();

    let options = {
        choices.iter().map(move |(key, value)| {
            let selected = selected.as_ref() == Some(key);
            html! {<option {selected}>{format!("{}", value)}</option>}
        })
    };
    let onchange = {
        let select_ref = select_ref.clone();
        let choices = choices.clone();
        submit.reform(move |_| {
            select_ref
                .cast::<HtmlSelectElement>()
                .and_then(|select| choices.get(select.selected_index() as usize - 1))
                .map(|x| x.0)
        })
    };
    html! {
        <select class="form-select" aria-label={format!("select {}", name)} {onchange} ref={select_ref}>
            <option disabled=true selected={selected.is_none()}>{placeholder}</option>
            {for options}
        </select>
    }
}
