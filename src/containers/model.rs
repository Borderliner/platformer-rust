extern crate sfml;

use std::fmt;
use sfml::system::Vector2f;

pub struct Model<'a, 'b> {
    name: &'a str,
    model: &'b sfml::graphics::Drawable,
}

impl<'a, 'b> fmt::Debug for Model<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {0}\nModel: SFML Drawable", self.name)
    }
}

impl<'a, 'b> fmt::Display for Model<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {0}\nModel: SFML Drawable", self.name)
    }
}

impl<'a, 'b> Model<'a, 'b> {
    const empty_rec: sfml::graphics::RectangleShape<'static> = sfml::graphics::RectangleShape::with_size(Vector2f::new((0.0, 0.0)));
    const empty_model: Model<'static, 'static> = Model{name: "empty_model".to_string(), model: Model::empty_rec};

    fn new() -> &'static Model<'static, 'static> {
        &Model::empty_model
    }
    
    fn new_with(name: &'a str, model: &'b sfml::graphics::Drawable) -> Model<'a, 'b> {
        Model {
            name: name,
            model: model
        }
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name
    }

    fn get_model(&self) -> &'b sfml::graphics::Drawable {
        self.model
    }

    fn set_model(&mut self, model: &'b sfml::graphics::Drawable) {
        self.model = model
    }
}
