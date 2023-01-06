use crate::{surface, window::WindowDescriptor};



pub fn create_window(window_descriptor: &WindowDescriptor) {
    surface::create_surface(window_descriptor);
}