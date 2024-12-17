use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SafeAreaInsetsExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.safe_area_insets().ping(payload)
}
