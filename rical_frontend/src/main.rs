use std::io::{self, stdout, Write};

use crossterm::{
    execute,
    terminal,
    cursor,
    event::read,
};

use dotenvy;

mod api;
mod state;
mod components;
mod utils;
mod styles;
mod types;

fn main() -> io::Result<()> {
    // TODO: connect to the API/get auth token

    // Env setup
    dotenvy::dotenv().ok();

    // State and data setup
    // TODO: default to main page, not calendar
    let mut state = state::RicalState {
        screen_state: state::ScreenState::Menu(
            state::MenuState::MainMenu
        )
    };
    /*let mut state = state::RicalState {
        screen_state: state::ScreenState::Calendar(state::CalendarState {
            year: 2025,
            month: 8,
            day: 2,
            task_id: None,
            pane: state::CalendarPane::Month,
            making_new_task: None,
            editing_task: None,
        })
    };*/
    let mut api_handler = api::ApiHandler::new();

    // Initial rendering setup only
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    execute!(stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;
    components::root::render(&state, &mut api_handler)?;
    stdout.flush()?;

    while let Ok(event) = read() {
        // Each "frame" to render
        // Get input (only be concerned with key presses)
        let Some(event) = event.as_key_press_event() else {
            continue;
        };
        let keypress = utils::read_key_event(event);

        // Handle input and update state
        let new_screen_state = components::root::handle_input(&state, &keypress, &mut api_handler).screen_state;
        match new_screen_state {
            state::ScreenState::ShouldQuit => {
                // Quit now
                break;
            },
            _ => {
                state.screen_state = new_screen_state;
            }
        };
        // Render the results after the keypress
        components::root::render(&state, &mut api_handler)?;

        stdout.flush()?;
    }

    // Cleanup
    // TODO: any other cleanup?
    execute!(stdout,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
