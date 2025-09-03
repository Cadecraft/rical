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
use crate::state;

use crate::components::text;

// A text input box

pub enum InputMode {
    Normal,
    Password
}

/// Given the current state (value/contents and other associated input state) and the user's keypress,
/// returns the new value and whether to submit.
/// Handles semi-complex text input methods like moving the cursor
pub fn handle_input(currstate: &state::TextInputState, key: &KeyInfo) -> (state::TextInputState, bool) {
    let contents = currstate.contents.clone();
    let mut chars: Vec<char> = contents.chars().collect();
    let cursor_pos = currstate.cursor_pos;

    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Char(c) => {
                chars.insert(cursor_pos, c);
                (state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos + 1 }, false)
            },
            KeyCode::Backspace => {
                if cursor_pos == 0 {
                    return (currstate.clone(), false);
                }
                chars.remove(cursor_pos - 1);
                (state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos - 1 }, false)
            },
            KeyCode::Delete => {
                if cursor_pos < chars.len() {
                    chars.remove(cursor_pos);
                }
                (state::TextInputState { contents: chars.into_iter().collect(), cursor_pos }, false)
            },
            KeyCode::Enter => {
                // Submit
                (currstate.clone(), true)
            },
            KeyCode::Left => {
                let new_cursor_pos = if cursor_pos == 0 { 0 } else { cursor_pos - 1 };
                (state::TextInputState { cursor_pos: new_cursor_pos, ..currstate.clone() }, false)
            },
            KeyCode::Right => {
                let new_cursor_pos = if cursor_pos == contents.chars().count() { cursor_pos } else { cursor_pos + 1 };
                (state::TextInputState { cursor_pos: new_cursor_pos, ..currstate.clone() }, false)
            },
            KeyCode::End => {
                (state::TextInputState { cursor_pos: contents.chars().count(), ..currstate.clone() }, false)
            },
            KeyCode::Home => {
                (state::TextInputState { cursor_pos: 0, ..currstate.clone() }, false)
            },
            // TODO: allow "tab" to go forwards but not submit
            _ => (currstate.clone(), false)
        },
        KeyModifiers::SHIFT => match key.code {
            KeyCode::Char(c) => {
                let capitalized: String = c.to_uppercase().collect();
                // TODO: better capitalization logic?
                chars.insert(cursor_pos, capitalized.chars().nth(0).unwrap_or(' '));
                (state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos + 1 }, false)
            },
            KeyCode::Backspace => {
                // Duplicated for the sake of simplicity
                // TODO: reduce duplicate code?
                if cursor_pos == 0 {
                    return (currstate.clone(), false);
                }
                chars.remove(cursor_pos - 1);
                (state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos - 1 }, false)
            },
            // TODO: allow "shift+tab" to go backwards
            _ => (currstate.clone(), false)
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Backspace | KeyCode::Char('w') => {
                // Delete the last word
                // Ex. 'abcd efg |' -> 'abcd |'
                if cursor_pos == 0 {
                    return (currstate.clone(), false);
                }
                // Delete everything before the cursor until we've both discovered a non-space character AND a space character
                let mut reversed: Vec<char> = Vec::new();
                let mut seen_nonspace = false;
                let mut finished_deleting = false;
                for (i, c) in chars.iter().enumerate().rev() {
                    if finished_deleting {
                        reversed.push(*c);
                        continue;
                    }

                    let seen_cursor = i <= cursor_pos;
                    if seen_cursor {
                        if *c == ' ' {
                            if seen_nonspace {
                                finished_deleting = true;
                                reversed.push(*c);
                            }
                        } else {
                            seen_nonspace = true;
                        }
                    } else {
                        reversed.push(*c);
                    }
                }
                let res_string: String = reversed.into_iter().rev().collect();
                let new_cursor_pos = cursor_pos.saturating_sub(contents.len() - res_string.chars().count());
                (state::TextInputState { contents: res_string, cursor_pos: new_cursor_pos }, false)
            },
            _ => (currstate.clone(), false)
        }
        _ => (currstate.clone(), false)
    }
}

pub fn render(label: &str, currstate: &state::TextInputState, styles: Styles, mode: InputMode) -> io::Result<()> {
    let mut stdout = io::stdout();

    let label_width = (label.chars().count() + 2) as u16;
    let total_width = styles.width.unwrap_or(30);

    queue!(stdout,
        cursor::MoveTo(styles.margin_left, styles.margin_top),
        style::Print(format!("{}: ", label))
    )?;
    let mut count = label_width;
    for c in currstate.contents.chars() {
        queue!(stdout, style::Print(match mode {
            InputMode::Normal => c,
            InputMode::Password => '*'
        }))?;
        count += 1;
    }
    text::pad_characters(total_width, count, "_")?;
    // Clear to the end of the line unless explicitly told not to
    match styles.last_in_row {
        Some(is_last) => if is_last {
            text::clear_rest_of_line()?;
        },
        _ => {
            text::clear_rest_of_line()?;
        }
    };
    // Render the "cursor" if the user is actively typing
    if styles.active {
        let char_under_cursor = currstate.contents.chars().nth(currstate.cursor_pos).unwrap_or('_').to_string();
        queue!(stdout,
            cursor::MoveTo(styles.margin_left + label_width + currstate.cursor_pos as u16, styles.margin_top),
            style::PrintStyledContent(char_under_cursor.black().on_white())
        )?;
    }

    Ok(())
}
