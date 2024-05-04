// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const COMMANDS: &[&str] = &[
    "execute",
];

fn main() {
    if let Err(error) = tauri_plugin::Builder::new(COMMANDS)
        .try_build()
    {
        println!("{error:#}");
    }
}
