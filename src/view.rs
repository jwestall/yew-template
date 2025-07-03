use yew::{html, Context, Html};

use crate::app::{App, Msg};

impl App {
    pub fn view_input(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::IncPressed)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <button onclick={ctx.link().callback(|_| Msg::DecPressed)}>{ "-1" }</button>
            </div>
        }
    }
}