use anvil_core::{
    radar::RadarSite,
    warnings::{SevereThunderStormWarning, TornadoWarning, Warning, WarningPolygon},
};
use chrono::Datelike;
use yew::prelude::*;

use crate::components::{
    radar::radar_grid::RadarGrid,
    sidebar::Sidebar,
    status_bar::{BottomStatusBar, TopStatusBar},
};

#[function_component(App)]
pub fn app() -> Html {
    let radar_site = use_state(|| RadarSite {
        identifier: "KLOT".to_string(),
    });

    let warnings = use_state(|| {
        vec![
            Warning {
                polygon: WarningPolygon {
                    points: vec![
                        (-2.0, 2.0),
                        (-2.0, -3.0),
                        (3.0, 2.0),
                        (3.0, 1.75),
                        (2.0, 2.75),
                        (2.0, 2.0),
                    ],
                },
                valid_until: chrono::Utc::now()
                    .with_year(200000)
                    .expect("Expected valid date"),
                warning_type: anvil_core::warnings::WarningType::SevereThunderStorm(
                    SevereThunderStormWarning {
                        damage_threat:
                            anvil_core::warnings::SevereThunderStormDamageThreat::Destructive,
                    },
                ),
            },
            Warning {
                polygon: WarningPolygon {
                    points: vec![(-0.2, 0.5), (-0.2, -0.5), (2.0, 0.75), (2.0, -0.75)],
                },
                valid_until: chrono::Utc::now()
                    .with_year(200000)
                    .expect("Expected valid date"),
                warning_type: anvil_core::warnings::WarningType::TornadoPDS(TornadoWarning {
                    tornado_status: anvil_core::warnings::TornadoStatus::Observed,
                }),
            },
        ]
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
