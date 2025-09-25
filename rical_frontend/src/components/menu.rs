use std::io;
use crossterm::{
    event::{KeyCode, KeyModifiers},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};
use crate::api::ApiHandler;

use crate::components::{login, signup, text};

fn handle_input_mainmenu(key: &KeyInfo) -> state::MenuState {
    if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
        state::MenuState::Login(state::FormState::<2>::new())
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('s')) {
        state::MenuState::Signup(state::FormState::<3>::new())
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('a')) {
        state::MenuState::About
    } else {
        state::MenuState::MainMenu
    }
}

fn render_mainmenu() -> io::Result<()> {
    text::println(0, "Rical API")?;
    text::println(1, "(l) Log in")?;
    text::println(2, "(s) Sign up instantly")?;
    text::println(3, "")?;
    text::println(4, "Rical Local (no syncing)")?;
    text::println(5, "(Local database support coming soon!)")?;
    text::println(6, "")?;
    text::println(7, "System")?;
    text::println(8, "(a) About")?;
    text::println(9, "(^C) Quit")?;
    text::clear_to_end()?;

    Ok(())
}

fn render_about() -> io::Result<()> {
    text::println(0, "(esc) back")?;
    text::println(1, "")?;
    let version = env!("CARGO_PKG_VERSION");
    text::println(2, &format!("Rical Frontend v{}", version))?;
    text::println(3, "")?;
    text::println(4, "By Cadecraft and any other Rical contributors (MIT license)")?;
    text::clear_to_end()?;

    Ok(())
}

pub fn handle_input(currstate: &state::MenuState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    match &currstate {
        state::MenuState::MainMenu => {
            state::ScreenState::Menu(handle_input_mainmenu(key))
        },
        state::MenuState::About => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
                state::ScreenState::Menu(state::MenuState::MainMenu)
            } else {
                state::ScreenState::Menu(currstate.clone())
            }
        }
        state::MenuState::Login(login_state) => {
            login::handle_input(login_state, key, api_handler)
        },
        state::MenuState::Signup(signup_state) => {
            signup::handle_input(signup_state, key, api_handler)
        }
    }
}

pub fn render(currstate: &state::MenuState) -> io::Result<()> {
    match &currstate {
        state::MenuState::MainMenu => {
            render_mainmenu()?;
        },
        state::MenuState::About => {
            render_about()?;
        },
        state::MenuState::Login(login_state) => {
            login::render(login_state)?;
        },
        state::MenuState::Signup(signup_state) => {
            signup::render(signup_state)?;
        },
    };

    Ok(())
}
