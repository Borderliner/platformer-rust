extern crate platformer;

use std::ffi::OsStr;

use platformer::bindings::Json;
use platformer::bindings::Lua;

fn read_directory(path: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.path().extension() == Some(OsStr::new("lua")) {
            vec.push(entry.path().to_str().unwrap().to_string())
        }
    }

    vec
}

fn main() {
    {
        let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
        let mut scene = platformer::engine::Scene::new(&mut win);

        // Read config file
        let mut config = Json::new("game/config.json");
        config.load_json();
        let config_scripts_path = &config.json["scripts_folders"];
        let config_data_path = &config.json["data_folder"].as_str();

        let script_path_one = config_scripts_path[0].as_str().unwrap();

        println!("Scripts directory: {}", script_path_one);
        println!("Data directory: {}", config_data_path.unwrap());

        let lua_paths: Vec<String> = read_directory(&format!("game/{}", script_path_one));
        let mut lua = Lua::new();
        let lua_ptr = lua.get();

        for path in lua_paths {
            let content: String = Lua::read(&path);
            let function = lua_ptr.load(&content, Some(&path)).unwrap();
            function.call::<_, ()>(()).unwrap();
        }

        // Load backgrounds
        let mut backgrounds_tex: Vec<sfml::graphics::Texture> = Vec::new();
        let mut tex_ptr: *mut sfml::graphics::Texture;

        let mut model: platformer::containers::Model;

        for i in 0..50 {
            backgrounds_tex.push(sfml::graphics::Texture::from_file(&format!("game/{}{}{}.jpg", config_data_path.unwrap(), "assets/backgrounds/", (i + 1).to_string())).unwrap());
            tex_ptr = backgrounds_tex.as_mut_ptr();
            unsafe {
                model = platformer::containers::Model {
                    name: format!("{}{}", i, ".jpg"),
                    model: Box::new(sfml::graphics::Sprite::with_texture(&*tex_ptr.add(i))),
                    hidden: false
                };
            }

            scene.add_model(model);
        }

        println!("Running Platformer Rust for 10 seconds...");
        scene.render();

        println!("Cleaning resources...");
    }
    
    println!("Cleaned all the assets. Waiting for 30 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(30));
}