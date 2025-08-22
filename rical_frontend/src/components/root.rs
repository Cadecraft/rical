use std::io;
use crossterm::{
    queue,
    cursor,
    terminal,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize},
};

use crate::state::{self, RicalState};
use crate::utils::{KeyInfo, key_pressed};
use crate::api::ApiHandler;

use crate::components::menu;

// The root component that renders all other components

/// Handle a keypress and return the new state
pub fn handle_input(currstate: &RicalState, key: &KeyInfo, api_handler: &mut ApiHandler) -> RicalState {
    // Handle GLOBAL inputs (as this is the top level component)
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('q')) || key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('c')) {
        // Quit
        return state::RicalState {
            screen_state: state::ScreenState::ShouldQuit
        };
    }

    // Because we didn't capture any input yet,
    // the children are responsible for updating the state
    match &currstate.screen_state {
        state::ScreenState::Calendar { month, day, year } => {
            // TODO: handle via the calendar component
            currstate.clone()
        },
        state::ScreenState::Menu(contents) => {
            state::RicalState {
                screen_state: menu::handle_input(contents, key, api_handler)
            }
        },
        _ => currstate.clone()
    }
}

/// Render the screen based on the current state
pub fn render(currstate: &RicalState) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear performantly without flickering
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
    )?;

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
            menu::render(contents)?;
        },
        _ => ()
    };

    Ok(())
}
