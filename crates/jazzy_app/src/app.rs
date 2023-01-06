use jazzy_ecs::systems::EcsSystems;

struct App {
    systems: EcsSystems,
}

impl Default for App {
    fn default() -> Self {
        App { systems: EcsSystems::default() }
    }
}