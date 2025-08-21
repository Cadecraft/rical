use std::io::{self, Stdout};
use crossterm::{
    queue,
    cursor, terminal,
    event::{KeyEvent, read, KeyCode, KeyModifiers},
    style::{self, Stylize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::constants;
use crate::state::{self, RicalState};
use crate::utils::{KeyInfo, RenderResult};

/// The root component that renders all other components
pub struct Menu {
}

impl Menu {
    pub fn render(&self, currstate: &state::MenuState, keypress: Option<&KeyInfo>, stdout: &mut Stdout) -> io::Result<(RenderResult, state::ScreenState)> {
        // Handle keypress
        match keypress {
            Some(key) => {
                if key.code == KeyCode::Char('l') {
                    // Login
                    // TODO: impl
                    return Ok((RenderResult::Nominal, state::ScreenState::Menu(state::MenuState::Login(
                        state::LoginState::EnteringInfo {
                            form_pos: 0,
                            username: String::new(),
                            password: String::new()
                        }
                    )
                    )));
                }
            }, _ => ()
        };

        // Render children, based on state
        let newstate = match &currstate {
            state::MenuState::MainMenu => {
                // TODO: render the calendar component
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Main Menu".cyan())
                )?;
                currstate.clone()
            },
            state::MenuState::Login(contents) => {
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Login".cyan())
                )?;
                currstate.clone()
            },
            state::MenuState::Signup(contents) => {
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Signup".cyan())
                )?;
                currstate.clone()
            },
        };

        Ok((RenderResult::Nominal, state::ScreenState::Menu(newstate)))
    }
}
