use std::rc::Rc;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::{panel::Panel, radar_2d::Radar2D};

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Container {
        direction: Direction,
        children: Vec<Node>,
        sizes: Vec<f32>,
    },
    Leaf {
        content: Html,
    },
}

fn render_node(
    node: &Node,
    path: Vec<usize>,
    on_resize_start: Rc<Callback<(MouseEvent, Vec<usize>, usize)>>,
) -> Html {
    match node {
        Node::Container {
            direction,
            children,
            sizes,
            ..
        } => {
            let flex_direction = if matches!(direction, Direction::Horizontal) {
                "row"
            } else {
                "column"
            };

            html! {
                <div class="panel-container" style={format!("flex-direction: {};", flex_direction)}>
                    { for children.iter().enumerate().map(|(i, child)| {
                        let mut child_path = path.clone();
                        child_path.push(i);
                        let on_resize_start_clone = on_resize_start.clone();
                        let path_clone = path.clone();

                        html! {
                            <>
                                <div class="panel-wrapper" style={format!("flex-basis: {}%", sizes[i] * 100.0)}>
                                    { render_node(child, child_path, on_resize_start.clone()) }
                                </div>
                                if i < children.len() - 1 {
                                    <div
                                        class={format!("panel-gutter {}", if matches!(direction, Direction::Vertical) { "vertical" } else { "horizontal" })}
                                        onmousedown={Callback::from(move |e: MouseEvent| {
                                            on_resize_start_clone.emit((e, path_clone.clone(), i));
                                        })}
                                    />
                                }
                            </>
                        }
                    })}
                </div>
            }
        }
        Node::Leaf { content } => {
            html! {
                <Panel>{ content.clone() }</Panel>
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let root = use_state(|| Node::Container {
        direction: Direction::Horizontal,
        children: vec![
            Node::Container {
                direction: Direction::Vertical,
                children: vec![
                    Node::Leaf {
                        content: html! { <Radar2D /> },
                    },
                    Node::Leaf {
                        content: html! { <Radar2D /> },
                    },
                ],
                sizes: vec![0.5, 0.5],
            },
            Node::Leaf {
                content: html! { <Radar2D /> },
            },
        ],
        sizes: vec![0.5, 0.5],
    });

    let dragging_gutter = use_state(|| None::<(Vec<usize>, usize)>);
    let start_pos = use_state(|| (0, 0));
    let start_sizes = use_state(|| (0.0, 0.0));
    let container_ref = use_node_ref();

    let on_resize_start = {
        let dragging_gutter = dragging_gutter.clone();
        let start_pos = start_pos.clone();
        let start_sizes = start_sizes.clone();
        let root = root.clone();

        Rc::new(Callback::from(
            move |(e, path, index): (MouseEvent, Vec<usize>, usize)| {
                e.prevent_default();
                dragging_gutter.set(Some((path.clone(), index)));
                start_pos.set((e.client_x(), e.client_y()));

                let mut node = &*root;
                for &i in &path {
                    if let Node::Container { children, .. } = node {
                        node = &children[i];
                    }
                }

                if let Node::Container { sizes, .. } = node {
                    start_sizes.set((sizes[index] as f64, sizes[index + 1] as f64));
                }
            },
        ))
    };

    let on_resize_move = {
        let dragging_gutter = dragging_gutter.clone();
        let start_pos = start_pos.clone();
        let start_sizes = start_sizes.clone();
        let root = root.clone();
        let container_ref = container_ref.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some((path, index)) = &*dragging_gutter {
                if let Some(element) = container_ref.cast::<HtmlElement>() {
                    let mut container_pixel_width = element.client_width() as f64;
                    let mut container_pixel_height = element.client_height() as f64;

                    let mut node_for_sizing = &*root;
                    for &i in path.iter() {
                        if let Node::Container {
                            direction,
                            children,
                            sizes,
                            ..
                        } = node_for_sizing
                        {
                            match direction {
                                Direction::Horizontal => {
                                    container_pixel_width *= sizes[i] as f64;
                                }
                                Direction::Vertical => {
                                    container_pixel_height *= sizes[i] as f64;
                                }
                            }
                            node_for_sizing = &children[i];
                        }
                    }

                    let mut new_root = (*root).clone();
                    let mut container_to_mutate = &mut new_root;
                    for &i in path.iter() {
                        if let Node::Container { children, .. } = container_to_mutate {
                            container_to_mutate = &mut children[i];
                        }
                    }

                    if let Node::Container {
                        direction, sizes, ..
                    } = container_to_mutate
                    {
                        let (delta, container_dimension) = match direction {
                            Direction::Horizontal => (
                                (e.client_x() - start_pos.0) as f64,
                                container_pixel_width,
                            ),
                            Direction::Vertical => (
                                (e.client_y() - start_pos.1) as f64,
                                container_pixel_height,
                            ),
                        };

                        if container_dimension == 0.0 {
                            return;
                        }

                        let delta_fraction = delta / container_dimension;
                        let mut new_size1 = start_sizes.0 + delta_fraction;
                        let mut new_size2 = start_sizes.1 - delta_fraction;

                        let min_size = 0.05;
                        if new_size1 < min_size {
                            new_size1 = min_size;
                            new_size2 = start_sizes.0 + start_sizes.1 - new_size1;
                        }
                        if new_size2 < min_size {
                            new_size2 = min_size;
                            new_size1 = start_sizes.0 + start_sizes.1 - new_size2;
                        }

                        sizes[*index] = new_size1 as f32;
                        sizes[*index + 1] = new_size2 as f32;

                        root.set(new_root);
                    }
                }
            }
        })
    };

    let on_resize_end = {
        let dragging_gutter = dragging_gutter.clone();
        Callback::from(move |_: MouseEvent| {
            dragging_gutter.set(None);
        })
    };

    html! {
        <main
            class="main-container"
            ref={container_ref}
            onmousemove={on_resize_move}
            onmouseup={on_resize_end.clone()}
            onmouseleave={on_resize_end}
        >
            { render_node(&*root, vec![], on_resize_start) }
        </main>
    }
}