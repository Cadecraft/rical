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
pub struct Root {
}

impl /*Renderable for*/ Root {
    pub fn render(&self, currstate: &mut RicalState, keypress: Option<&KeyInfo>, stdout: &mut Stdout) -> io::Result<RenderResult> {

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
                    return Ok(RenderResult::QuitProgram);
                }
            }, _ => ()
        };

        // Render children, based on state
        match &currstate.screen_state {
            state::ScreenState::Calendar { month, day, year } => {
                // TODO: render the calendar component
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Calendar".cyan())
                )?;
            },
            state::ScreenState::Menu(contents) => {
                queue!(stdout,
                    cursor::MoveTo(0,0),
                    style::PrintStyledContent("Menu".cyan())
                )?;

                // TODO: render a submenu based on the contents
            }
        };

        Ok(RenderResult::Nominal)
    }
}
