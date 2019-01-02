extern crate sfml;
extern crate platformer;

use sfml::graphics::RenderTarget;

fn main() {
    
    let mut win = platformer::engine::Window::new_with(1024, 768, "Platformer Rust");
    let scene = platformer::engine::Scene::new(&mut win);
    while win.is_open() {
        win.poll_events();

        win.clear(sfml::graphics::Color::BLACK);
        win.display();
    }
    println!("Running Platformer Rust...");
}
