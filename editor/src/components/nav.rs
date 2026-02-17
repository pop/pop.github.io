use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::routes::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

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
            if auth.token.is_some() {
                <button onclick={on_logout} class="logout-btn">{"Logout"}</button>
            } else {
                <Link<Route> to={Route::Login} classes="login-link">{"Login"}</Link<Route>>
            }
        </nav>
    }
}
