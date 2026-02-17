use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/edit/*path")]
    Editor { path: String },
    #[at("/preview/*path")]
    Preview { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
