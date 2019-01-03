use std::fs::File;
use std::io::prelude::*;

pub struct Lua {
    pub lua: rlua::Lua,
}

impl Lua {
    pub fn new() -> Lua {
        Lua {
            lua: rlua::Lua::new(),
        }
    }

    pub fn get(&mut self) -> &rlua::Lua {
        &self.lua
    }

    pub fn read(path: &str) -> String {
        let mut file_handle = File::open(path).expect(&format!("Cannot read lua file \"{}\"", path));
        let mut file_contents = String::new();
        file_handle.read_to_string(&mut file_contents).expect(&format!("Could not convert lua script \"{}\" to string.", path));
        file_contents
    }
}
