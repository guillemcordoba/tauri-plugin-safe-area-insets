// Copyright 2024 Your Name
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(mobile)]

use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;
mod error;
mod models;
pub use error::{Error, Result};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.safeareainsets";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_safe_area_insets);

/// Access to the safe-area-insets APIs.
pub struct SafeAreaInsets<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SafeAreaInsets<R> {
    pub fn get_insets(&self) -> Result<Insets> {
        self.0
            .run_mobile_plugin("getInsets", ())
            .map_err(Into::into)
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the safe-area-insets APIs.
pub trait SafeAreaInsetsExt<R: Runtime> {
    fn safe_area_insets(&self) -> &SafeAreaInsets<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SafeAreaInsetsExt<R> for T {
    fn safe_area_insets(&self) -> &SafeAreaInsets<R> {
        self.state::<SafeAreaInsets<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("safe-area-insets")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "InsetPlugin")?;
            #[cfg(target_os = "ios")]
            let handle = api.register_ios_plugin(init_plugin_safe_area_insets)?;
            app.manage(SafeAreaInsets(handle));
            Ok(())
        })
        .build()
}
