extern crate platformer;

fn main() {
    
    let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
    let mut scene = platformer::engine::Scene::new(&mut win);

    println!("Running Platformer Rust...");
    scene.render();
}
