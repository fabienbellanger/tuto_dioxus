use dioxus::prelude::*;

#[component]
pub fn DogApp(breed: String) -> Element {
    rsx! {
        div { class: "text-center", "DogApp: breed = {breed}" }
        button {
            class: "btn btn-soft btn-primary m-2",
            onclick: move |_| {
                info!("Button clicked");
            },
            "Click me"
        }
    }
}
