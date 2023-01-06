pub mod render;
pub mod window;

pub mod prelude {
    pub use crate::render::create_window;
    pub use crate::window::WindowDescriptor;
}

mod surface;