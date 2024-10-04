use crate::models::ButtonType;
use dioxus::prelude::*;

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    button_type: ButtonType,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element {
    cx.render(rsx! {
        button {
            class: "{button_type.to_string()}",
            onclick: move |_event| onclick.call(_event),
            children,
        }
    })
}
