extern crate sfml;

use std::boxed::Box;

use super::window::Window;
use super::super::containers::model::Model;

pub struct Scene<'a, 'b> {
    elapsed_time: sfml::system::Time,
    window: &'b mut Window,
    models: Vec<Box<Model<'a>>>,
    clock: sfml::system::Clock,
    frame_time: f32
}

impl<'a, 'b> Drop for Scene<'a, 'b> { 
    fn drop(&mut self) {}
}

impl<'a, 'b> Scene<'a, 'b> {
    pub fn new(window: &'b mut Window) -> Scene<'a, 'b> {
        Scene {
            elapsed_time: sfml::system::Time::ZERO,
            window: window,
            models: Vec::new(),
            clock: sfml::system::Clock::default(),
            frame_time: 1.0 / 60.0
        }
    }

    pub fn render(&mut self) {
        let clock = sfml::system::Clock::start();
        while self.window.is_open() && clock.elapsed_time().as_seconds() < 10.0 {
            self.pre_frame();
            self.on_frame();
            self.post_frame();
            self.restart_clock();
        }
    }

    fn pre_frame(&mut self) {
        // Call draw function on all objects
        self.window.poll_events();
        self.window.clear(sfml::graphics::Color::BLACK);

        for model in &mut self.models {
            if model.is_visible() {
                self.window.draw(model);
            }
        }
    }

    fn on_frame(&mut self) {
        if self.elapsed_time.as_seconds() > self.frame_time {
            self.window.display();
            self.elapsed_time -= sfml::system::Time::seconds(self.frame_time)
        }
    }

    fn post_frame(&self) { }

    pub fn add_model(&mut self, model: Model<'a>) {
        self.models.push(Box::new(model))
    }

    pub fn add_model_ptr(&mut self, model: Box<Model<'a>>) {
        self.models.push(model)
    }

    pub fn get_model(&mut self, name: &str) -> Option<Box<Model<'a>>> {
        for model in &self.models {
            if model.get_name() == name {
                Some(model);
            }
        }

        println!("Could not find a model named \"{}\"", name);
        None
    }

    pub fn make_sprite(name: &str, sprite: sfml::graphics::Sprite<'a>) -> Model<'a> {
        Model::new_sprite(name, sprite)
    }

    fn get_elapsed_time(&self) -> sfml::system::Time {
        self.elapsed_time
    }

    fn restart_clock(&mut self) {
        self.elapsed_time += self.clock.restart()
    }
}
