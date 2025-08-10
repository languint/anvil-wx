use yew::{function_component, html, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct PanelProps {
    pub size: i32,
    pub children: yew::Children,
}

#[function_component]
pub fn Panel(props: &PanelProps) -> Html {
    let style = format!("width: {}px;", props.size);
    html! {
        <div class="panel" style={style}>
            { for props.children.iter() }
        </div>
    }
}
