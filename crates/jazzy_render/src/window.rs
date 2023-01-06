use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
};

pub struct WindowDescriptor {
    title: String,
    width: u32,
    height: u32,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        WindowDescriptor {
            title: "Jazzy window".to_string(),
            width: 800,
            height: 600,
        }
    }
}

impl WindowDescriptor {
     pub fn new() -> Self {
        WindowDescriptor::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;

        self
    }

    pub fn build(&self) -> (Window, EventLoop<()>) {
        let event_loop = EventLoop::new();

        (WindowBuilder::new()
        .with_title(&self.title)
        .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
        .build(&event_loop).unwrap(),
        event_loop)
    }
}