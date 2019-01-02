extern crate sfml;
extern crate engine;

pub use ::sfml::graphics::RenderTarget;

fn main() {
    let mut win = engine::Window::new_with(1024, 768, "Platformer Rust");
    while win.is_open() {
        win.poll_events();

        win.get_handle().clear(&sfml::graphics::Color::BLACK);
        win.get_handle().display();
    }
    println!("Running Platformer Rust...");
}
