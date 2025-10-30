use dioxus::{logger::tracing, prelude::*};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::deserializer::WorldInfo;

mod deserializer;
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

#[component]
fn App() -> Element {
    let supported_browser = js_utils::is_usb_supported();
    let calculator_connected = use_signal(|| false);
    let worlds_list: Signal<Vec<(usize, String, Vec<u8>, WorldInfo)>> = use_signal(|| Vec::new());

    rsx! {
        document::Stylesheet { href: CSS }
        document::Script { src: UPSILON_JS }
        div { id: "header-bar",
            img { src: LOGO, id: "website-logo" }
            h1 { id: "title", "Numcraft" }
        }
        div { id: "page-body",
            if supported_browser {
                ConnectPage { calculator_connected, worlds_list }
                ListWorldsPage { calculator_connected, worlds_list }
            } else {
                span { class: "connection-info",
                    "Your browser is not supported. Please use a Chromium based browser."
                }
            }
        }
    }
}

#[component]
fn ConnectPage(
    calculator_connected: Signal<bool>,
    worlds_list: Signal<Vec<(usize, String, Vec<u8>, WorldInfo)>>,
) -> Element {
    rsx!(
        div {
            id: "connect-page-div",
            class: if *calculator_connected.read() { "fade-down-despawn" } else { "" },
            span { class: "connection-info", "Please connect you calculator to your computer." }

            button {
                id: "connect-button",
                onclick: move |_| async move {
                    let mut eval = document::eval(
                        r#"if (window.calculator === undefined) {window.calculator = new window.Upsilon()};
                            await window.calculator.detect(async function() {
                                window.storage = await window.calculator.backupStorage();
                                dioxus.send(true);

                                let length = window.storage.records.length;
                                dioxus.send(length);
                                for (let i = 0; i < length; i++) {
                                    if (window.storage.records[i].type == "ncw") {
                                        dioxus.send(i);
                                        dioxus.send(window.storage.records[i].name);
                                        let data = Array.from(new Uint8Array(await window.storage.records[i].data.arrayBuffer()));
                                        dioxus.send(data);
                                    }
                                }
                            }, function(error) {
                                dioxus.send(false);
                            });
                            "#,
                    );
                    let connected: bool = eval.recv().await.expect("Page has not loaded correctly.");
                    *calculator_connected.write() = connected;

                    let array_length: usize = eval.recv().await.expect("An error occured during the download of the records.");
                    for _ in 0..array_length {
                        let index: usize = eval.recv().await.expect("An error occured during the download of the records.");
                        let name: String = eval.recv().await.expect("An error occured during the download of the records.");
                        let data: Vec<u8> = eval.recv().await.expect("An error occured during the download of the records.");
                        if let Some(world_info) = deserializer::get_world_info(&data) {
                            worlds_list.write().push((index, name, data, world_info));
                        } else {
                            tracing::warn!("Invalid world info detected.");
                        }
                    }
                    document::eval(format!("console.log('{:?}')", worlds_list).as_str());
                },
                "Detect calculator"
            }
            img { id: "connect-calculator-svg", src: CONNECT_CALCULATOR_SVG }
        }
    )
}

#[component]
fn ListWorldsPage(
    calculator_connected: Signal<bool>,
    worlds_list: Signal<Vec<(usize, String, Vec<u8>, WorldInfo)>>,
) -> Element {
    rsx!(
        div {
            id: "list-worlds-page-div",
            class: if *calculator_connected.read() { "page-spawn-fade-down" } else { "" },
            span { class: "connection-info", "Manage you Numcraft worlds." }

            div { id: "worlds-list-div",
                    for i in 0..(*worlds_list.read()).len() {
                        div { key: format!("worlds-list-div-{}", i), class: "worlds-list-element",
                        span { {format!("{}.ncw", worlds_list.read()[i].1) } }
                        span { {format!("{}", worlds_list.read()[i].3.world_name) } }
                        span { {format!("{}", worlds_list.read()[i].3.world_version.get_matching_name()) } }
                    }
                }
            }
        }
    )
}
