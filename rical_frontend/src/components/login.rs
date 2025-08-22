use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};

// The login screen

pub fn handle_input(currstate: &state::LoginState, key: &KeyInfo) -> state::ScreenState {
    // Always allow esc to exit
    if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
        return state::ScreenState::Menu(state::MenuState::MainMenu);
    }

    match &currstate {
        state::LoginState::EnteringInfo { form_pos, username, password } => {
            // TODO: handle
            // TODO: refactor into an input component
            if key.modifiers == KeyModifiers::NONE {
                match key.code {
                    KeyCode::Char(c) => {
                        // TODO: depending on form pos
                        let mut new_username = username.clone();
                        new_username.push(c);
                        state::ScreenState::Menu(state::MenuState::Login(state::LoginState::EnteringInfo {
                            form_pos: *form_pos, username: new_username, password: password.clone()
                        }))
                    },
                    KeyCode::Backspace => {
                        // TODO: delete from password depending on form pos
                        let mut chars = username.chars();
                        chars.next_back();
                        state::ScreenState::Menu(state::MenuState::Login(state::LoginState::EnteringInfo {
                            form_pos: *form_pos, username: chars.as_str().to_string(), password: password.clone()
                        }))
                    },
                    _ => state::ScreenState::Menu(state::MenuState::Login(currstate.clone()))
                }
            } else {
                state::ScreenState::Menu(state::MenuState::Login(currstate.clone()))
            }
        },
        _ => state::ScreenState::Menu(state::MenuState::Login(currstate.clone()))
    }
}

pub fn render(currstate: &state::LoginState) -> io::Result<()> {
    let mut stdout = io::stdout();

    match &currstate {
        state::LoginState::Failed { error_message } => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("(esc) back"),
                cursor::MoveTo(0,2),
                // TODO: get and display version
                style::Print("Login failed. Make sure you have an account and that your username and password are correct."),
                cursor::MoveTo(0,4),
                style::Print(format!("Error message: {}", error_message)),
                cursor::MoveTo(0,0),
            )?;
        },
        state::LoginState::EnteringInfo { form_pos, username, password } => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("(esc) back"),
                cursor::MoveTo(0,2),
                // TODO: get and display version
                style::Print("Login"),
                cursor::MoveTo(0,4),
                style::Print("Username: "),
            )?;
            let mut count = 0;
            for c in username.chars() {
                queue!(stdout, style::Print(c))?;
                count += 1;
            }
            let cursor_pos = count;
            // TODO: refactor into an input component
            let max_input_width = 20;
            while count < max_input_width {
                queue!(stdout, style::Print('_'))?;
                count += 1;
            }
            // Return to the position
            queue!(stdout, cursor::MoveTo(cursor_pos, 4))?;
        }
    };

    Ok(())
}

