use std::io;

use crate::state;
use crate::utils::{KeyInfo, display_error};
use crate::api::ApiHandler;
use crate::styles;

use crate::components::inputtext;
use crate::components::form;

// The sign up screen

pub fn handle_input(currstate: &state::FormState<3>, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let res = form::handle_input(currstate, key, ["username", "password", "confirmpw"], None);
    match res.1 {
        form::FormResult::InProgress => {
            state::ScreenState::Menu(state::MenuState::Signup(res.0))
        },
        form::FormResult::CancelAll => {
            state::ScreenState::Menu(state::MenuState::MainMenu)
        },
        form::FormResult::Submit(result) => {
            // TODO: show loading screen
            let username = result["username"].clone();
            let password = result["password"].clone();
            let confirm_password = result["confirmpw"].clone();
            if confirm_password != password {
                return state::ScreenState::Menu(state::MenuState::Signup(state::FormState::from_result_message(vec![
                    "Your passwords don't match! Be careful when typing".to_string(),
                ])))
            }
            match api_handler.try_signup(username.clone(), password) {
                Ok(_) => {
                    state::ScreenState::Menu(state::MenuState::Signup(state::FormState::from_result_message(vec![
                        format!("You've successfully signed up as {}!", username),
                        "Go back to the menu (esc) and log in to access your calendar".to_string()
                    ])))
                },
                Err(err) => {
                    state::ScreenState::Menu(state::MenuState::Signup(state::FormState::from_result_message(vec![
                        "Signing up failed:".to_string(),
                        format!("  - {}", display_error(err)),
                        "Make sure you entered a unique username, and that you're connected to the server".to_string(),
                        "as specified in 'rical_frontend/.env'".to_string(),
                    ])))
                }
            }
        }
    }
}

pub fn render(currstate: &state::FormState<3>) -> io::Result<()> {
    let render_params = form::FormRenderParameters {
        title: "Sign up".to_string(),
        hint_y: 8,
        fields: [
            form::FormFieldParameters {
                name: "New username".to_string(),
                styles: styles::Styles {
                    margin_left: 0,
                    margin_top: 4,
                    width: Some(38),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "New password".to_string(),
                styles: styles::Styles {
                    margin_left: 0,
                    margin_top: 5,
                    width: Some(38),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Password
            },
            form::FormFieldParameters {
                name: "^ Confirm pw".to_string(),
                styles: styles::Styles {
                    margin_left: 0,
                    margin_top: 6,
                    width: Some(38),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Password
            }
        ],
        decoration_strings: vec![],
        clear_lines: vec![7]
    };
    form::render(currstate, render_params)?;
    Ok(())
}
