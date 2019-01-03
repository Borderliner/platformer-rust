extern crate platformer;

use std::ffi::OsStr;

use platformer::engine::Scene;
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
    
    let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
    let mut scene = platformer::engine::Scene::new(&mut win);

    // Read config file
    let mut config = Json::new("game/config.json");
    config.load_json();
    let config_scripts_path = &config.json["scripts_folders"];
    let config_data_path = &config.json["data_folder"].as_str();

    let script_path_one = config_scripts_path[0].as_str().unwrap();
    println!("Scripts directory: {}", script_path_one);
    let lua_paths: Vec<String> = read_directory(&format!("game/{}", script_path_one));
    let mut lua = Lua::new();
    let lua_ptr = lua.get();

    for path in lua_paths {
        let content: String = Lua::read(&path);
        let function = lua_ptr.load(&content, Some(&path)).unwrap();
        function.call::<_, ()>(()).unwrap();
    }

    let mushroom_tex_path = format!("game/{}{}", config_data_path.unwrap(), "assets/sprites/mushroom.png");
    println!("Data directory: {}", mushroom_tex_path);
    let mut mushroom_tex = sfml::graphics::Texture::from_file(&mushroom_tex_path).unwrap();
    let mut mushroom_spr = sfml::graphics::Sprite::with_texture(&mut mushroom_tex);
    let mushroom_mdl = Scene::make_model("mushroom", &mut mushroom_spr);

    scene.add_model(mushroom_mdl);

    println!("Running Platformer Rust...");
    scene.render();
}