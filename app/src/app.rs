use crate::components::panel::Panel;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let panels = use_state(|| vec![0, 0]);
    let is_dragging = use_state(|| false);
    let start_pos = use_state(|| 0);
    let start_panel_size = use_state(|| 0);

    let container_ref = use_node_ref();

    {
        let panels = panels.clone();
        let container_ref = container_ref.clone();
        use_effect_with(container_ref, move |container_ref| {
            if let Some(element) = container_ref.cast::<web_sys::HtmlElement>() {
                let width = element.client_width();
                panels.set(vec![width / 2, width / 2]);
            }
            || {}
        });
    }

    let on_resize_start = {
        let is_dragging = is_dragging.clone();
        let start_pos = start_pos.clone();
        let start_panel_size = start_panel_size.clone();
        let panels_clone = panels.clone();
        Callback::from(move |event: MouseEvent| {
            is_dragging.set(true);
            start_pos.set(event.client_x());
            start_panel_size.set(panels_clone[0]);
            event.prevent_default();
        })
    };

    let on_resize_move = {
        let is_dragging = is_dragging.clone();
        let start_pos = start_pos.clone();
        let start_panel_size = start_panel_size.clone();
        let panels_clone = panels.clone();
        let container_ref = container_ref.clone();

        Callback::from(move |event: MouseEvent| {
            if *is_dragging {
                if let Some(element) = container_ref.cast::<web_sys::HtmlElement>() {
                    let container_width = element.client_width();
                    let delta = event.client_x() - *start_pos;
                    let new_size = *start_panel_size + delta;

                    let new_size = new_size.max(0).min(container_width);

                    let mut new_panels = panels_clone.to_vec();
                    new_panels[0] = new_size;
                    new_panels[1] = container_width - new_size;
                    panels_clone.set(new_panels);
                }
            }
        })
    };

    let on_resize_end = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_: MouseEvent| {
            is_dragging.set(false);
        })
    };

    html! {
        <main
            class="panel-container"
            ref={container_ref}
            onmousemove={on_resize_move}
            onmouseup={on_resize_end}
        >
            <Panel size={panels[0]}><a /></Panel>
            <div class="panel-gutter" onmousedown={on_resize_start} />
            <Panel size={panels[1]}><a /></Panel>
        </main>
    }
}
