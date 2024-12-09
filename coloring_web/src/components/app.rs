use crate::components::color_selector::{Color, ColorSelector};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let color = use_state(|| Color::Green);

    html! {
        <>
            <ColorSelector value={color} />
        </>
    }
}
