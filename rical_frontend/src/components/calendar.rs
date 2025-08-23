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

// The main calendar screen

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('m')) {
        return state::ScreenState::Menu(state::MenuState::MainMenu);
    }

    state::ScreenState::Calendar(match &currstate.pane {
        // TODO: allow switching panes, etc.
        state::CalendarPane::Month => {
            // Month navigation

            let nav_res = if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('j')) {
                Some(utils::calendar_grid_navigation(currstate.year, currstate.month, currstate.day, utils::GridDirection::Down))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('k')) {
                Some(utils::calendar_grid_navigation(currstate.year, currstate.month, currstate.day, utils::GridDirection::Up))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('h')) {
                Some(utils::calendar_grid_navigation(currstate.year, currstate.month, currstate.day, utils::GridDirection::Left))
            } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Char('l')) {
                Some(utils::calendar_grid_navigation(currstate.year, currstate.month, currstate.day, utils::GridDirection::Right))
            } else {
                None
            };
            match nav_res {
                Some(res) => {
                    match res {
                        utils::DayCoordsResult::PrevMonth => {
                            state::CalendarState {
                                month: if currstate.month == 1 { 12 } else { currstate.month - 1 },
                                year: if currstate.month == 1 { currstate.year - 1 } else { currstate.year },
                                ..currstate.clone()
                            }
                        },
                        utils::DayCoordsResult::NextMonth => {
                            state::CalendarState {
                                month: if currstate.month == 12 { 1 } else { currstate.month + 1 },
                                year: if currstate.month == 12 { currstate.year + 1 } else { currstate.year },
                                ..currstate.clone()
                            }
                        },
                        utils::DayCoordsResult::SameMonth(newday) => {
                            state::CalendarState {
                                day: newday,
                                ..currstate.clone()
                            }
                        },
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
    // Individual sections
    queue!(stdout,
        cursor::MoveTo(0,0),
        // TODO: obtain info on the calendar: username, whether public/private, etc.
        style::Print("[username]'s Calendar ([private])"),
        cursor::MoveTo(0,1),
        style::Print("(Ctrl+M) main menu/log out | (Ctrl+S) settings | (Ctrl+C) quit"),
        cursor::MoveTo(0,3),
        // TODO: render a centered text component here, with the proper width
        style::Print("MONTH PLACEHOLDER TEXT"),
        cursor::MoveTo(0,4),
        style::Print("____________________________|________________"),
        cursor::MoveTo(0,5),
        style::Print(" Su  Mo  Tu  We  Th  Fr  Sa |"),
    )?;
    // Calendar
    // TODO: cache this?
    let calendar_frame = get_calendar_frame(currstate.year, currstate.month as u32);
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
            queue!(stdout,
                cursor::MoveTo(cursorx, cursory),
                if date == currstate.day {
                    // Selection
                    style::PrintStyledContent(dateformat.black().on_white())
                } else {
                    // TODO: highlight today's date as well
                    style::PrintStyledContent(dateformat.reset())
                }
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

    // Specifically highlighted section

    Ok(())
}
