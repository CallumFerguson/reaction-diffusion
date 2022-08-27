use std::cell::RefCell;
use std::rc::Rc;
use crate::Viewport;

pub struct EngineState {
    viewport: Rc<RefCell<Viewport>>
}

impl EngineState {
    pub fn new(viewport: Rc<RefCell<Viewport>>) -> Self {
        return Self {
            viewport
        }
    }
}

impl EngineState {
    pub fn viewport(&self) -> Rc<RefCell<Viewport>> {
        return Rc::clone(&self.viewport);
    }
}
