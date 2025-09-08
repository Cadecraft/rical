use std::io;

use crate::state;
use crate::utils::{KeyInfo};
use crate::api::ApiHandler;

use crate::components::inputtext;
use crate::components::form;
use crate::styles;

// The login screen

pub fn handle_input(currstate: &state::FormState<2>, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let res = form::handle_input(currstate, key);
    match res.1 {
        form::FormResult::InProgress => {
            state::ScreenState::Menu(state::MenuState::Login(res.0))
        },
        form::FormResult::CancelAll => {
            state::ScreenState::Menu(state::MenuState::MainMenu)
        },
        form::FormResult::Submit => {
            // TODO: show loading screen
            let username = res.0.fields[0].contents.clone();
            let password = res.0.fields[1].contents.clone();
            match api_handler.try_login(username, password) {
                Ok(token) => {
                    state::ScreenState::Calendar(state::CalendarState::new())
                }, _ => {
                    // TODO: better error message
                    state::ScreenState::Menu(state::MenuState::Login(state::FormState {
                        error_message: Some(vec![
                            "Login failed. Make sure your username and password are correct.".to_string(),
                            "If you don't have an account, sign up first!".to_string()
                        ]),
                        ..res.0
                    }))
                }
            }
        }
    }
}

pub fn render(currstate: &state::FormState<2>) -> io::Result<()> {
    let render_params = form::FormRenderParameters {
        title: "Login".to_string(),
        hint_y: 7,
        fields: [
            form::FormFieldParameters {
                name: "Username".to_string(),
                styles: styles::Styles {
                    margin_left: 0,
                    margin_top: 4,
                    width: Some(38),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Password".to_string(),
                styles: styles::Styles {
                    margin_left: 0,
                    margin_top: 5,
                    width: Some(38),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Password
            }
        ],
        decoration_strings: vec![],
        clear_lines: vec![6]
    };
    form::render(currstate, render_params)?;
    Ok(())
}
