use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
}

#[function_component(Preview)]
pub fn preview(props: &Props) -> Html {
    html! {
        <div class="placeholder">
            <h2>{"Preview"}</h2>
            <p>{format!("Previewing: {}", props.path)}</p>
            <p>{"Preview coming in Phase 4."}</p>
        </div>
    }
}
