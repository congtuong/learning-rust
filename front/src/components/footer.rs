use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
    footer{
        class: "sticky bottom-0 flex flex-row justify-center items-center bg-blue-300 w-full h-16",
            a {
                class: "h-full w-auto p-2",
                img {
                    class: "bg-transparent animate-jump h-full w-auto",
                    alt: "devbcn",
                    src: "devbcn.png",
                    "loading": "lazy"
                }
            }

            a {
                class: "h-full w-auto p-2",
                img {
                    class: "bg-transparent animate-jump h-full w-auto",
                    alt: "rustcity",
                    src: "rustcity.png",
                    "loading": "lazy"
                }
            }
        }
    })
}
