use std::fs::File;
use std::io::prelude::*;

pub struct Json {
    file_path: String,
    file_handle: File,
    content: String,
    pub json: serde_json::Value,
}

impl Json {
    pub fn new(path: &str) -> Json {
        Json {
            file_path: path.to_string(),
            file_handle: File::open(path).expect("Json file not found"),
            content: String::new(),
            json: serde_json::Value::default()
        }
    }

    pub fn load_json(&mut self) {
        self.file_handle.read_to_string(&mut self.content).expect("Could not read json file");
        self.json = serde_json::from_str(&self.content).unwrap();
    }
}
