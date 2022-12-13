use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
};

pub fn create() -> (EventLoop<()>, Window) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    (event_loop, window)
}