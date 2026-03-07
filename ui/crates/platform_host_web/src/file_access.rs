//! Browser file-access capability helpers.

/// Returns whether the File System Access directory picker is available.
pub fn directory_picker_supported() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| {
                js_sys::Reflect::has(window.as_ref(), &"showDirectoryPicker".into()).ok()
            })
            .unwrap_or(false)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}
