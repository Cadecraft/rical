use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};

fn handle_input_mainmenu(key: &KeyInfo) -> state::MenuState {
    if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
        state::MenuState::Login(
            state::LoginState::EnteringInfo {
                form_pos: 0,
                username: String::new(),
                password: String::new()
            }
        )
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('s')) {
        state::MenuState::Signup(
            state::SignupState::EnteringInfo {
                form_pos: 0,
                username: String::new(),
                password: String::new()
            }
        )
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('a')) {
        state::MenuState::About
    } else {
        state::MenuState::MainMenu
    }
}

fn render_mainmenu() -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout,
        cursor::MoveTo(0,0),
        style::Print("Rical API"),
        cursor::MoveTo(0,1),
        style::Print("(l) Log in"),
        cursor::MoveTo(0,2),
        style::Print("(s) Sign up instantly"),
        cursor::MoveTo(0,4),
        style::Print("Rical Local (no syncing)"),
        cursor::MoveTo(0,5),
        style::Print("(Local database support coming soon!)"),
        cursor::MoveTo(0,7),
        style::Print("System"),
        cursor::MoveTo(0,8),
        style::Print("(a) About"),
        cursor::MoveTo(0,9),
        style::Print("(ctrl+q) Quit"),
        cursor::MoveTo(0,0),
    )?;
    Ok(())
}

pub fn handle_input(currstate: &state::MenuState, key: &KeyInfo) -> state::ScreenState {
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
        state::MenuState::Login(_) => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
                state::ScreenState::Menu(state::MenuState::MainMenu)
            } else {
                state::ScreenState::Menu(currstate.clone())
            }
        },
        state::MenuState::Signup(_) => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
                state::ScreenState::Menu(state::MenuState::MainMenu)
            } else {
                state::ScreenState::Menu(currstate.clone())
            }
        }
    }
}

pub fn render(currstate: &state::MenuState) -> io::Result<()> {
    let mut stdout = io::stdout();

    match &currstate {
        state::MenuState::MainMenu => {
            render_mainmenu()?;
        },
        state::MenuState::About => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("(esc) back"),
                cursor::MoveTo(0,2),
                // TODO: get and display version
                style::Print("Rical Frontend"),
                cursor::MoveTo(0,4),
                style::Print("By Cadecraft and any other Rical contributors (MIT license)"),
                cursor::MoveTo(0,0),
            )?;
        },
        state::MenuState::Login(_) => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("Login")
            )?;
        },
        state::MenuState::Signup(_) => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::Print("Signup")
            )?;
        },
    };

    Ok(())
}
