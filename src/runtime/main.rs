extern crate platformer;

use platformer::engine::Scene;
use platformer::bindings::Json;

fn read_directory(path: &str) {
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry.unwrap();
        println!("Script file read: {}", entry.path().display());
    }
}

fn main() {
    
    let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
    let mut scene = platformer::engine::Scene::new(&mut win);

    // Read config file
    let mut config = Json::new("game/config.json");
    config.load_json();
    let _config_scripts_path = &config.json["scripts_folders"];
    let config_data_path = &config.json["data_folder"].as_str();


    let mushroom_tex_path = format!("game/{}{}", config_data_path.unwrap(), "assets/sprites/mushroom.png");
    println!("Data directory: {}", mushroom_tex_path);
    let mut mushroom_tex = sfml::graphics::Texture::from_file(&mushroom_tex_path).unwrap();
    let mut mushroom_spr = sfml::graphics::Sprite::with_texture(&mut mushroom_tex);
    let mushroom_mdl = Scene::make_model("mushroom", &mut mushroom_spr);

    scene.add_model(mushroom_mdl);

    println!("Running Platformer Rust...");
    scene.render();
}