// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const COMMANDS: &[&str] = &[
    "write_text",
    "read_text",
    "write_image",
    "read_image",
    "write_html",
    "clear",
];

fn main() {
    if let Err(error) = tauri_plugin::Builder::new(COMMANDS)
        .global_api_script_path("./api-iife.js")
        .android_path("android")
        .ios_path("ios")
        .try_build()
    {
        println!("{error:#}");
        // when building documentation for Android the plugin build result is irrelevant to the crate itself
        if !(cfg!(docsrs) && std::env::var("TARGET").unwrap().contains("android")) {
            std::process::exit(1);
        }
    }
}
