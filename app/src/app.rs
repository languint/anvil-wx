use anvil_core::{
    radar::RadarSite,
    warnings::{TornadoWarning, Warning, WarningPolygon},
};
use yew::prelude::*;

use crate::components::{
    radar_grid::RadarGrid,
    sidebar::Sidebar,
    status_bar::{BottomStatusBar, TopStatusBar},
};

#[function_component(App)]
pub fn app() -> Html {
    let radar_site = use_state(|| RadarSite {
        identifier: "KLOT".to_string(),
    });

    let warnings = use_state(|| {
        vec![Warning {
            polygon: WarningPolygon {},
            valid_until: chrono::Utc::now(),
            warning_type: anvil_core::warnings::WarningType::Tornado(TornadoWarning {
                tornado_status: anvil_core::warnings::TornadoStatus::Observed,
            }),
        }]
    });
    html! {
        <main class="main-container">
            <div class="main-container-contents">
                <Sidebar />
                <RadarGrid radar_site={radar_site.clone()} warnings={warnings.clone()}/>
            </div>
            <TopStatusBar {radar_site}/>
            <BottomStatusBar {warnings}/>
        </main>
    }
}
