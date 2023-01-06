use jazzy::prelude::*;

fn main() {
    jazzy::render::create_window(
        &WindowDescriptor::new()
        .title("Own")
        .size(1280, 720));
}