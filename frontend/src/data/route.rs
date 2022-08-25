use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum PageRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
}
