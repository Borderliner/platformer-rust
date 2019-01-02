extern crate sfml;

use std::boxed::Box;

use sfml::window::{Event, Key, Style, VideoMode};
use sfml::graphics::{RenderWindow, RenderTarget};

#[derive(Debug)]
pub struct Window {
    m_window_width: u32,
    m_window_height: u32,
    m_window_title: String,
    m_fullscreen: bool,
    m_fps: f32,
    m_initial_aspect_ratio: f32,
    m_event: sfml::window::Event,
    m_window_handle: Box<RenderWindow>
}

impl Window {
    pub fn new() -> Window {
        Window::new_with(1024, 768, "Main Window (Rust)")
    }

    pub fn new_with(width: u32, height: u32, title: &str) -> Window {
        let desktop_mode = VideoMode::desktop_mode();
        
        Window {
            m_window_width: width,
            m_window_height: height,
            m_window_title: title.to_string(),
            m_fullscreen: false,
            m_initial_aspect_ratio: (width as f32) / (height as f32),
            m_fps: 60.0,
            m_event: Event::GainedFocus,
            m_window_handle: Box::new(
                RenderWindow::new(
                    VideoMode::new(width as u32, height as u32, desktop_mode.bits_per_pixel),
                title, Style::CLOSE, &Default::default())
            )
        }
    }

    pub fn poll_events(&mut self) {
        while let Some(event) = self.m_window_handle.poll_event() {
            self.m_event = event;

            match self.m_event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
                    self.m_window_handle.close();
                },
                Event::KeyPressed { code: Key::F5, .. } => { }, // Toggle fullscreen,
                Event::Resized {width: new_width, height: new_height} => { }, // Resized event
                Event::LostFocus => { },
                Event::GainedFocus => { },
                _ => { }
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.m_window_handle.is_open()
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        let desktop_mode = VideoMode::desktop_mode();

        if !self.m_fullscreen && fullscreen {
            self.m_window_handle.close();
            self.m_window_handle = Box::new(
                RenderWindow::new(
                    VideoMode::new(desktop_mode.width, desktop_mode.height, desktop_mode.bits_per_pixel),
                &self.m_window_title, Style::FULLSCREEN, &Default::default())
            )
        } else if self.m_fullscreen && !fullscreen {
            self.m_window_handle.close();
            let new_width = (desktop_mode.height as f32) * self.m_initial_aspect_ratio;
            self.m_window_handle = Box::new(
                RenderWindow::new(
                    VideoMode::new(new_width.floor() as u32, desktop_mode.height, desktop_mode.bits_per_pixel),
                &self.m_window_title, Style::CLOSE, &Default::default())
            )
        } else {
            // Nothing
        }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.m_fps = fps;
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.m_window_width, self.m_window_height)
    }

    pub fn get_title(&self) -> &str {
        &self.m_window_title
    }

    pub fn clear(&mut self, color: sfml::graphics::Color) {
        self.m_window_handle.clear(&color);
    }

    pub fn display(&mut self) {
        self.m_window_handle.display();
    }
}
