//! Browser preference storage implementation backed by IndexedDB via the shared bridge.

use platform_host::{PrefsStore, PrefsStoreFuture};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, Copy, Default)]
/// Browser preference store backed by IndexedDB through the shared bridge.
pub struct WebPrefsStore;

impl WebPrefsStore {
    /// Loads a raw JSON string for a preference key.
    pub fn load_json(self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            futures::executor::block_on(crate::bridge::load_pref(key))
                .ok()
                .flatten()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = key;
            None
        }
    }

    /// Saves a raw JSON string for a preference key.
    ///
    /// # Errors
    ///
    /// Returns an error when browser persistence is unavailable or the write fails.
    pub fn save_json(self, key: &str, raw_json: &str) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            futures::executor::block_on(crate::bridge::save_pref(key, raw_json))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (key, raw_json);
            Ok(())
        }
    }

    /// Deletes a preference key from IndexedDB-backed browser persistence.
    ///
    /// # Errors
    ///
    /// Returns an error when browser persistence is unavailable or the delete fails.
    pub fn delete_json(self, key: &str) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            futures::executor::block_on(crate::bridge::delete_pref(key))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = key;
            Ok(())
        }
    }

    /// Loads and deserializes a typed preference value.
    pub fn load_typed<T: DeserializeOwned>(self, key: &str) -> Option<T> {
        let raw = self.load_json(key)?;
        serde_json::from_str(&raw).ok()
    }

    /// Serializes and saves a typed preference value.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization or localStorage write fails.
    pub fn save_typed<T: Serialize>(self, key: &str, value: &T) -> Result<(), String> {
        let raw = serde_json::to_string(value).map_err(|e| e.to_string())?;
        self.save_json(key, &raw)
    }
}

impl PrefsStore for WebPrefsStore {
    fn load_pref<'a>(
        &'a self,
        key: &'a str,
    ) -> PrefsStoreFuture<'a, Result<Option<String>, String>> {
        let store = *self;
        Box::pin(async move { Ok(store.load_json(key)) })
    }

    fn save_pref<'a>(
        &'a self,
        key: &'a str,
        raw_json: &'a str,
    ) -> PrefsStoreFuture<'a, Result<(), String>> {
        let store = *self;
        Box::pin(async move { store.save_json(key, raw_json) })
    }

    fn delete_pref<'a>(&'a self, key: &'a str) -> PrefsStoreFuture<'a, Result<(), String>> {
        let store = *self;
        Box::pin(async move { store.delete_json(key) })
    }
}
