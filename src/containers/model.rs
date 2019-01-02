extern crate sfml;

use std::boxed::Box;
use std::fmt;

pub struct Model<'a> {
    pub name: String,
    pub model: Box<&'a mut sfml::graphics::Drawable>,
}

impl<'a> fmt::Debug for Model<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {0}\nModel: SFML Drawable", self.name)
    }
}

impl<'a> fmt::Display for Model<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {0}\nModel: SFML Drawable", self.name)
    }
}

impl<'a> Model<'a> {
    pub fn new_with(name: &str, model: &'a mut sfml::graphics::Drawable) -> Model<'a> {
        Model {
            name: name.to_string(),
            model: Box::new(model),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    pub fn set_model(&mut self, model: &'a mut sfml::graphics::Drawable) {
        self.model = Box::new(model)
    }
}
