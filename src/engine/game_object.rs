use std::any::Any;
use std::cell::RefCell;
use std::path::Components;
use std::rc::Rc;
use crate::{ClearCanvas, Component};
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

    // pub fn test<'a>(c: Rc<Box<dyn Any>>) -> &'a ClearCanvas {
    //     return c.downcast_ref::<ClearCanvas>().unwrap();
    // }
}

impl GameObject {
    pub fn add_component(&mut self, component: Box<dyn Any>, app: &App) {
        // component.downcast_ref::<Component>();
        // component.on_add_to_game_object(app);
        // self.components.borrow_mut().push(Rc::new(RefCell::new(component)));
        // self.components.push(Box::new(Rc::new(Box::new(ClearCanvas::new()))));
        self.components.push(component);
    }

    pub fn get_component(&mut self) {

        // let c = self.components.borrow_mut().remove(0);
        // let c = c.borrow_mut().downcast::<ClearCanvas>();

        // return Rc::clone(&self.components[0]);
        let c = &self.components[0];
        let a = c.downcast_ref::<Rc<Box<ClearCanvas>>>().unwrap();

        // let mut found_component: Option<Rc<RefCell<Box<dyn Component>>>> = None;
        // for (i, component) in self.components.borrow().iter().enumerate() {
        //     let is_match = component.borrow_mut().as_any().downcast_ref::<T>().is_some();
        //     if is_match {
        //         console_log!("found");
        //         found_component = Some(Rc::clone(component));
        //         break;
        //     }
        // }
        //
        // console_log!("end");
        // return found_component;
        // // return match found_component {
        // //     Some(component) => Some(ComponentRef::new(component)),
        // //     None => None,
        // // };
    }

    // pub fn components_iter(&self) -> IterMut<Box<dyn Component>> {
    //     return self.components.borrow_mut().iter_mut();
    // }

    // pub fn components(&self) -> &RefCell<Vec<Rc<RefCell<Box<dyn Component>>>>> {
    //     return &self.components;
    // }
}
