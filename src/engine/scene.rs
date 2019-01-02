extern crate sfml;

use std::boxed::Box;

use super::window::Window;
use super::super::containers::model::Model;

#[derive(Debug)]
pub struct Scene<'a> {
    m_time: sfml::system::Time,
    m_window: Box<Window<'a>>,
    m_models: Vec<Box<Model>>,
}
