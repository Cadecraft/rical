use std::io::{self};
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};

fn handle_input_mainmenu(key: &KeyInfo) -> state::MenuState {
    if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
        return state::MenuState::Login(
            state::LoginState::EnteringInfo {
                form_pos: 0,
                username: String::new(),
                password: String::new()
            }
        );
    }
    state::MenuState::MainMenu
}

fn render_mainmenu() -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout,
        cursor::MoveTo(0,0),
        style::PrintStyledContent("Main Menu".cyan()),
        cursor::MoveTo(0,1),
        style::PrintStyledContent("(l) Login".cyan()),
        cursor::MoveTo(0,2),
        style::PrintStyledContent("(Ctrl+Q) Quit".cyan()),
    )?;
    Ok(())
}

pub fn handle_input(currstate: &state::MenuState, key: &KeyInfo) -> state::ScreenState {
    match &currstate {
        state::MenuState::MainMenu => {
            state::ScreenState::Menu(handle_input_mainmenu(key))
        },
        state::MenuState::Login(_) => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
                state::ScreenState::Menu(state::MenuState::MainMenu)
            } else {
                state::ScreenState::Menu(currstate.clone())
            }
        },
        state::MenuState::Signup(_) => {
            state::ScreenState::Menu(currstate.clone())
        }
    }
}

pub fn render(currstate: &state::MenuState) -> io::Result<()> {
    let mut stdout = io::stdout();

    match &currstate {
        state::MenuState::MainMenu => {
            render_mainmenu()?;
        },
        state::MenuState::Login(_) => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::PrintStyledContent("Login".cyan())
            )?;
        },
        state::MenuState::Signup(_) => {
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::PrintStyledContent("Signup".cyan())
            )?;
        },
    };

    Ok(())
}
