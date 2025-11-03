use dioxus::{logger::tracing, prelude::*};

use crate::{components::alert_dialog::*, deserializer::WorldInfo};

mod components;
mod deserializer;
mod js_utils;

const CSS: Asset = asset!("/assets/main.css");

const UPSILON_JS: Asset = asset!(
    "/assets/libs/upsilon.bundle.js",
    JsAssetOptions::new().with_minify(false)
);

const DOWNLOAD_ICON_SVG: Asset = asset!("/assets/download.svg");
const DELETE_ICON_SVG: Asset = asset!("/assets/delete.svg");
const UPDATE_ICON_SVG: Asset = asset!("/assets/update.svg");

static LOGO: Asset = asset!("/assets/logo.svg");
static CONNECT_CALCULATOR_SVG: Asset = asset!("/assets/connect_calculator.svg");

#[derive(Debug, Clone)]
pub struct WorldRecord {
    pub record_index: usize,
    pub file_name: String,
    pub world_data: Vec<u8>,
    pub world_info: WorldInfo,
    pub need_remove: bool,
}

impl WorldRecord {
    pub fn new(
        record_index: usize,
        file_name: String,
        world_data: Vec<u8>,
        world_info: WorldInfo,
        need_remove: bool,
    ) -> Self {
        WorldRecord {
            record_index,
            file_name,
            world_data,
            world_info,
            need_remove,
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let supported_browser = js_utils::is_usb_supported();
    let calculator_connected = use_signal(|| false);
    let worlds_list: Signal<Vec<WorldRecord>> = use_signal(Vec::new);

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
    worlds_list: Signal<Vec<WorldRecord>>,
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
                    let array_length: usize = eval
                        .recv()
                        .await
                        .expect("An error occured during the download of the records.");
                    for _ in 0..array_length {
                        let index: usize = eval
                            .recv()
                            .await
                            .expect("An error occured during the download of the records.");
                        let name: String = eval
                            .recv()
                            .await
                            .expect("An error occured during the download of the records.");
                        let data: Vec<u8> = eval
                            .recv()
                            .await
                            .expect("An error occured during the download of the records.");
                        if let Some(world_info) = deserializer::get_world_info(&data) {
                            worlds_list
                                .write()
                                .push(WorldRecord::new(index, name, data, world_info, false));
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
    worlds_list: Signal<Vec<WorldRecord>>,
) -> Element {
    let mut open_delete_dialog = use_signal(|| false);
    let mut open_update_dialog = use_signal(|| false);
    let mut selected_world: Signal<Option<usize>> = use_signal(|| None);
    rsx!(
        div {
            id: "list-worlds-page-div",
            class: if *calculator_connected.read() { "page-spawn-fade-down" } else { "" },
            span { class: "connection-info", "Manage you Numcraft worlds." }

            div { id: "worlds-list-div",
                if (*worlds_list.read()).is_empty() {
                    span { id: "no-world-info", "There is no world on your calculator." }
                }
                for i in 0..(*worlds_list.read()).len() {
                    div {
                        key: "{i}",
                        class: "worlds-list-element",
                        class: if worlds_list.read()[i].need_remove { "removed-world-record" } else { "" },
                        span { class: "worlds-file-name",
                            {format!("File Name: {}.ncw", worlds_list.read()[i].file_name)}
                        }
                        span { class: "worlds-world-name",
                            {format!("World Name: {}", worlds_list.read()[i].world_info.world_name)}
                        }
                        span { class: "worlds-world-version",
                            {
                                format!(
                                    "Version: {}",
                                    worlds_list.read()[i].world_info.world_version.get_matching_name(),
                                )
                            }
                        }
                        a {
                            onclick: move |_| async move {
                                let record_index = worlds_list.read()[i].record_index;
                                document::eval(
                                        format!(
                                            r#"
                                            var record = window.storage.records[{record_index}];
                                            var blob = new Blob([record.data], {{
                                                type: "application/octet-stream",
                                            }});
                                            var link = document.createElement("a");
                                            link.href = window.URL.createObjectURL(blob);
                                            link.download = record.name + "." + record.type;
                                            link.click();
                                            return null;"#,
                                        )
                                            .as_str(),
                                    )
                                    .await
                                    .expect("Download failed.");
                            },
                            title: "Download",
                            img {
                                class: "world-button-icon",
                                src: DOWNLOAD_ICON_SVG,
                            }
                        }
                        a {
                            onclick: move |_| {
                                selected_world.set(Some(i));
                                open_delete_dialog.set(true)
                            },
                            title: "Delete",
                            img {
                                class: "world-button-icon",
                                src: DELETE_ICON_SVG,
                            }
                        }
                        a {
                            onclick: move |_| {
                                selected_world.set(Some(i));
                                open_update_dialog.set(true)
                            },
                            title: "Update to the latest version",
                            img {
                                class: "world-button-icon",
                                src: UPDATE_ICON_SVG,
                            }
                        }
                    }
                }
            }
        }
        AlertDialogRoot {
            open: *open_delete_dialog.read(),
            on_open_change: move |v| open_delete_dialog.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Are you sure?" }
                AlertDialogDescription {
                    {
                        if let Some(index) = *selected_world.read() {
                            format!(
                                "You are about to delete the world `{}`. This action cannot be undone!",
                                worlds_list.read()[index].world_info.world_name,
                            )
                        } else {
                            "".to_string()
                        }
                    }
                }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        on_click: move |_| async move {
                            let world_index = (*selected_world.read()).expect("The page is broken.");
                            if let Some(record) = worlds_list.write().get_mut(world_index) {
                                let record_index = record.record_index;
                                record.need_remove = true;
                                document::eval(
                                        format!(
                                            "window.storage.records.splice({record_index}, 1); await window.calculator.installStorage(window.storage, function () {{}}); return null;",
                                        )
                                            .as_str(),
                                    )
                                    .await
                                    .unwrap();
                            }
                            gloo_timers::future::TimeoutFuture::new(800).await;
                            worlds_list.write().remove(world_index);
                            selected_world.set(None);
                        },
                        "Delete"
                    }
                }
            }
        
        }
        AlertDialogRoot {
            open: *open_update_dialog.read(),
            on_open_change: move |v| open_update_dialog.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Are you sure?" }
                AlertDialogDescription {
                    {
                        if let Some(index) = *selected_world.read() {
                            format!(
                                "You are about to update the world `{}` to the latest game version. You should backup your world before doing this. This action cannot be undone!",
                                worlds_list.read()[index].world_info.world_name,
                            )
                        } else {
                            "".to_string()
                        }
                    }
                }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        on_click: move |_| async move {
                            let world_index = (*selected_world.read()).expect("The page is broken.");
                            if let Some(record) = worlds_list.write().get_mut(world_index) {
                                
                            }
                            selected_world.set(None);
                        },
                        "Update"
                    }
                }
            }
        }
    )
}
