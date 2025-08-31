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

use crate::types;

use crate::components::text;

// The main calendar screen

enum CalAction {
    Move(utils::GridDirection),
    SwitchToMonth,
    SwitchToTasks,
    None
}

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    if key_pressed(&key, KeyModifiers::CONTROL, KeyCode::Char('m')) {
        return state::ScreenState::Menu(state::MenuState::MainMenu);
    }

    let action = match &currstate.pane {
        // TODO: better-document how to switch panes via the ui (maybe a help menu or bottom bar)
        state::CalendarPane::Month => {
            if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('j')) {
                CalAction::Move(utils::GridDirection::Down)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('k')) {
                CalAction::Move(utils::GridDirection::Up)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('h')) {
                CalAction::Move(utils::GridDirection::Left)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('l')) {
                CalAction::Move(utils::GridDirection::Right)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Enter) {
                CalAction::SwitchToTasks
            } else {
                CalAction::None
            }
        },
        state::CalendarPane::Tasks => {
            if key_pressed(key, KeyModifiers::NONE, KeyCode::Esc) {
                CalAction::SwitchToMonth
            } else {
                CalAction::None
            }
        }
    };

    state::ScreenState::Calendar(match action {
        CalAction::Move(dir) => {
            let selected_date = utils::RicalDate::new(currstate.year, currstate.month, currstate.day);
            let res = utils::calendar_grid_navigation(&selected_date, dir);
            state::CalendarState {
                year: res.year,
                month: res.month,
                day: res.day,
                ..currstate.clone()
            }
        },
        CalAction::SwitchToTasks => {
            state::CalendarState {
                pane: state::CalendarPane::Tasks,
                ..currstate.clone()
            }
        },
        CalAction::SwitchToMonth => {
            state::CalendarState {
                pane: state::CalendarPane::Month,
                ..currstate.clone()
            }
        },
        CalAction::None => currstate.clone()
    })
}

// TODO: use styles instead
pub fn render_date(
    day_of_month: i32,
    x: u16,
    y: u16,
    is_selected: bool,
    is_today: bool,
    tasks: &Vec<types::TaskDataWithId>,
    pane: &state::CalendarPane
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
                match pane {
                    state::CalendarPane::Month => dateformat.dark_blue().on_white(),
                    state::CalendarPane::Tasks => dateformat.dark_blue().on_dark_grey(),
                }
            } else if is_selected {
                match pane {
                    state::CalendarPane::Month => dateformat.black().on_white(),
                    state::CalendarPane::Tasks => dateformat.black().on_dark_grey(),
                }
            } else if is_today {
                dateformat.black().on_blue()
            } else {
                dateformat.reset()
            }
        )
    )?;
    // TODO: print tasks beneath
    // TODO: refactor this
    queue!(stdout,
        // First 2 tasks
        cursor::MoveTo(x, y + 1),
        style::Print(" "),
        style::Print(if tasks.len() > 0 {
            "▆"
        } else { " " }),
        style::Print(if tasks.len() > 1 {
            "▆"
        } else { " " }),
        style::Print(" "),
        // Second 2 tasks
        cursor::MoveTo(x, y + 2),
        style::Print(" "),
        style::Print(if tasks.len() > 2 {
            "▆"
        } else { " " }),
        style::Print(if tasks.len() > 3 {
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
    text::println(1, "(Ctrl+M) menu/log out | (Ctrl+S) settings | (Ctrl+C) quit")?;
    text::println(2, "")?;
    const CALENDAR_WIDTH: u16 = 30;
    const TASKS_PANE_WIDTH: u16 = 46;
    const DATE_HEIGHT: u16 = 4;
    // Individual sections
    queue!(stdout, cursor::MoveTo(0, 3))?;
    text::padded_text(&format!(
        "  {}/{} - {}",
        selected_date.year,
        utils::fmt_twodigit(selected_date.month),
        utils::get_month_name(selected_date.month)
    ), CALENDAR_WIDTH, " ")?;
    text::padded_text("  Tasks", TASKS_PANE_WIDTH, " ")?;
    queue!(stdout, cursor::MoveTo(0, 4))?;
    text::println(4, "┌────────────────────────────┬──────────────────────────────────────────────┐")?;
    queue!(stdout,
        cursor::MoveTo(0, 5),
        style::Print("│"),
        style::Print(" Su  Mo  Tu  We  Th  Fr  Sa │")
    )?;
    // Calendar
    let calendar_frame = get_calendar_frame(selected_date.year, selected_date.month);
    let mut cursory = 6;
    for week in calendar_frame {
        queue!(stdout,
            cursor::MoveTo(0, cursory),
            style::Print("│"),
        )?;
        let mut cursorx = 1;
        for date in week {
            // Date itself
            // TODO: refactor
            // TODO: improve colors
            let today = utils::RicalDate::today();
            let is_today = today.year == selected_date.year && today.month == selected_date.month && today.day as i32 == date;
            let is_selected = date == selected_date.day as i32;
            let empty_tasks = vec![];
            let tasks = calendar_tasks.days.get((date - 1) as usize).unwrap_or(&empty_tasks);
            render_date(date, cursorx, cursory, is_selected, is_today, tasks, &currstate.pane)?;

            cursorx += DATE_HEIGHT;
        }
        for _i in 0..DATE_HEIGHT {
            queue!(stdout,
                cursor::MoveTo(0, cursory),
                style::Print("│"),
                cursor::MoveTo(cursorx, cursory),
                style::Print("│"),
            )?;
            cursory += 1;
        }
    }
    // TODO: have a constant-height calendar for consistency
    let calendarframe_bottom_y = cursory;
    // Tasks menu
    // This should display tasks grouped by the current day and the days surrounding it
    const DAYS_DISPLAYED: u64 = 7;
    cursory = 5;
    let cursorx = CALENDAR_WIDTH;
    for date_offset in 0..DAYS_DISPLAYED {
        let date = selected_date.add_days(date_offset);
        if date.month != currstate.month {
            // Displaying tasks from other months overcomplicates the logic
            break;
        }
        // Date title
        let date_title = format!(" {} - {} ", date.format(), date.weekday_name());
        let date_title_len = date_title.len();
        let is_selected = date_offset == 0;
        let is_today = date == utils::RicalDate::today();
        queue!(stdout,
            cursor::MoveTo(cursorx, cursory),
            style::PrintStyledContent(
                if is_selected && is_today {
                    match currstate.pane {
                        state::CalendarPane::Month => date_title.black().on_dark_blue(),
                        state::CalendarPane::Tasks => date_title.dark_blue().on_white(),
                    }
                } else if is_selected {
                    match currstate.pane {
                        state::CalendarPane::Month => date_title.black().on_dark_grey(),
                        state::CalendarPane::Tasks => date_title.black().on_white(),
                    }
                } else if is_today {
                    date_title.blue()
                } else {
                    date_title.dark_grey()
                }
            ),
        )?;
        text::pad_characters(TASKS_PANE_WIDTH, date_title_len as u16, " ")?;
        queue!(stdout, style::Print("│"))?;
        cursory += 1;
        // Tasks below the date
        let empty_tasks = vec![];
        let tasks = calendar_tasks.days.get((date.day - 1) as usize).unwrap_or(&empty_tasks);
        for task in tasks {
            queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
            // Time column
            const COL_TIME_WIDTH: u16 = 15;
            text::padded_text(&format!("   {}", utils::fmt_timerange(task.start_min, task.end_min)), COL_TIME_WIDTH, " ")?;
            // Checkbox column
            queue!(stdout, if task.complete {
                style::PrintStyledContent("[x] ".dark_grey())
            } else {
                style::PrintStyledContent("[ ] ".reset())
            })?;
            // Task text column
            // TODO: multiline
            // TODO: descriptions too?
            // TODO: what if user selects it
            text::padded_text(&task.title, TASKS_PANE_WIDTH - COL_TIME_WIDTH - 4, " ")?;
            queue!(stdout, style::Print("│"))?;
            cursory += 1;
        }
        queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
        text::pad_characters(TASKS_PANE_WIDTH, 0, " ")?;
        queue!(stdout, style::Print("│"))?;
        cursory += 1;
        // Divider between selected date and upcoming dates
        if date_offset == 0 {
            queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
            text::padded_text_styled("──── Upcoming ".dark_grey(), TASKS_PANE_WIDTH, "─".dark_grey())?;
            queue!(stdout, style::Print("│"))?;
            cursory += 1;
            queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
            text::pad_characters(TASKS_PANE_WIDTH, 0, " ")?;
            queue!(stdout, style::Print("│"))?;
            cursory += 1;
        }
    }
    // Clear anything below those dates
    for newy in cursory..calendarframe_bottom_y {
        // Clear out this line
        queue!(stdout, cursor::MoveTo(cursorx, newy))?;
        text::pad_characters(TASKS_PANE_WIDTH, 0, " ")?;
        queue!(stdout, style::Print("│"))?;
    }
    cursory = calendarframe_bottom_y;
    // Bottom frame closer
    queue!(stdout, cursor::MoveTo(0, cursory))?;
    text::println(cursory, "└────────────────────────────┴──────────────────────────────────────────────┘")?;
    cursory += 1;

    // End
    queue!(stdout, cursor::MoveTo(0, cursory))?;
    text::cleartoend()?;

    // Specifically highlighted section

    Ok(())
}
