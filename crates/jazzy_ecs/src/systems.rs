pub struct EcsSystems {
    systems: Vec<fn()>,
    inits: Vec<fn()>,
}

impl Default for EcsSystems {
    fn default() -> Self {
        EcsSystems { 
            systems: Vec::new(),
            inits: Vec::new() 
        }
    }
}

impl EcsSystems {
    fn new() -> Self {
        EcsSystems::default()
    }

    fn add_init(mut self, init: fn()) -> Self {
        self.inits.push(init);

        self
    }
    
    fn add_system(mut self, system: fn()) -> Self {
        self.systems.push(system);

        self
    }

    fn init(self) {
        for system in self.inits {
            system();
        }
    }

    fn run(self) {
        for system in self.systems {
            system();
        }
    }
}