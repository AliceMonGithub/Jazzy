use jazzy::prelude::*;

fn main() {
    jazzy::render::create_window(
        &WindowDescriptor::new()
        .title("Game")
        .size(1280, 720));
}