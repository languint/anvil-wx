use anvil_core::radar::RadarSite;
use yew::{function_component, html, Html, Properties, UseStateHandle};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TopStatusBarProps {
    pub radar_site: UseStateHandle<RadarSite>,
}

#[function_component]
pub fn TopStatusBar(props: &TopStatusBarProps) -> Html {
    html! {
        <div class="status-bar top">
            <img src="assets/logo.svg" style={"width: 1.75rem; height: 1.75rem;"}/>
            <span>{props.radar_site.identifier.clone()}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BottomStatusBarProps {
    pub warnings: UseStateHandle<Vec<anvil_core::warnings::Warning>>,
}

#[function_component]
pub fn BottomStatusBar(props: &BottomStatusBarProps) -> Html {
    let number_of_svt = props.warnings.len();
    html! {
        <div class="status-bar bottom">
            <Icon icon_id={IconId::OcticonsAlert16} class={"icon"} style="color:#D7BA7D;"/>
            <span style="color:#D7BA7D;">{number_of_svt}</span>
            <Icon icon_id={IconId::OcticonsAlertFill12} class={"icon"} style="color:#F14C4C;"/>
            <span style="color:#F14C4C;">{number_of_svt}</span>
        </div>
    }
}
