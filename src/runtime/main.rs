extern crate sfml;
extern crate platformer;

use sfml::graphics::RenderTarget;


fn main() {
    platformer::engine::window::Window
    let mut win = platformer::engine::window::Window::new_with(1024, 768, "Platformer Rust");
    while win.is_open() {
        win.poll_events();

        win.get_handle().clear(&sfml::graphics::Color::BLACK);
        win.get_handle().display();
    }
    println!("Running Platformer Rust...");
}
