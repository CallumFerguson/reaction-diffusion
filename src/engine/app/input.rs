pub struct Input {
    mouse_position: (i32, i32),
    buttons: u16,
}

pub enum Button {
    Left,
    Middle,
    Right,
}

impl Input {
    pub fn new() -> Self {
        return Self {
            mouse_position: (0, 0),
            buttons: 0,
        }
    }
}

impl Input {
     pub fn set_buttons(&mut self, buttons: u16) {
         self.buttons = buttons;
     }

    pub fn set_mouse_position(&mut self, mouse_position: (i32, i32)) {
        self.mouse_position = mouse_position;
    }

    pub fn get_button(&self, button: Button) -> bool {
        return match button {
            Button::Left => self.buttons & (1u16 << 0) > 0,
            Button::Middle => self.buttons & (1u16 << 2) > 0,
            Button::Right => self.buttons & (1u16 << 1) > 0,
        };
    }

    pub fn buttons(&self) -> u16 {
        return self.buttons;
    }

    pub fn mouse_position(&self) -> (i32, i32) {
        return self.mouse_position;
    }
}
