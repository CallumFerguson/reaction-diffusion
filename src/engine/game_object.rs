use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::IterMut;
use crate::Component;
use crate::engine::app::App;
use crate::engine::component_holder::ComponentHolder;

pub struct GameObject {
    components: Rc<RefCell<Vec<ComponentHolder>>>,
    components_as_any: Vec<Box<dyn Any>>,
    had_first_update: bool,
}

impl GameObject {
    pub fn new() -> Self {
        return Self {
            components: Rc::new(RefCell::new(Vec::new())),
            components_as_any: Vec::new(),
            had_first_update: false,
        };
    }
}

impl GameObject {
    pub fn add_component(&mut self, mut component: impl Component + 'static, app: &App) {
        component.on_add_to_game_object(self, &app);

        let component_rc_1 = Rc::new(RefCell::new(component));
        let component_rc_2 = Rc::clone(&component_rc_1);

        self.components_as_any.push(Box::new(component_rc_1));
        self.components.borrow_mut().push(ComponentHolder::new(Box::new(component_rc_2)));
    }

    pub fn get_component<T: 'static>(&self) -> Option<Rc<RefCell<T>>> {
        for component in self.components_as_any.iter() {
            if let Some(component) = component.downcast_ref::<Rc<RefCell<T>>>() {
                return Some(Rc::clone(component));
            }
        }
        return None;
    }

    // pub fn components(&self) -> &RefCell<Vec<Rc<RefCell<Box<dyn Component>>>>> {
    //     return &self.components;
    // }

    // pub fn components_iter_mut(&mut self) -> IterMut<Box<Rc<RefCell<dyn Component>>>> {
    //     return self.components.borrow_mut().iter_mut();
    // }

    pub fn components(&self) -> Rc<RefCell<Vec<ComponentHolder>>> {
        return Rc::clone(&self.components);
    }
}
