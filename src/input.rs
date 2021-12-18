use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputState {
    None,
    Pressed,
    Released
}

impl Default for InputState {
    fn default() -> Self {
        Self::None
    }
}

impl Into<InputState> for ElementState {
    fn into(self) -> InputState {
        match self {
            ElementState::Pressed => InputState::Pressed,
            ElementState::Released => InputState::Released,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardButton {
    pub key_code: Option<VirtualKeyCode>
}

impl KeyboardButton {
    pub fn is_pressed(&self, key_code: VirtualKeyCode, state: &InputState) -> bool {
        if state != &InputState::Pressed {
            return false;
        } 

        if let Some(code) = self.key_code {
            return code == key_code
        }

        false
    }
}

impl Into<KeyboardButton> for Option<VirtualKeyCode> {
    fn into(self) -> KeyboardButton {
        KeyboardButton { key_code: self }
    }
}