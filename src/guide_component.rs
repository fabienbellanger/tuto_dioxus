use dioxus::prelude::*;

#[component]
pub fn DogApp(breed: String) -> Element {
    rsx! {
        div {
            class: "text-gray-100 text-center cursor-pointer",
            onclick: move |_| {
                info!("DogApp clicked");
            },
            "DogApp: breed = {breed}"
        }
        button {
            class: "btn btn-primary",
            onclick: move |_| {
                info!("Button clicked");
            },
            "Click me"
        }
    }
}
