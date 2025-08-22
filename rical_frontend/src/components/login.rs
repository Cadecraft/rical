use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};
use crate::api::ApiHandler;

use crate::components::inputtext;
use crate::styles;

// The login screen

pub fn handle_input(currstate: &state::LoginState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    // Always allow esc to exit
    if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
        return state::ScreenState::Menu(state::MenuState::MainMenu);
    }

    match &currstate {
        state::LoginState::EnteringInfo { form_pos, username, password } => {
            if *form_pos == 0 {
                // Entering username
                let (new_username, should_submit) = inputtext::handle_input(username, key);
                let new_form_pos = if should_submit { *form_pos + 1 } else { *form_pos };
                state::ScreenState::Menu(state::MenuState::Login(state::LoginState::EnteringInfo {
                    form_pos: new_form_pos, username: new_username, password: password.clone()
                }))
            } else {
                // Entering password
                let (new_password, should_submit) = inputtext::handle_input(password, key);
                if should_submit {
                    // Try to submit!
                    // TODO: show loading screen
                    match api_handler.try_login(username.clone(), new_password) {
                        Ok(token) => {
                            state::ScreenState::Calendar {
                                month: 1, year: 2025, day: 1
                            }
                        }, _ => {
                            // TODO: better error message
                            state::ScreenState::Menu(state::MenuState::Login(state::LoginState::Failed {
                                error_message: "Could not log in".to_string()
                            }))
                        }
                    }
                } else {
                    state::ScreenState::Menu(state::MenuState::Login(state::LoginState::EnteringInfo {
                        form_pos: *form_pos, username: username.clone(), password: new_password
                    }))
                }
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
                style::Print("Login failed. Make sure your username and password are correct."),
                cursor::MoveTo(0,3),
                style::Print("If you don't have an account, sign up first!"),
                cursor::MoveTo(0,5),
                style::Print(format!("Error message: {}", error_message)),
            )?;
        },
        state::LoginState::EnteringInfo { form_pos, username, password } => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("(esc) back"),
                cursor::MoveTo(0,2),
                style::Print("Login"),
            )?;

            inputtext::render("Username", username, styles::Styles {
                margin_left: 0,
                margin_top: 4,
                width: Some(38),
                active: *form_pos == 0,
                ..styles::Styles::new()
            }, inputtext::InputMode::Normal)?;

            inputtext::render("Password", password, styles::Styles {
                margin_left: 0,
                margin_top: 5,
                width: Some(38),
                active: *form_pos == 1,
                ..styles::Styles::new()
            }, inputtext::InputMode::Password)?;

            queue!(stdout,
                cursor::MoveTo(0, 7),
                style::Print(if *form_pos == 0 { "(enter) Next field" } else { "(enter) Submit" })
            )?;
        }
    };

    Ok(())
}
