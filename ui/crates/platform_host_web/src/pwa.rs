//! Progressive enhancement helpers for installable browser builds.

/// Returns whether service workers are supported.
pub fn service_worker_supported() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else {
            return false;
        };
        js_sys::Reflect::has(window.navigator().as_ref(), &"serviceWorker".into()).unwrap_or(false)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}
