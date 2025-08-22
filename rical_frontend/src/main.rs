use std::io::{self, stdout, Write};

use crossterm::{
    execute,
    terminal,
    event::read,
};

mod api;
mod state;
mod components;
mod constants;
mod utils;
mod styles;

fn main() -> io::Result<()> {
    // TODO: connect to the API
    api::placeholder();

    // State setup
    let mut state = state::RicalState {
        screen_state: state::ScreenState::Menu(
            state::MenuState::MainMenu
        )
    };

    // Initial rendering setup only
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    components::root::render(&state)?;
    stdout.flush()?;

    while let Ok(event) = read() {
        // Each "frame" to render
        // Get input (only be concerned with key presses)
        let Some(event) = event.as_key_press_event() else {
            continue;
        };
        let keypress = utils::read_key_event(event);

        // Handle input and update state
        let new_screen_state = components::root::handle_input(&state, &keypress).screen_state;
        match new_screen_state {
            state::ScreenState::ShouldQuit => {
                // Quit now
                // TODO: any cleanup?
                return Ok(())
            },
            _ => {
                state.screen_state = new_screen_state;
            }
        };
        // Render the results after the keypress
        components::root::render(&state)?;

        stdout.flush()?;
    }
    Ok(())
}
