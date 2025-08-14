use anvil_core::{radar::RadarSite, warnings::Warning};
use yew::{function_component, html, Html, Properties, UseStateHandle};

use crate::components::radar::radar_2d::Radar2D;

#[derive(Properties, PartialEq)]
pub struct RadarGridProps {
    pub radar_site: UseStateHandle<RadarSite>,
    pub warnings: UseStateHandle<Vec<Warning>>,
}

#[function_component]
pub fn RadarGrid(props: &RadarGridProps) -> Html {
    html! {
        <div class="radar-grid">
            <Radar2D radar_site={props.radar_site.clone()} warnings={props.warnings.clone()}/>
            <Radar2D radar_site={props.radar_site.clone()} warnings={props.warnings.clone()}/>
            <Radar2D radar_site={props.radar_site.clone()} warnings={props.warnings.clone()}/>
            <Radar2D radar_site={props.radar_site.clone()} warnings={props.warnings.clone()}/>
        </div>
    }
}
