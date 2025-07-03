use yew::{html, Component, Context, Html};

pub enum Msg {
    IncPressed,
    DecPressed,
}

pub struct App {
    pub value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::IncPressed => {
                self.value += 1;
            }
            Msg::DecPressed => {
                self.value -= 1;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="contents">
                <h1>{ "Counter" }</h1>

                { self.view_input(ctx) }

            </div>
        }
    }
}