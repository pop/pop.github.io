use yew::prelude::*;
use crate::config::SplashConfig;

#[derive(Properties, PartialEq)]
pub struct SplashProps {
    pub config: SplashConfig,
    pub on_dismiss: Callback<()>,
}

#[function_component(Splash)]
pub fn splash(props: &SplashProps) -> Html {
    let on_click = {
        let cb = props.on_dismiss.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    html! {
        <div class="splash-backdrop" onclick={on_click}>
            <div class="splash-content">
                <div class="splash-title-row">
                    <span class="splash-title">{ &props.config.title }</span>
                    <img class="splash-logo" src={props.config.logo.clone()} alt="logo" />
                </div>
                <p class="splash-hint">{ "Click to continue" }</p>
            </div>
        </div>
    }
}
