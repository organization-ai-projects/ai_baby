use serde::{Deserialize, Serialize};
use std::fs;

pub fn load<T: for<'de> Deserialize<'de>>(path: &str) -> Option<T> {
    let s = fs::read_to_string(path).ok()?;
    serde_json::from_str(&s).ok()
}

pub fn save<T: Serialize>(data: &T, path: &str) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(path, json);
    }
}
