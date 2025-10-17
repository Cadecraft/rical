use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
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

/// Given the current state and the user's keypress, returns the new state
/// Handles semi-complex text input methods like moving the cursor
/// Keybinds for submitting, switching between inputs, etc. should be handled by the caller
pub fn handle_input(currstate: &state::TextInputState, key: &KeyInfo) -> state::TextInputState {
    let contents = currstate.contents.clone();
    let mut chars: Vec<char> = contents.chars().collect();
    let cursor_pos = currstate.cursor_pos;

    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Char(c) => {
                chars.insert(cursor_pos, c);
                state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos + 1 }
            },
            KeyCode::Backspace => {
                if cursor_pos == 0 {
                    return currstate.clone();
                }
                chars.remove(cursor_pos - 1);
                state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos - 1 }
            },
            KeyCode::Delete => {
                if cursor_pos < chars.len() {
                    chars.remove(cursor_pos);
                }
                state::TextInputState { contents: chars.into_iter().collect(), cursor_pos }
            },
            KeyCode::Left => {
                let new_cursor_pos = if cursor_pos == 0 { 0 } else { cursor_pos - 1 };
                state::TextInputState { cursor_pos: new_cursor_pos, ..currstate.clone() }
            },
            KeyCode::Right => {
                let new_cursor_pos = if cursor_pos == contents.chars().count() { cursor_pos } else { cursor_pos + 1 };
                state::TextInputState { cursor_pos: new_cursor_pos, ..currstate.clone() }
            },
            KeyCode::End => {
                state::TextInputState { cursor_pos: contents.chars().count(), ..currstate.clone() }
            },
            KeyCode::Home => {
                state::TextInputState { cursor_pos: 0, ..currstate.clone() }
            },
            _ => currstate.clone()
        },
        KeyModifiers::SHIFT => match key.code {
            KeyCode::Char(c) => {
                let capitalized: String = c.to_uppercase().collect();
                // TODO: better capitalization logic?
                chars.insert(cursor_pos, capitalized.chars().nth(0).unwrap_or(' '));
                state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos + 1 }
            },
            KeyCode::Backspace => {
                // Duplicated for the sake of simplicity
                // TODO: reduce duplicate code?
                if cursor_pos == 0 {
                    return currstate.clone();
                }
                chars.remove(cursor_pos - 1);
                state::TextInputState { contents: chars.into_iter().collect(), cursor_pos: cursor_pos - 1 }
            },
            // TODO: allow "shift+tab" to go backwards
            _ => currstate.clone()
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Backspace | KeyCode::Char('w') => {
                // Delete the last word
                // Ex. 'abcd efg |' -> 'abcd |'
                if cursor_pos == 0 {
                    return currstate.clone();
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
                state::TextInputState { contents: res_string, cursor_pos: new_cursor_pos }
            },
            _ => currstate.clone()
        }
        _ => currstate.clone()
    }
}

pub fn render(label: &str, currstate: &state::TextInputState, styles: &Styles, mode: &InputMode) -> io::Result<()> {
    let mut stdout = io::stdout();

    if styles.wrap_text {
        return render_multiline(label, currstate, styles, mode);
    }

    let gap = styles.gap.unwrap_or(1);
    let label_width = label.chars().count() as u16 + 1 + gap;
    let total_width = styles.width.unwrap_or(30);

    queue!(stdout,
        cursor::MoveTo(styles.margin_left, styles.margin_top),
        style::Print(label),
        style::Print(":"),
        style::Print((0..gap).map(|_| " ").collect::<String>()),
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

fn render_multiline(label: &str, currstate: &state::TextInputState, styles: &Styles, mode: &InputMode) -> io::Result<()> {
    let mut stdout = io::stdout();

    let gap = styles.gap.unwrap_or(1);
    let label_width = label.chars().count() as u16 + 1 + gap;
    let total_width = styles.width.unwrap_or(30);
    let region_width = total_width - label_width - 1;
    let region_height = styles.height.unwrap_or(2);
    let y_start = styles.margin_top;
    let x_start = styles.margin_left + label_width;

    // Label
    queue!(stdout,
        cursor::MoveTo(styles.margin_left, styles.margin_top),
        style::Print(label),
        style::Print(":"),
        style::Print((0..gap).map(|_| " ").collect::<String>()),
    )?;
    // Blank space below label
    for local_y in 1..(region_height) {
        queue!(stdout,
            cursor::MoveTo(styles.margin_left, styles.margin_top + local_y),
            style::Print((0..label_width).map(|_| " ").collect::<String>()),
        )?;
    }

    let mut char_stack = currstate.contents.chars().rev();
    for local_y in 0..(region_height) {
        for local_x in 0..(region_width) {
            let current_char_index = local_x + local_y * region_width;
            let char_to_render = match char_stack.next_back() {
                Some(c) => match mode {
                    InputMode::Normal => c,
                    InputMode::Password => '*'
                },
                None => '_'
            };
            let render_cursor = current_char_index == currstate.cursor_pos as u16
                && styles.active;
            queue!(
                stdout,
                cursor::MoveTo(x_start + local_x, y_start + local_y),
                if render_cursor {
                    style::PrintStyledContent(char_to_render.black().on_white())
                } else {
                    style::PrintStyledContent(char_to_render.reset().white())
                }
            )?;
        }
        match styles.last_in_row {
            Some(is_last) => if is_last {
                text::clear_rest_of_line()?;
            },
            _ => {
                text::clear_rest_of_line()?;
            }
        };
    }

    Ok(())
}
