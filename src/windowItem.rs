use std::ops::Deref;
use std::sync::Arc;

use dioxus::prelude::*;
use dioxus::prelude::{component, rsx, Element};

use crate::{Action, HoverProps};

#[component]
pub fn WindowItem(action: ReadOnlySignal<Action>, position: i64, onClick: EventHandler<Action>, onhover: EventHandler<HoverProps>, selected: ReadOnlySignal<Option<Action>>) -> Element {
    let mut transform = use_signal(|| "".to_string());
    let mut opacity = use_signal(|| 1.0);
    let mut z = use_signal(|| 0);


    use_effect(move || {
        if let Some(selected) = selected() {
            *transform.write() = format!("translateX(calc({position}*(100% + 40px)))");
            if action != selected {
                *opacity.write() = 0.0;
                *z.write() = -9999;
            }
        } else {
            *transform.write() = "".to_string();
            *opacity.write() = 1.0;
            *z.write() = 0;
        }
    });

    rsx! {
        div {
            onmouseenter: move |_| onhover.call(HoverProps{inside: true,  action: action()}),
            onmouseleave: move |_| onhover.call(HoverProps{inside: false, action: action()}),
            onclick: move |_| {
                onClick.call(action());
            },
            class: "vbox window-item",
            transform: transform,
            opacity: opacity,
            z_index: z,
            img { 
                src: action().get_image(), class: "image-with-text"
            },
            div { font_size: "28px",
                "{action}"
            }
        }
    }
}