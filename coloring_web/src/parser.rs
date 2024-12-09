use crate::components::color_selector::Color;
use coloring_common::FragSpecs;

pub struct FragItems {
    pub frag: FragSpecs,
    pub color: Color,
}

pub struct AppState {
    pub target_frags: Vec<FragSpecs>,
}
