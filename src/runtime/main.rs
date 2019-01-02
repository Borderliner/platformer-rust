extern crate platformer;

use platformer::engine::Scene;

fn main() {
    
    let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
    let mut scene = platformer::engine::Scene::new(&mut win);

    let mut mushroom_tex = sfml::graphics::Texture::from_file("data/assets/sprites/mushroom.png").unwrap();
    let mut mushroom_spr = sfml::graphics::Sprite::with_texture(&mut mushroom_tex);
    scene.add_model(Scene::make_model("mushroom", &mut mushroom_spr));
    
    println!("Running Platformer Rust...");
    scene.render();
}