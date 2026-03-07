//! Standards-based same-origin cross-context synchronization helpers.

use serde::{Deserialize, Serialize};

/// Shell-level events shared across browser tabs when `BroadcastChannel` is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShellSyncEvent {
    /// Theme or accessibility preferences changed.
    ThemeChanged,
    /// Wallpaper selection or wallpaper metadata changed.
    WallpaperChanged,
    /// Desktop layout state was durably persisted and other contexts should rehydrate.
    LayoutChanged,
}

#[cfg(target_arch = "wasm32")]
impl ShellSyncEvent {
    fn as_message(&self) -> &'static str {
        match self {
            Self::ThemeChanged => "theme-changed",
            Self::WallpaperChanged => "wallpaper-changed",
            Self::LayoutChanged => "layout-changed",
        }
    }
}

/// Returns whether the current browser exposes the `BroadcastChannel` API.
pub fn broadcast_channel_supported() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| {
                js_sys::Reflect::has(window.as_ref(), &"BroadcastChannel".into()).ok()
            })
            .unwrap_or(false)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Publishes a shell-sync event to other same-origin browser contexts.
pub fn publish_shell_sync_event(event: ShellSyncEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        if !broadcast_channel_supported() {
            return;
        }

        if let Ok(channel) = web_sys::BroadcastChannel::new("origin-os-shell-sync") {
            let _ = channel.post_message(&wasm_bindgen::JsValue::from_str(event.as_message()));
            channel.close();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event;
    }
}
