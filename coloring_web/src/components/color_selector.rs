use material_yew::list::{GraphicType, MatListItem};
use material_yew::select::MatSelect;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use yew::prelude::*;

#[derive(Display, PartialEq, Eq, EnumIter, EnumString, Clone, Copy)]
pub enum Color {
    #[strum(serialize = "blueviolet")]
    BlueViolet,
    #[strum(serialize = "plum")]
    Plum,
    #[strum(serialize = "lightcoral")]
    LightCoral,
    #[strum(serialize = "firebrick")]
    FireBrick,
    #[strum(serialize = "lightpink")]
    LightPink,
    #[strum(serialize = "lightsalmon")]
    LightSalmon,
    #[strum(serialize = "orange")]
    Orange,
    #[strum(serialize = "lightgreen")]
    LightGreen,
    #[strum(serialize = "lightseagreen")]
    LightSeaGreen,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "lightblue")]
    LightBlue,
    #[strum(serialize = "blue")]
    Blue,
    #[strum(serialize = "rgb({0}, {1}, {2})")]
    Rgb(u8, u8, u8),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: UseStateHandle<Color>,
}

#[function_component]
pub fn ColorSelector(Props { value }: &Props) -> Html {
    let color_items = Color::iter().map(|c| {
        let c_value = c.to_string();
        let c_str = match c {
            Color::Rgb(_, _, _) => "custom".to_string(),
            c => c.to_string(),
        };

        html! {
            <MatListItem value={c_value} graphic={GraphicType::Icon}>{c_str}</MatListItem>
        }
    });

    html! {
        <div
            style={format!("{{
                --mdc-theme-primary: {} !important;
                --mdc-select-label-ink-color: {};
                --mdc-select-dropdown-icon-color: {};
            }}", **value, **value, **value)}
        >
            <MatSelect
                label="Highlight Color"
                outlined=true
                icon="font_download"
            >
                { for color_items }
            </MatSelect>
            if let c @ Color::Rgb(_, _, _) = **value {
                <input type="color" value={c.to_string()} />
            }
        </div>
    }
}
