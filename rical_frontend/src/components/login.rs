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
use crate::components::text;

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
                            state::ScreenState::Calendar(state::CalendarState::new())
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
    match &currstate {
        state::LoginState::Failed { error_message } => {
            text::println(0, "(esc) back")?;
            text::println(1, "")?;
            text::println(2, "Login failed. Make sure your username and password are correct.")?;
            text::println(3, "If you don't have an account, sign up first!")?;
            text::println(4, "")?;
            text::println(5, &format!("Error message: {}", error_message))?;
            text::clear_to_end()?;
        },
        state::LoginState::EnteringInfo { form_pos, username, password } => {
            text::println(0, "(esc) back")?;
            text::println(1, "")?;
            text::println(2, "Login")?;
            text::println(3, "")?;

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

            text::println(6, "")?;

            text::println(7, if *form_pos == 0 { "(enter) Next field" } else { "(enter) Submit" })?;
            text::clear_to_end()?;
        }
    };

    Ok(())
}
