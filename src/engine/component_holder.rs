use std::cell::RefCell;
use std::rc::Rc;
use crate::Component;

pub struct ComponentHolder {
    component: Box<Rc<RefCell<dyn Component>>>,
    had_first_update: bool,
}

impl ComponentHolder {
    pub fn new(component: Box<Rc<RefCell<dyn Component>>>) -> Self {
        return Self {
            component,
            had_first_update: false,
        };
    }
}

 impl ComponentHolder {
     pub fn had_first_update(&self) -> bool { self.had_first_update }
     pub fn set_had_first_update(&mut self) { self.had_first_update  = true; }

     pub fn component(&self) -> &Box<Rc<RefCell<dyn Component>>> { &self.component }
 }
