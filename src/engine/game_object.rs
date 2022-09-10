use std::cell::RefCell;
use std::rc::Rc;
use crate::Component;
use crate::engine::app::App;

pub struct GameObject {
    components: RefCell<Vec<Rc<RefCell<Box<dyn Component>>>>>,
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
        self.components.borrow_mut().push(Rc::new(RefCell::new(component)));
    }

    pub fn get_component<T: 'static>(&self) -> Option<Rc<RefCell<Box<dyn Component>>>> {
        let mut found_component: Option<Rc<RefCell<Box<dyn Component>>>> = None;
        for (i, component) in self.components.borrow().iter().enumerate() {
            let is_match = component.borrow_mut().as_any().downcast_ref::<T>().is_some();
            if is_match {
                console_log!("found");
                found_component = Some(Rc::clone(component));
                break;
            }
        }

        console_log!("end");
        return found_component;
        // return match found_component {
        //     Some(component) => Some(ComponentRef::new(component)),
        //     None => None,
        // };
    }

    // pub fn components_iter(&self) -> IterMut<Box<dyn Component>> {
    //     return self.components.borrow_mut().iter_mut();
    // }

    pub fn components(&self) -> &RefCell<Vec<Rc<RefCell<Box<dyn Component>>>>> {
        return &self.components;
    }
}
