mod guide_component;

use dioxus::prelude::*;
use guide_component::DogApp;

#[macro_use]
extern crate tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");

fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).expect("Failed to initialize logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Dioxus" }

        document::Meta { name: "charset", content: "UTF-8" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        document::Meta {
            name: "description",
            content: "Dioxus is a fast, portable, and ergonomic framework for building user interfaces.",
        }
        document::Meta { name: "keywords", content: "dioxus, rust, web, framework" }
        document::Meta { name: "author", content: "Dioxus Labs" }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "flex flex-col items-center justify-center h-screen p-4 bg-gray-800",

            DogApp { breed: "bulldog".to_string() }
        }
    }
}
