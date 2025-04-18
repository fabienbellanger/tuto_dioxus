use dioxus::prelude::*;

#[component]
pub fn DogApp(breed: String) -> Element {
    rsx! { "Breed: {breed}" }
}
