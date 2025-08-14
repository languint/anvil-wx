use anvil_core::{
    radar::RadarSite,
    warnings::{Warning, WarningPolygonPoint},
};
use chrono::Utc;
use web_sys::{wasm_bindgen::{prelude::Closure, JsCast}, CanvasRenderingContext2d};
use yew::{
    function_component, html, use_effect_with, use_node_ref, use_state, Html, KeyboardEvent, Properties, UseStateHandle, WheelEvent
};

#[derive(Properties, PartialEq)]
pub struct Radar2DProps {
    pub radar_site: UseStateHandle<RadarSite>,
    pub warnings: UseStateHandle<Vec<Warning>>,
}

pub fn render_radar_site(w: f64, h: f64, ctx: &CanvasRenderingContext2d) {
    ctx.set_line_width(2.0);
    ctx.set_stroke_style_str("#FFFFFF");
    ctx.round_rect_with_f64(w / 2.0 - 2.5, h / 2.0 - 2.5, 5.0, 5.0, 5.0)
        .unwrap();
    ctx.stroke();
}

pub fn render_warning_polygon(w: f64, h: f64, ctx: &CanvasRenderingContext2d, warning: &Warning) {
    let mut points: Vec<WarningPolygonPoint> = warning
        .polygon
        .points
        .iter()
        .map(|p| center_on_radar_site(w, h, *p))
        .collect();

    if points.len() > 2 {
        let (cx, cy) = {
            let sum_x: f32 = points.iter().map(|p| p.0).sum();
            let sum_y: f32 = points.iter().map(|p| p.1).sum();
            (sum_x / points.len() as f32, sum_y / points.len() as f32)
        };

        points.sort_by(|a, b| {
            let ang_a = (a.1 - cy).atan2(a.0 - cx);
            let ang_b = (b.1 - cy).atan2(b.0 - cx);
            ang_a.partial_cmp(&ang_b).unwrap()
        });
    }

    // Helper to draw polygon
    let draw_poly = |width: f64, color: &str| {
        ctx.set_line_width(width);
        ctx.set_stroke_style_str(color);
        ctx.begin_path();
        ctx.move_to(points[0].0.into(), points[0].1.into());
        for &(x, y) in points.iter().skip(1) {
            ctx.line_to(x.into(), y.into());
        }
        ctx.close_path();
        ctx.stroke();
    };

    // Outer border (thick)
    draw_poly(4.0, &warning.warning_type.get_color().outer.to_string());

    // Inner border (thin, sits on top)
    draw_poly(1.5, &warning.warning_type.get_color().inner.to_string());
}


pub fn center_on_radar_site(w: f64, h: f64, coord: WarningPolygonPoint) -> WarningPolygonPoint {
    (
        ((w / 2.0) + (coord.0 as f64) * 20.0) as f32,
        ((h / 2.0) + (coord.1 as f64) * 20.0) as f32,
    )
}


#[function_component]
pub fn Radar2D(props: &Radar2DProps) -> Html {
    let valid_warnings: Vec<Warning> = props
        .warnings
        .iter()
        .filter(|w| w.valid_until >= Utc::now())
        .cloned()
        .collect();

    let canvas_ref = use_node_ref();

    // Zoom and pan state
    let scale = use_state(|| 1.0);
    let offset_x = use_state(|| 0.0);
    let offset_y = use_state(|| 0.0);

    {
        // Handle scroll zoom
        let scale = scale.clone();
        let canvas_ref = canvas_ref.clone();

        use_effect_with(canvas_ref.clone(), move |_| {
            let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() else {todo!()};
            let closure = Closure::<dyn FnMut(_)>::wrap(Box::new(move |event: WheelEvent| {
                event.prevent_default();
                let delta = if event.delta_y() < 0.0 { 1.1 } else { 0.9 };
                scale.set((*scale) * delta);
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
            
            || {}
        });
    }

    {
        // Handle WASD pan
        let offset_x = offset_x.clone();
        let offset_y = offset_y.clone();

        use_effect_with((), move |_| {
            let closure = Closure::<dyn FnMut(_)>::wrap(Box::new(move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "w" | "ArrowUp" => offset_y.set(*offset_y - 10.0),
                    "s" | "ArrowDown" => offset_y.set(*offset_y + 10.0),
                    "a" | "ArrowLeft" => offset_x.set(*offset_x - 10.0),
                    "d" | "ArrowRight" => offset_x.set(*offset_x + 10.0),
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
            || {}
        });
    }

    {
        // Drawing effect
        let canvas_ref = canvas_ref.clone();
        let valid_warnings = valid_warnings.clone();
        let scale = *scale;
        let offset_x = *offset_x;
        let offset_y = *offset_y;

        use_effect_with((canvas_ref.clone(), valid_warnings.clone(), scale, offset_x, offset_y), move |_| {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let width = canvas.client_width() as f64;
                let height = canvas.client_height() as f64;
                canvas.set_width(width as u32);
                canvas.set_height(height as u32);

                ctx.clear_rect(0.0, 0.0, width, height);

                ctx.save();
                ctx.translate(width / 2.0 + offset_x, height / 2.0 + offset_y).unwrap();
                ctx.scale(scale, scale).unwrap();
                ctx.translate(-width / 2.0, -height / 2.0).unwrap();

                render_radar_site(width, height, &ctx);

                for warning in &valid_warnings {
                    render_warning_polygon(width, height, &ctx, warning);
                }

                ctx.restore();
            }
            || {}
        });
    }

    html! {
        <div class="radar-2d">
            <canvas id="radar-2d" ref={canvas_ref}/>
        </div>
    }
}