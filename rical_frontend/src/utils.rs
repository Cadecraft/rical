use crossterm::{
    execute, queue,
    cursor, terminal,
    event::{KeyEvent, read, KeyCode, KeyModifiers},
    style::{self, Stylize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub struct KeyInfo {
    pub modifiers: KeyModifiers,
    pub code: KeyCode
}

// The formatted key event
pub fn read_key_event(event: KeyEvent) -> KeyInfo {
    KeyInfo {
        modifiers: event.modifiers, code: event.code
    }
}

pub enum RenderResult {
    QuitProgram,
    Nominal
}
