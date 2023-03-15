use serde::{Deserialize, Serialize};

use std::{fs, path::Path};

#[derive(Deserialize, Serialize)]
pub struct ConfigInstance {
    pub dark_theme: bool,
    pub fonts: Vec<String>,
}

impl ConfigInstance {
    pub fn get() -> Self {
        if let Option::Some(c) = Self::read_from_file(Self::get_path()) {
            c
        } else {
            Self::default()
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        if let Ok(s) = fs::read_to_string(path) {
            if let Ok(x) = serde_json::from_str::<Self>(&s) {
                return Option::Some(x);
            }
        };
        return Option::None;
    }

    pub fn write(&self) {
        crate::util::destroy(fs::write(
            Self::get_path(),
            serde_json::to_string(&self).unwrap(),
        ));
    }

    fn get_path() -> String {
        String::from("./.tuffous/config_gui.json")
    }
}

impl Default for ConfigInstance {
    fn default() -> Self {
        Self {
            dark_theme: false,
            fonts: Vec::new(),
        }
    }
}
