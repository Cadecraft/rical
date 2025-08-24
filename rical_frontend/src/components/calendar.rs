use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize},
};

use crate::state;
use crate::utils::{self, KeyInfo, key_pressed, get_calendar_frame};
use crate::api::ApiHandler;

use crate::components::inputtext;
use crate::styles;

use crate::components::text;

// The main calendar screen

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('m')) {
        return state::ScreenState::Menu(state::MenuState::MainMenu);
    }

    state::ScreenState::Calendar(match &currstate.pane {
        // TODO: allow switching panes, etc.
        state::CalendarPane::Month => {
            // Month navigation

            let rical_date = utils::RicalDate::new(currstate.year, currstate.month, currstate.day);

            let nav_res = if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('j')) {
                Some(utils::calendar_grid_navigation(&rical_date, utils::GridDirection::Down))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('k')) {
                Some(utils::calendar_grid_navigation(&rical_date, utils::GridDirection::Up))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('h')) {
                Some(utils::calendar_grid_navigation(&rical_date, utils::GridDirection::Left))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
                Some(utils::calendar_grid_navigation(&rical_date, utils::GridDirection::Right))
            } else {
                None
            };
            match nav_res {
                Some(res) => {
                    state::CalendarState {
                        year: res.year,
                        month: res.month,
                        day: res.day,
                        ..currstate.clone()
                    }
                },
                _ => currstate.clone()
            }
        },
        _ => currstate.clone()
    })
}

pub fn render(currstate: &state::CalendarState) -> io::Result<()> {
    let mut stdout = io::stdout();

    /*
        0         1         2         3         4
        01234567890123456789012345678901234567890123456789

        myusername's Calendar (private)
        (Ctrl+M) Logout/menu | (Ctrl+S) Settings | (Ctrl+C Quit)

                2025/05 - May           2025/05/05
        ____________________________|________________
         Su  Mo  Tu  We  Th  Fr  Sa |
         01  02  03  04  05  06  07 |
         xx  x=  ==  =   =x      =  |
         x   ==          =          |
         08  09  10  11  12  13  14 |
                                    |
                                    |
         15  16  17  18  19  20  21 |
                                    |
                                    |
         22  23  24  25  26  27  28 |
                                    |
                                    |
         29  30  31  01  02  03  04 |
                                    |
                                    |
    */

    // Main layout
    text::println(0, "[username]'s Calendar ([private])")?;
    text::println(1, "(Ctrl+M) main menu/log out | (Ctrl+S) settings | (Ctrl+C) quit")?;
    text::println(2, "")?;
    // Individual sections
    text::println(3, &format!("MONTH PLACEHOLDER: {}/{}", currstate.year, currstate.month))?;
    text::println(4, "____________________________|________________")?;
    text::println(5, " Su  Mo  Tu  We  Th  Fr  Sa |")?;
    // Calendar
    // TODO: cache this?
    let calendar_frame = get_calendar_frame(currstate.year, currstate.month);
    let mut cursory = 6;
    for week in calendar_frame {
        queue!(stdout,
            cursor::MoveTo(0, cursory),
        )?;
        let mut cursorx = 0;
        for date in week {
            // Date itself
            // TODO: improve colors
            let dateformat = if date > 9 {
                format!(" {} ", date.to_string())
            } else if date > 0 {
                format!(" 0{} ", date.to_string())
            } else {
                " -- ".to_string()
            };
            let curr_date = utils::RicalDate::today();
            let is_today = curr_date.year == currstate.year && curr_date.month == currstate.month && curr_date.day as i32 == date;
            let is_selected = date == currstate.day as i32;
            queue!(stdout,
                cursor::MoveTo(cursorx, cursory),
                style::PrintStyledContent(
                    if is_selected && is_today {
                        dateformat.dark_blue().on_white()
                    } else if is_selected {
                        dateformat.black().on_white()
                    } else if is_today {
                        dateformat.black().on_blue()
                    } else {
                        dateformat.reset()
                    }
                )
            )?;
            // TODO: print events beneath
            queue!(stdout,
                cursor::MoveTo(cursorx, cursory + 1),
                cursor::MoveTo(cursorx, cursory + 1),
            )?;

            cursorx += 4;
        }
        queue!(stdout,
            cursor::MoveTo(cursorx, cursory),
            style::Print("|"),
            cursor::MoveTo(cursorx, cursory + 1),
            style::Print("|"),
            cursor::MoveTo(cursorx, cursory + 2),
            style::Print("|"),
        )?;
        cursory += 3;
    }

    // End
    text::cleartoend()?;

    // Specifically highlighted section

    Ok(())
}
