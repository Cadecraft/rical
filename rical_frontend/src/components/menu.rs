use std::io::{self, Stdout};
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::state::{self, RicalState};
use crate::utils::{KeyInfo, RenderResult, key_pressed};

pub fn render(currstate: &state::MenuState, key: Option<&KeyInfo>, stdout: &mut Stdout) -> io::Result<(RenderResult, state::ScreenState)> {
    // Render children, based on state
    let newstate = match &currstate {
        state::MenuState::MainMenu => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
                return Ok((RenderResult::Nominal, state::ScreenState::Menu(state::MenuState::Login(
                    state::LoginState::EnteringInfo {
                        form_pos: 0,
                        username: String::new(),
                        password: String::new()
                    }
                )
                )));
            }
            // TODO: render the calendar component
            queue!(stdout,
                cursor::MoveTo(0,0),
                style::PrintStyledContent("Main Menu".cyan()),
                cursor::MoveTo(0,1),
                style::PrintStyledContent("(l) Login".cyan()),
                cursor::MoveTo(0,2),
                style::PrintStyledContent("(Ctrl+Q) Quit".cyan()),
            )?;
            currstate.clone()
        },
        state::MenuState::Login(contents) => {
            if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
                return Ok((RenderResult::Nominal, state::ScreenState::Menu(state::MenuState::MainMenu)));
            }
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
