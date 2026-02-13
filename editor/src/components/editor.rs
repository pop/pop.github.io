use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
}

#[function_component(EditorPage)]
pub fn editor_page(props: &Props) -> Html {
    html! {
        <div class="placeholder">
            <h2>{"Editor"}</h2>
            <p>{format!("Editing: {}", props.path)}</p>
            <p>{"Editor coming in Phase 3."}</p>
        </div>
    }
}
