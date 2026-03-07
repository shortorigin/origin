//! Browser PWA/runtime enhancement helpers.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Registers the service worker when the platform supports it.
pub fn register_service_worker() {
    #[cfg(target_arch = "wasm32")]
    {
        if !platform_host_web::pwa::service_worker_supported() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let navigator = window.navigator();
        let Ok(service_worker) = js_sys::Reflect::get(
            navigator.as_ref(),
            &wasm_bindgen::JsValue::from_str("serviceWorker"),
        ) else {
            return;
        };
        let Ok(register) = js_sys::Reflect::get(
            &service_worker,
            &wasm_bindgen::JsValue::from_str("register"),
        ) else {
            return;
        };
        let Some(register_fn) = register.dyn_ref::<js_sys::Function>() else {
            return;
        };
        let _ = register_fn.call1(&service_worker, &wasm_bindgen::JsValue::from_str("/sw.js"));
    }
}
