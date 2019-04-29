use crate::types::System;

pub struct SystemContainer(Vec<Box<System>>);

impl SystemContainer {
    pub fn new() -> Self {
        SystemContainer(Vec::new())
    }

    pub fn register<S: 'static>(&mut self, system: S)
    where
        S: System,
    {
        self.0.push(Box::new(system) as Box<dyn System>);
    }

    pub fn run(&mut self) {
        for system in self.0.iter_mut() {
            system.run();
        }
    }
}
