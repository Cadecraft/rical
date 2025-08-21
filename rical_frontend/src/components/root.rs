use std::io::{self, Stdout};
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize},
};

use crate::constants;
use crate::state::{self, RicalState};
use crate::utils::{KeyInfo, RenderResult, key_pressed};

use crate::components::menu;

/// The root component that renders all other components
pub fn render(currstate: &RicalState, key: Option<&KeyInfo>, stdout: &mut Stdout) -> io::Result<(RenderResult, RicalState)> {

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
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('q')) {
        // Quit
        return Ok((RenderResult::QuitProgram, state::RicalState {
            screen_state: state::ScreenState::ShouldQuit
        }));
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
            state::RicalState {
                screen_state: menu::render(contents, key, stdout)?.1
            }
        },
        _ => currstate.clone()
    };

    Ok((RenderResult::Nominal, newstate))
}
