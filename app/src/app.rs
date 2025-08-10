use anvil_core::warnings::{TornadoWarning, Warning, WarningPolygon};
use yew::prelude::*;

use crate::components::status_bar::{BottomStatusBar, TopStatusBar};

#[function_component(App)]
pub fn app() -> Html {
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
            <TopStatusBar />
            <BottomStatusBar {warnings}/>
        </main>
    }
}
