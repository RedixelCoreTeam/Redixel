use serde::de::DeserializeOwned;
use serde_json::{Value};
use std::fs::File;
use std::io::BufReader;
// use std::sync::{OnceLock, RwLock};

#[derive(Debug)]
pub struct EngineSettings {
    settings: Value,
}

impl EngineSettings {
    pub fn load(path: &str) -> Self {
        let file: File = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => log::error!("File not found: {error}"),
                    _ => log::error!("Error opening file: {error}"),
                }
                
                return Self {
                    settings: serde_json::Value::Null
                };
            }
        };

        let reader: BufReader<File> = BufReader::new(file);
        
        let json: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(val) => val,
            Err(error) => {
                log::error!("Failed to parse JSON: {error}");
                
                return Self {
                    settings: serde_json::Value::Null
                };
            }
        };

        Self {
            settings: json
        }
    }

    #[allow(dead_code)]
    fn get<T>(&self, key: &str, default: T) -> T
    where
        T: DeserializeOwned
    {
        match self.settings.get(key) {
            Some(value) => {
                serde_json::from_value(value.clone()).unwrap_or(default)
            },
            None => default
        }
    }

    #[allow(dead_code)]
    fn get_path<T>(&self, path: &str, default: T) -> T    
    where
        T: DeserializeOwned
    {
        let mut current_value = &self.settings;

        for key in path.split('.') {
            match current_value.get(key) {
                Some(v) => current_value = v,
                None => return default,
            }
        }

        serde_json::from_value(current_value.clone()).unwrap_or(default)
    }
}