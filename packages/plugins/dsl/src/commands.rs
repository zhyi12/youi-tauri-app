// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command,Runtime,AppHandle};
use youi_dsl::{Param,default_df_execute};
use crate::error::Result;

#[command]
pub(crate) async fn execute<R: Runtime>(_app: AppHandle<R>,script:String,params:Vec<Param>) -> Result<String> {
    Ok(default_df_execute(&script,&params)?)
}
