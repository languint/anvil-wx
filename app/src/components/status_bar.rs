use yew::{function_component, html, Html, Properties, UseStateHandle};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TopStatusBarProps {}

#[function_component]
pub fn TopStatusBar(props: &TopStatusBarProps) -> Html {
    html! {
        <div class="status-bar top">

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
            <Icon icon_id={IconId::OcticonsAlert16} class={"icon"} style="color:#FFD866;"/>
            <span style="color:#FFD866;">{number_of_svt}</span>
            <Icon icon_id={IconId::OcticonsAlertFill12} class={"icon"} style="color:#FF6188;"/>
            <span style="color:#FF6188;">{number_of_svt}</span>
        </div>
    }
}
