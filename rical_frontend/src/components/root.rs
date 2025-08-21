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

use crate::components::menu;

/// The root component that renders all other components
pub struct Root {
}

impl Root {
    pub fn render(&self, currstate: &RicalState, keypress: Option<&KeyInfo>, stdout: &mut Stdout) -> io::Result<(RenderResult, RicalState)> {

        // Background color
        for y in 0..constants::WINDOW_HEIGHT {
            for x in 0..constants::WINDOW_WIDTH {
                queue!(
                    stdout,
                    cursor::MoveTo(x,y),
                    style::PrintStyledContent(" ".black())
                )?;
            }
        }

        // Handle GLOBAL inputs (as this is the top level component)
        match keypress {
            Some(key) => {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                    // Quit
                    return Ok((RenderResult::QuitProgram, state::RicalState {
                        screen_state: state::ScreenState::ShouldQuit
                    }));
                }
            }, _ => ()
        };

        // Render children, based on state
        // Because we didn't capture any input yet,
        // the children are responsible for updating the state
        let newstate = match &currstate.screen_state {
            state::ScreenState::Calendar { month, day, year } => {
                // TODO: render the calendar component
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Calendar".cyan())
                )?;
                currstate.clone()
            },
            state::ScreenState::Menu(contents) => {
                let m = menu::Menu { };
                let res = state::RicalState {
                    screen_state: m.render(contents, keypress, stdout)?.1
                };
                res
            },
            _ => currstate.clone()
        };

        Ok((RenderResult::Nominal, newstate))
    }
}
