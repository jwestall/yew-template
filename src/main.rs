mod app;
mod view;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
