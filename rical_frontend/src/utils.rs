use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

pub struct KeyInfo {
    pub modifiers: KeyModifiers,
    pub code: KeyCode
}

pub fn read_key_event(event: KeyEvent) -> KeyInfo {
    KeyInfo {
        modifiers: event.modifiers, code: event.code
    }
}

pub fn key_pressed(key: &KeyInfo, modifiers: KeyModifiers, code: KeyCode) -> bool {
    key.code == code && key.modifiers == modifiers
}
