use std::cell::RefCell;
use std::rc::Rc;
use crate::Component;

pub struct ComponentHolder {
    component: Box<Rc<RefCell<dyn Component>>>,
    had_first_update: RefCell<bool>,
}

impl ComponentHolder {
    pub fn new(component: Box<Rc<RefCell<dyn Component>>>) -> Self {
        return Self {
            component,
            had_first_update: RefCell::new(false),
        };
    }
}

 impl ComponentHolder {
     pub fn had_first_update(&self) -> bool { *self.had_first_update.borrow() }
     pub fn set_had_first_update(&self) { *self.had_first_update.borrow_mut() = true; }

     pub fn component(&self) -> &Box<Rc<RefCell<dyn Component>>> { &self.component }
 }
