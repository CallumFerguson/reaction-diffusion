use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::Component;
use crate::engine::app::App;

pub struct GameObject {
    // components: RefCell<Vec<Rc<RefCell<Box<dyn Any>>>>>,
    components: Vec<Box<dyn Any>>,
}

impl GameObject {
    pub fn new() -> Self {
        return Self {
            // components: RefCell::new(Vec::new()),
            components: Vec::new(),
        };
    }
}

impl GameObject {
    pub fn add_component(&mut self, mut component: impl Component + 'static, app: &App) {
        component.on_add_to_game_object(&app);
        self.components.push(Box::new(Rc::new(RefCell::new(component))));
    }

    pub fn get_component<T: 'static>(&mut self) -> Option<Rc<RefCell<T>>> {
        for component in self.components.iter() {
            if let Some(component) = component.downcast_ref::<Rc<RefCell<T>>>() {
                return Some(Rc::clone(component));
            }
        }
        return None;
    }

    // pub fn components(&self) -> &RefCell<Vec<Rc<RefCell<Box<dyn Component>>>>> {
    //     return &self.components;
    // }
}
