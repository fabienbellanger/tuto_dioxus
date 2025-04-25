use dioxus::prelude::*;

#[component]
pub fn DogApp(breed: String) -> Element {
    rsx! {
        div {
            class: "text-red-500 text-center cursor-pointer",
            onclick: move |_| {
                info!("DogApp clicked");
            },
            "DogApp: breed = {breed}"
        }
    }
}
