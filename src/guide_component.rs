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
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-2 rounded cursor-pointer",
            onclick: move |_| {
                info!("Button clicked");
            },
            "Click me"x
        }
    }
}
