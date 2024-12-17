use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<SafeAreaInsets<R>> {
  Ok(SafeAreaInsets(app.clone()))
}

/// Access to the safe-area-insets APIs.
pub struct SafeAreaInsets<R: Runtime>(AppHandle<R>);

impl<R: Runtime> SafeAreaInsets<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
