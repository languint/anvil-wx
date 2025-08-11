use anvil_core::{radar::RadarSite, warnings::Warning};
use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Radar2DProps {
    pub radar_site: UseStateHandle<RadarSite>,
    pub warnings: UseStateHandle<Vec<Warning>>,
}

#[function_component]
pub fn Radar2D(props: &Radar2DProps) -> Html {
    html! {
        <div class="radar-2d">
            <canvas id="radar-2d" />
        </div>
    }
}
