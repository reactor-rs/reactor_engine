use glfw::{Key, MouseButton, Scancode, Action, Modifiers, Window};

use lang::{RasterFloat, TimeSec};


#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct KeyEvent(pub Key, pub Scancode, pub Action, pub Modifiers);

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct MouseButtonEvent(pub MouseButton, pub Action, pub Modifiers);

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct MouseEvent {
    pub x_pos: RasterFloat,
    pub y_pos: RasterFloat,
    pub x_offset: RasterFloat,
    pub y_offset: RasterFloat,
    pub is_scroll: bool,
    pub button_event: Option<MouseButtonEvent>
}

pub trait InputEvent {
    fn mouse_event(&mut self, event: MouseEvent);
    fn keyboard_event(&mut self, event: KeyEvent);
}

pub trait InputControl {
    fn on_mouse(&mut self, mouse: MouseEvent, delta_time: TimeSec);
    fn on_keyboard(&mut self, key: KeyEvent, delta_time: TimeSec);
    fn on_input(&mut self, window: &Window, delta_time: TimeSec);
}