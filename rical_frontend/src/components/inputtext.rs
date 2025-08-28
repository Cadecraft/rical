use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    terminal::{Clear, ClearType},
    style::{self, Stylize}
};

use crate::utils::KeyInfo;

use crate::styles::Styles;

use crate::components::text;

// A text input box

pub enum InputMode {
    Normal,
    Password
}

/// Given the current value and the user's keypress, returns the new value
/// and whether to submit
pub fn handle_input(value: &str, key: &KeyInfo) -> (String, bool) {
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Char(c) => {
                let mut res = value.to_string();
                res.push(c);
                (res, false)
            },
            KeyCode::Backspace => {
                let mut chars = value.chars();
                chars.next_back();
                (chars.as_str().to_string(), false)
            },
            KeyCode::Enter => {
                // Submit
                (value.to_string(), true)
            }
            _ => (value.to_string(), false)
        },
        KeyModifiers::SHIFT => match key.code {
            KeyCode::Char(c) => {
                let capitalized: String = c.to_uppercase().collect();
                let mut res = value.to_string();
                res.push_str(&capitalized);
                (res, false)
            },
            KeyCode::Backspace => {
                let mut chars = value.chars();
                chars.next_back();
                (chars.as_str().to_string(), false)
            },
            _ => (value.to_string(), false)
        },
        _ => (value.to_string(), false)
    }
}

pub fn render(label: &str, value: &str, styles: Styles, mode: InputMode) -> io::Result<()> {
    let mut stdout = io::stdout();

    let label_width = (label.chars().count() + 2) as u16;
    let total_width = styles.width.unwrap_or(30);

    queue!(stdout,
        cursor::MoveTo(styles.margin_left, styles.margin_top),
        style::Print(format!("{}: ", label))
    )?;
    let mut count = label_width;
    for c in value.chars() {
        queue!(stdout, style::Print(match mode {
            InputMode::Normal => c,
            InputMode::Password => '*'
        }))?;
        count += 1;
    }
    let cursor_pos = count;
    text::pad_characters(total_width, cursor_pos, "_")?;
    // Clear to the end of the line
    queue!(stdout,
        Clear(ClearType::UntilNewLine)
    )?;
    // Render the "cursor" if the user is actively typing
    if styles.active {
        queue!(stdout,
            cursor::MoveTo(styles.margin_left + cursor_pos, styles.margin_top),
            style::PrintStyledContent("_".black().on_white())
        )?;
    }

    Ok(())
}
