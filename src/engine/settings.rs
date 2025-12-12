use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::sync::{OnceLock, RwLock};

#[derive(Debug)]
pub struct EngineSettings {
    settings: Value,
}

impl EngineSettings {
    /// Access the global singleton instance of `EngineSettings`.
    ///
    /// Initializes the `RwLock` and the `EngineSettings` struct lazily on first access.
    pub fn global() -> &'static RwLock<EngineSettings> {
        static INSTANCE: OnceLock<RwLock<EngineSettings>> = OnceLock::new();

        INSTANCE.get_or_init(|| RwLock::new(EngineSettings { settings: Value::Null }))
    }

    /// Obtains a read lock on the global settings.
    ///
    /// # Returns
    /// A read guard allowing concurrent read access.
    ///
    /// # Panics
    /// Handles lock poisoning by logging a warning and forcing access to the data.
    pub fn global_read() -> std::sync::RwLockReadGuard<'static, EngineSettings> {
        match EngineSettings::global().read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log::warn!("Warning: EngineSettings lock was poisoned. Recovering...");
                poisoned.into_inner()
            }
        }
    }

    /// Obtains a write lock on the global settings.
    ///
    /// # Returns
    /// A write guard allowing exclusive write access.
    ///
    /// # Panics
    /// Handles lock poisoning by logging a warning and forcing access to the data.
    pub fn global_write() -> std::sync::RwLockWriteGuard<'static, EngineSettings> {
        match EngineSettings::global().write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log::warn!("Warning: EngineSettings lock was poisoned. Recovering...");
                poisoned.into_inner()
            }
        }
    }

    /// Loads settings from a JSON file path and updates the current instance.
    ///
    /// If the file cannot be opened or the JSON is invalid, the settings
    /// will be set to `Value::Null` (or kept as is, depending on preference,
    /// currently resets to Null on error based on your logic).
    ///
    /// # Arguments
    /// * `path` - The string path to the JSON file.
    pub fn load(&mut self, path: &str) {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => log::error!("File not found: {error}"),
                    _ => log::error!("Error opening file: {error}"),
                }
                self.settings = Value::Null;
                return;
            }
        };

        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(value) => {
                self.settings = value;
            }
            Err(error) => {
                log::error!("Failed to parse JSON: {error}");
                self.settings = Value::Null;
            }
        };
    }

    /// Retrieves a value by its key.
    ///
    /// # Arguments
    /// * `key` - The top-level JSON key to look for.
    /// * `default` - The value to return if the key is missing or type mismatch occurs.
    #[allow(dead_code)]
    pub fn get<T>(&self, key: &str, default: T) -> T
    where
        T: DeserializeOwned,
    {
        match self.settings.get(key) {
            Some(value) => {
                // Attempt to deserialize; if it fails (wrong type), return default
                serde_json::from_value(value.clone()).unwrap_or(default)
            }
            None => {
                log::warn!("Key Not found: {}, using fallback.", key);
                default
            }
        }
    }

    /// Retrieves a nested value using a dot-notation path string.
    ///
    /// Example path: `"server.database.port"`
    ///
    /// # Arguments
    /// * `path` - The dot-separated path to the value.
    /// * `default` - The value to return if the path doesn't exist.
    #[allow(dead_code)]
    pub fn get_path<T>(&self, path: &str, default: T) -> T
    where
        T: DeserializeOwned,
    {
        // 'mut' here only allows the pointer 'current_value' to be reassigned
        // to different nodes. It does NOT allow mutating the data itself.
        let mut current_value: &Value = &self.settings;

        for key in path.split('.') {
            match current_value.get(key) {
                Some(v) => current_value = v,
                None => {
                    log::warn!("Path Key Not found: {}, using fallback.", key);
                    return default;
                }
            }
        }

        serde_json::from_value(current_value.clone()).unwrap_or(default)
    }
}
