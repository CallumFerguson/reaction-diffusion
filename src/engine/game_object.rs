use std::slice::IterMut;
use crate::Component;

pub struct GameObject {
    components: Vec<Box<dyn Component>>,
}

impl GameObject {
    pub fn new() -> Self {
        return Self {
            components: Vec::new()
        };
    }
}

impl GameObject {
    pub fn add_component(&mut self, mut component: Box<dyn Component>) {
        component.on_add_to_game_object();
        self.components.push(component);
    }

    pub fn components_iter(&mut self) -> IterMut<Box<dyn Component>> {
        return self.components.iter_mut();
    }
}
