use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::routes::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    // Redirect to login if not authenticated
    {
        let navigator = navigator.clone();
        let token = auth.token.clone();
        use_effect_with(token, move |token| {
            if token.is_none() {
                navigator.push(&Route::Login);
            }
            || ()
        });
    }

    html! {
        <div class="placeholder">
            <h2>{"Dashboard"}</h2>
            <p>{"Content browser coming in Phase 2."}</p>
        </div>
    }
}
