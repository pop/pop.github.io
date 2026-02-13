use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{dashboard::Dashboard, editor::EditorPage, login::Login, nav::Nav, preview::Preview};
use crate::routes::Route;
use crate::services::auth;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub set_token: Callback<Option<String>>,
}

#[function_component(App)]
pub fn app() -> Html {
    let token = use_state(|| auth::get_token());

    let set_token = {
        let token = token.clone();
        Callback::from(move |new_token: Option<String>| {
            match &new_token {
                Some(t) => auth::store_token(t),
                None => auth::clear_token(),
            }
            token.set(new_token);
        })
    };

    let auth_ctx = AuthContext {
        token: (*token).clone(),
        set_token,
    };

    html! {
        <ContextProvider<AuthContext> context={auth_ctx}>
            <BrowserRouter>
                <Nav />
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </ContextProvider<AuthContext>>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Editor { path } => html! { <EditorPage path={path} /> },
        Route::Preview { path } => html! { <Preview path={path} /> },
        Route::NotFound => html! { <h1>{"404 — Not Found"}</h1> },
    }
}
