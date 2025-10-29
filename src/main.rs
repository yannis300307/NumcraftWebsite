use dioxus::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod js_utils;

const CSS: Asset = asset!("/assets/main.css");

const UPSILON_JS: Asset = asset!(
    "/assets/libs/upsilon.bundle.js",
    JsAssetOptions::new().with_minify(false)
);

static LOGO: Asset = asset!("/assets/logo.svg");
static CONNECT_CALCULATOR_SVG: Asset = asset!("/assets/connect_calculator.svg");

fn main() {
    dioxus::launch(App);
}

#[wasm_bindgen]
pub fn raise_error(error: &str) {
    println!("bad");
}

#[component]
fn App() -> Element {
    let supported_browser = js_utils::is_usb_supported();

    rsx! {
        document::Stylesheet { href: CSS }
        document::Script { src: UPSILON_JS }
        div { id: "header-bar",
            img { src: LOGO, id: "website-logo" }
            h1 { id: "title", "Numcraft" }
        }
        div { id: "page-body",
            if supported_browser {
                ConnectPage {}
            } else {
                span { class: "connection-info",
                    "Your browser is not supported. Please use a Chromium based browser."
                }
            }
        }
    }
}

#[component]
fn ConnectPage() -> Element {
    let mut calculator_connected = use_signal(|| false);

    rsx!(
        div {
            id: "connect-page-div",
            class: if *calculator_connected.read() { "fade-down-despawn" } else { "" },
            span { class: "connection-info", "Please connect your calculator to your computer" }

            button {
                id: "connect-button",
                onclick: move |_| async move {
                    let mut eval = document::eval(
                        r#"if (window.calculator === undefined) {window.calculator = new window.Upsilon()};
                                        calculator.detect(function() {
                                            dioxus.send(true);
                                        }, function(error) {
                                            dioxus.send(false);
                                        });"#,
                    );
                    let result: bool = eval.recv().await.expect("Page has not loaded correctly.");
                    *calculator_connected.write() = result;
                },
                "Detect calculator"
            }
            img { id: "connect-calculator-svg", src: CONNECT_CALCULATOR_SVG }
        }
    )
}

