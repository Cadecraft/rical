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
use crate::types;

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

// TODO: use styles instead
pub fn render_date(
    day_of_month: i32,
    x: u16,
    y: u16,
    is_selected: bool,
    is_today: bool,
    events: &Vec<types::TaskDataWithId>
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let dateformat = if day_of_month > 9 {
        format!(" {} ", day_of_month.to_string())
    } else if day_of_month > 0 {
        format!(" 0{} ", day_of_month.to_string())
    } else {
        " -- ".to_string()
    };

    queue!(stdout,
        cursor::MoveTo(x, y),
        style::PrintStyledContent(
            if day_of_month <= 0 {
                dateformat.dark_grey()
            }
            else if is_selected && is_today {
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
    // TODO: refactor this
    queue!(stdout,
        // First 2 events
        cursor::MoveTo(x, y + 1),
        style::Print(" "),
        style::Print(if events.len() > 0 {
            "▆"
        } else { " " }),
        style::Print(if events.len() > 1 {
            "▆"
        } else { " " }),
        style::Print(" "),
        // Second 2 events
        cursor::MoveTo(x, y + 2),
        style::Print(" "),
        style::Print(if events.len() > 2 {
            "▆"
        } else { " " }),
        style::Print(if events.len() > 3 {
            "▆"
        } else { " " }),
        style::Print(" "),
        // End
        cursor::MoveTo(x, y + 3),
        style::Print("   ")
    )?;

    Ok(())
}

pub fn render(currstate: &state::CalendarState, api_handler: &mut ApiHandler) -> io::Result<()> {
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

    // Organize state
    let selected_date = utils::RicalDate::new(currstate.year, currstate.month, currstate.day);

    // Fetch data
    let calendar_tasks = api_handler.fetch_calendar_tasks_cached(selected_date.year, selected_date.month);

    // Main layout
    text::println(0, "[username]'s Calendar ([private])")?;
    text::println(1, "(Ctrl+M) main menu/log out | (Ctrl+S) settings | (Ctrl+C) quit")?;
    text::println(2, "")?;
    // Individual sections
    text::println(3, &format!(
        "{}/{}                         Tasks",
        selected_date.year,
        utils::fmt_twodigit(selected_date.month)
    ))?;
    text::println(4, "____________________________|________________")?;
    queue!(stdout,
        cursor::MoveTo(0, 5),
        style::Print(" Su  Mo  Tu  We  Th  Fr  Sa |")
    )?;
    // Calendar
    let calendar_frame = get_calendar_frame(selected_date.year, selected_date.month);
    let mut cursory = 6;
    for week in calendar_frame {
        queue!(stdout,
            cursor::MoveTo(0, cursory),
        )?;
        let mut cursorx = 0;
        for date in week {
            // Date itself
            // TODO: refactor
            // TODO: improve colors
            let today = utils::RicalDate::today();
            let is_today = today.year == selected_date.year && today.month == selected_date.month && today.day as i32 == date;
            let is_selected = date == selected_date.day as i32;
            let empty_events = vec![];
            let events = calendar_tasks.days.get((date - 1) as usize).unwrap_or(&empty_events);
            render_date(date, cursorx, cursory, is_selected, is_today, events)?;

            cursorx += 4;
        }
        queue!(stdout,
            cursor::MoveTo(cursorx, cursory),
            style::Print("|"),
            cursor::MoveTo(cursorx, cursory + 1),
            style::Print("|"),
            cursor::MoveTo(cursorx, cursory + 2),
            style::Print("|"),
            cursor::MoveTo(cursorx, cursory + 3),
            style::Print("|"),
        )?;
        cursory += 4;
    }
    let calendarframe_bottom_y = cursory;
    // Tasks menu
    // This should display tasks grouped by the current day and the days surrounding it
    const DAYS_DISPLAYED: u64 = 4;
    cursory = 5;
    let mut cursorx = 29;
    const TASKS_PANE_WIDTH: u16 = 30;
    for date_offset in 0..DAYS_DISPLAYED {
        let date = selected_date.add_days(date_offset);
        if date.month != currstate.month {
            // Displaying tasks from other months overcomplicates the logic
            break;
        }
        let date_title = format!(" {} - {} ", date.format(), date.weekday_name());
        let date_title_len = date_title.len();
        queue!(stdout,
            cursor::MoveTo(cursorx, cursory),
            style::PrintStyledContent(if date_offset == 0 { date_title.on_dark_grey() } else { date_title.dark_grey() }),
        )?;
        cursory += 3;
        text::pad_characters(TASKS_PANE_WIDTH, date_title_len as u16, " ")?;
        queue!(stdout, style::Print("|"))?;
        // TODO: get the tasks for this date and render them and their times (calendar_tasks.days.get)
    }

    // End
    queue!(stdout, cursor::MoveTo(0, calendarframe_bottom_y))?;
    text::cleartoend()?;

    // Specifically highlighted section

    Ok(())
}
