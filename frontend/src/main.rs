use yew::prelude::*;
use yew_router::prelude::*;

mod bridge;
mod comps;
mod data;
mod pages;

use bridge::is_logged_in;
use data::*;
use pages::*;

pub struct App {
    auth_state: bool,
}

pub enum Msg {
    UpdateAuthStateAsync,
    UpdateAuthState(bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { auth_state: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.auth_state == false {
            return html! {
                <LoginPage />
            };
        }

        html! {


            <BrowserRouter>
                <Switch<PageRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAuthState(s) => {
                self.auth_state = s;
            }
            Msg::UpdateAuthStateAsync => {
                ctx.link().send_future(App::change_auth_state());
            }
        }

        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::UpdateAuthStateAsync);
        }
    }
}

impl App {
    pub async fn change_auth_state() -> Msg {
        Msg::UpdateAuthState(is_logged_in().await.as_bool().unwrap())
    }
}

fn switch(routes: &PageRoute) -> Html {
    match routes {
        PageRoute::Login => html! {<LoginPage />},
        PageRoute::Home => html! {<LoginPage/>},
        PageRoute::Admin => html! {<AdminPage/> },
    }
}

fn main() {
    yew::start_app::<App>();
}
