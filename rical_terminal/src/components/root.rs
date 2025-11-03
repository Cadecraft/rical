use crossterm::event::{KeyCode, KeyModifiers};
use std::io;

use crate::api::ApiHandler;
use crate::state::{self, RicalState};
use crate::utils::{KeyInfo, key_pressed};

use crate::components::{calendar, menu};

// The root component that renders all other components

/// Handle a keypress and return the new state
pub fn handle_input(
    currstate: &RicalState,
    key: &KeyInfo,
    api_handler: &mut ApiHandler,
) -> RicalState {
    // Handle GLOBAL inputs (as this is the top level component)
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('c')) {
        return state::RicalState {
            screen_state: state::ScreenState::ShouldQuit,
        };
    }

    // Because we didn't capture any input yet,
    // the children are responsible for updating the state
    match &currstate.screen_state {
        state::ScreenState::Calendar(contents) => state::RicalState {
            screen_state: calendar::handle_input(contents, key, api_handler),
        },
        state::ScreenState::Menu(contents) => state::RicalState {
            screen_state: menu::handle_input(contents, key, api_handler),
        },
        _ => currstate.clone(),
    }
}

/// Render the screen based on the current state
pub fn render(currstate: &RicalState, api_handler: &mut ApiHandler) -> io::Result<()> {
    // Do not re-clear the screen every time, as components will write over any changed text
    // This will prevent flickering

    // Render children, based on state
    match &currstate.screen_state {
        state::ScreenState::Calendar(contents) => {
            calendar::render(contents, api_handler)?;
        }
        state::ScreenState::Menu(contents) => {
            menu::render(contents)?;
        }
        _ => (),
    };

    Ok(())
}
