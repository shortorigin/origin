//! Browser navigation helpers built on standard URL/location primitives.

/// Returns the current browser href when available.
pub fn current_href() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window()?;
        window.location().href().ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
