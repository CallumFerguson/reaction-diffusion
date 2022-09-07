use std::cell::RefCell;
use std::slice::IterMut;
use crate::Component;
use crate::engine::app::App;

pub struct GameObject {
    components: RefCell<Vec<Box<dyn Component>>>,
}

impl GameObject {
    pub fn new() -> Self {
        return Self {
            components: RefCell::new(Vec::new())
        };
    }
}

impl GameObject {
    pub fn add_component(&self, mut component: Box<dyn Component>, app: &App) {
        component.on_add_to_game_object(app);
        self.components.borrow_mut().push(component);
    }

    // pub fn components_iter(&self) -> IterMut<Box<dyn Component>> {
    //     return self.components.borrow_mut().iter_mut();
    // }

    pub fn components(&self) -> &RefCell<Vec<Box<dyn Component>>> {
        return &self.components;
    }
}
