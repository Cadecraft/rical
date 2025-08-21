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

    let root = components::root::Root { };
    root.render(&mut state, None, &mut stdout)?;
    stdout.flush()?;

    while let Ok(event) = read() {
        // Each "frame" to render
        // Get input (only be concerned with key presses)
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        // TODO: use the keypress for input
        let keypress = utils::read_key_event(event);

        match root.render(&mut state, Some(&keypress), &mut stdout)? {
            utils::RenderResult::QuitProgram => {
                // Quit now
                // TODO: any cleanup?
                return Ok(())
            },
            _ => ()
        }

        stdout.flush()?;

    }
    
    Ok(())
}
