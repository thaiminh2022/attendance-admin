use super::super::bridge::sign_in;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct LoginPage {
    email: String,
    password: String,
}

pub enum Msg {
    Login,
    Refresh,

    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_login = link.callback(|_| Msg::Login);

        let on_login_email_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdateEmail(input.value()))
        });

        let on_login_password_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdatePassword(input.value()))
        });

        html! {
            <section class="lg">
                <header class="header">
                    <h1>{"Login"}</h1>
                    <strong>{"Use your given admin accout to login"}</strong>
                </header>
                <main class="login-main">
                    <div class = "login-form">
                        <div class="inputs">
                            <input type="email" placeholder = "your given email"
                             onchange = {on_login_email_change}
                             value = {self.email.clone()}
                            />
                            <input type="password" placeholder = "your given password"
                             onchange = {on_login_password_change}
                             value = {self.password.clone()}
                            />
                        </div>
                        <aside class ="login-msg">
                            <strong>{"here's a login message"}</strong>
                        </aside>
                        <div class="btns">
                            <button class="login-btn" onclick = {on_login} >{"Login"}</button>
                        </div>
                    </div>
                </main>
            </section>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::Login => {
                // Login with async
                link.send_future(LoginPage::login_email_password(
                    self.email.clone(),
                    self.password.clone(),
                ))
            }
            Msg::UpdateEmail(s) => {
                self.email = s;
            }
            Msg::UpdatePassword(s) => {
                self.password = s;
            }
            Msg::Refresh => {}
        }

        true
    }
}

impl LoginPage {
    async fn login_email_password(email: String, password: String) -> Msg {
        sign_in(&email, &password).await.unwrap();

        return Msg::Refresh;
    }
}
