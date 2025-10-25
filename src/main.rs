use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/main.css");

static LOGO: Asset = asset!("/assets/logo.svg");
static CONNECT_CALCULATOR_SVG: Asset = asset!("/assets/connect_calculator.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let supported_browser = true;
    let calculator_connected = false;
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "header-bar",
            img { src: LOGO, id: "website-logo" }
            h1 { id: "title", "Numcraft" }
        }
        div { id: "page-body",
            if supported_browser {
                span { class: "connection-info", "Please connect your calculator to your computer" }

                img { id: "connect-calculator-svg", src: CONNECT_CALCULATOR_SVG }
            } else {
                span { class: "connection-info",
                    "Your browser is not supported. Please use a Chromium based browser."
                }
            }
        }
    }
}
