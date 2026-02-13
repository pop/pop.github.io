use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::routes::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if auth.token.is_none() {
        return html! {};
    }

    let on_logout = {
        let set_token = auth.set_token.clone();
        Callback::from(move |_: MouseEvent| {
            set_token.emit(None);
        })
    };

    html! {
        <nav>
            <div class="nav-links">
                <Link<Route> to={Route::Dashboard}>{"Dashboard"}</Link<Route>>
            </div>
            <button onclick={on_logout} class="logout-btn">{"Logout"}</button>
        </nav>
    }
}
