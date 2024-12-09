use components::app::App;

mod components;
mod parser;

fn main() {
    yew::Renderer::<App>::new().render();
}
