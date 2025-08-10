use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Panel(props: &PanelProps) -> Html {
    html! {
        <div class="panel">
            { for props.children.iter() }
        </div>
    }
}