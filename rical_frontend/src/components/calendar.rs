use std::io;
use crossterm::{
    queue,
    cursor,
    terminal,
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize},
};

use crate::state;
use crate::utils::{self, KeyInfo, key_pressed, get_calendar_frame};
use crate::api::ApiHandler;

use crate::types;

use crate::components::text;
use crate::components::new_task_form;

// The main calendar screen

enum CalAction {
    Move(utils::GridDirection),
    SwitchToMonth,
    SwitchToTasks,
    SelectTaskUp,
    SelectTaskDown,
    StartNewTask,
    None
}

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    if currstate.making_new_task.is_some() {
        return new_task_form::handle_input(currstate, key, api_handler);
    }

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
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('o')) {
                CalAction::StartNewTask
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Enter) {
                CalAction::SwitchToTasks
            } else {
                CalAction::None
            }
        },
        state::CalendarPane::Tasks => {
            if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('j')) {
                CalAction::SelectTaskDown
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('k')) {
                CalAction::SelectTaskUp
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('b')) {
                // Back a day
                CalAction::Move(utils::GridDirection::Left)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('w')) {
                // Forward a day
                CalAction::Move(utils::GridDirection::Right)
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Char('o')) {
                CalAction::StartNewTask
            } else if key_pressed(key, KeyModifiers::NONE, KeyCode::Esc) {
                CalAction::SwitchToMonth
            } else {
                CalAction::None
            }
        }
    };

    let selected_date = utils::RicalDate::new(currstate.year, currstate.month, currstate.day);

    state::ScreenState::Calendar(match action {
        CalAction::Move(dir) => {
            let res = utils::calendar_grid_navigation(&selected_date, dir);
            state::CalendarState {
                year: res.year,
                month: res.month,
                day: res.day,
                task_id: None,
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
        CalAction::StartNewTask => {
            state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState::new(selected_date)),
                ..currstate.clone()
            }
        }
        CalAction::SelectTaskUp => {
            let date_tasks = api_handler.fetch_tasks_at_date_cached(&selected_date);

            // Select the task/day above the current one
            match currstate.task_id {
                Some(id) => {
                    let curr_task_index = date_tasks.iter().position(|task| task.task_id == id);
                    match curr_task_index {
                        Some(0) | None => {
                            // Reset to the parent date
                            state::CalendarState {
                                task_id: None,
                                ..currstate.clone()
                            }
                        },
                        Some(index) => {
                            state::CalendarState {
                                task_id: Some(date_tasks[index - 1].task_id),
                                ..currstate.clone()
                            }
                        }
                    }
                },
                None => {
                    // Previous date's last task
                    let res = selected_date.sub_days(1);
                    let date_tasks_prev = api_handler.fetch_tasks_at_date_cached(&res);
                    if date_tasks_prev.len() > 0 {
                        state::CalendarState {
                            year: res.year,
                            month: res.month,
                            day: res.day,
                            task_id: Some(date_tasks_prev[date_tasks_prev.len() - 1].task_id),
                            ..currstate.clone()
                        }
                    } else {
                        state::CalendarState {
                            year: res.year,
                            month: res.month,
                            day: res.day,
                            task_id: None,
                            ..currstate.clone()
                        }
                    }
                }
            }
        }
        CalAction::SelectTaskDown => {
            let date_tasks = api_handler.fetch_tasks_at_date_cached(&selected_date);

            // Select the task/day after the current one
            match currstate.task_id {
                Some(id) => {
                    // Next task or next date
                    let curr_task_index = date_tasks.iter().position(|task| task.task_id == id);
                    match curr_task_index {
                        Some(index) => {
                            if index == date_tasks.len() - 1 {
                                // Next date
                                let res = selected_date.add_days(1);
                                state::CalendarState {
                                    year: res.year,
                                    month: res.month,
                                    day: res.day,
                                    task_id: None,
                                    ..currstate.clone()
                                }
                            } else {
                                // Next task
                                state::CalendarState {
                                    task_id: Some(date_tasks[index + 1].task_id),
                                    ..currstate.clone()
                                }
                            }
                        },
                        None => {
                            // Should never happen; just reset
                            state::CalendarState {
                                task_id: None,
                                ..currstate.clone()
                            }
                        }
                    }
                },
                None => {
                    // First task or next date
                    if date_tasks.len() > 0 {
                        state::CalendarState {
                            task_id: Some(date_tasks[0].task_id),
                            ..currstate.clone()
                        }
                    } else {
                        // Next date
                        let res = selected_date.add_days(1);
                        state::CalendarState {
                            year: res.year,
                            month: res.month,
                            day: res.day,
                            task_id: None,
                            ..currstate.clone()
                        }
                    }
                }
            }
        }
        CalAction::None => currstate.clone()
    })
}

// Rendering constants
const CALENDAR_WIDTH: u16 = 30;
const CALENDAR_MARGIN_TOP: u16 = 2;
const TASKS_PANE_WIDTH_MIN: u16 = 24;
const TASKS_PANE_WIDTH_MAX: u16 = 90;
const DATE_SQUARE_WIDTH: u16 = 4;
/// The largest vertial height of the terminal where we need Mini Mode
/// (i.e. dates in the calendar are collapsed to take up less height)
const MINI_HEIGHT_BREAKPOINT: u16 = 29;

fn get_viewport_width() -> io::Result<u16> {
    let terminal_width = terminal::size()?.0;
    Ok(terminal_width - 1)
}

/// Whether the height is small enough that the dates must be collapsed vertically
fn is_mini_mode() -> io::Result<bool> {
    let terminal_height = terminal::size()?.1;
    Ok(terminal_height <= MINI_HEIGHT_BREAKPOINT)
}

/// A small, colorful representation of a task
pub fn render_task_candy(x: u16, y: u16, task: &types::TaskDataWithId, overdue: bool) -> io::Result<()> {
    let mut stdout = io::stdout();

    // TODO: in the future, determine whether overdue here? (issue only arises with demo fake api data)

    let task_char = if task.start_min.is_some() && task.end_min.is_none() {
        "▼"
    } else { match task.duration_mins() {
        Some(dur) => {
            if dur <= 15 { "▂" }
            else if dur <= 30 { "▃" }
            else if dur <= 45 { "▄" }
            else if dur <= 60 { "▅" }
            else if dur <= 120 { "▆" }
            else { "█" }
        }
        None => "•",
    } };

    queue!(stdout,
        cursor::MoveTo(x, y),
        style::PrintStyledContent(
            if task.complete {
                task_char.dark_green()
            } else if overdue {
                task_char.dark_yellow()
            } else {
                task_char.dark_blue()
            }
        )
    )?;
    Ok(())
}

// TODO: use styles instead
pub fn render_date_square(
    date: Option<utils::RicalDate>,
    x: u16,
    y: u16,
    is_selected: bool,
    tasks: &Vec<types::TaskDataWithId>,
    pane: &state::CalendarPane
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let dayformat = match &date {
        Some(d) => {
            if d.day > 9 {
                format!(" {} ", d.day.to_string())
            } else {
                format!(" 0{} ", d.day.to_string())
            }
        },
        None => " -- ".to_string()
    };

    let is_today = match &date { Some(d) => *d == utils::RicalDate::today(), None => false };
    let is_overdue = match &date { Some(d) => *d < utils::RicalDate::today(), None => false };

    let date_height: u16 = if is_mini_mode()? { 3 } else { 4 };

    queue!(stdout,
        cursor::MoveTo(x, y),
        style::PrintStyledContent(
            if date.is_none() {
                dayformat.dark_grey()
            }
            else if is_selected && is_today {
                match pane {
                    state::CalendarPane::Month => dayformat.black().on_dark_cyan(),
                    state::CalendarPane::Tasks => dayformat.cyan().on_dark_grey(),
                }
            } else if is_selected {
                match pane {
                    state::CalendarPane::Month => dayformat.black().on_white(),
                    state::CalendarPane::Tasks => dayformat.black().on_dark_grey(),
                }
            } else if is_today {
                dayformat.cyan()
            } else {
                dayformat.reset()
            }
        )
    )?;
    let max_tasks_displayed = (date_height - 1) * 2;
    for i in 0..max_tasks_displayed {
        let task_x = 1 + x + i % 2;
        let task_y = y + 1 + i / 2;
        match tasks.get(i as usize) {
            Some(task) => {
                render_task_candy(task_x, task_y, &task, is_overdue)?;
            },
            None => {
                queue!(stdout,
                    cursor::MoveTo(task_x, task_y),
                    style::Print(" ")
                )?;
            }
        }
    }
    // Clear the rest of the space beneath the date
    for clear_y in 0..(date_height - 1) {
        queue!(stdout, cursor::MoveTo(x, y + clear_y + 1), style::Print(" "))?;
        queue!(stdout, cursor::MoveTo(x + 3, y + clear_y + 1), style::Print(" "))?;
    }

    Ok(())
}

/// Render a date and its tasks in the tasks menu, returning its height in number of rows
pub fn render_tasks_date(
    date: utils::RicalDate,
    x: u16,
    y: u16,
    tasks_pane_width: u16,
    is_selected: bool,
    selected_task_id: Option<i64>,
    is_today: bool,
    tasks: &Vec<types::TaskDataWithId>,
    pane: &state::CalendarPane
) -> io::Result<u16> {
    let mut stdout = io::stdout();

    let mut cursory = y;
    // Date title
    let date_title = format!(" {} - {} ", date.format(), date.weekday_name());
    let date_title_len = date_title.len();
    let is_title_selected = is_selected && selected_task_id.is_none();
    queue!(stdout,
        cursor::MoveTo(x, cursory),
        style::PrintStyledContent(
            if is_title_selected && is_today {
                match pane {
                    state::CalendarPane::Month => date_title.cyan().on_dark_grey(),
                    state::CalendarPane::Tasks => date_title.black().on_dark_cyan(),
                }
            } else if is_title_selected {
                match pane {
                    state::CalendarPane::Month => date_title.black().on_dark_grey(),
                    state::CalendarPane::Tasks => date_title.black().on_white(),
                }
            } else if is_today {
                date_title.cyan()
            } else {
                date_title.dark_grey()
            }
        ),
    )?;
    text::pad_characters(tasks_pane_width, date_title_len as u16, " ")?;
    queue!(stdout, style::Print("│"))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
    cursory += 1;
    // Tasks below the date
    for task in tasks {
        queue!(stdout, cursor::MoveTo(x, cursory), style::Print(" "))?;
        // Candy column
        let overdue = date < utils::RicalDate::today();
        render_task_candy(x + 1, cursory, task, overdue)?;
        // Time column
        const COL_TIME_WIDTH: u16 = 13;
        let timerange_text = format!(" {}", utils::fmt_timerange(task.start_min, task.end_min));
        text::padded_text_styled(if task.complete { (&timerange_text as &str).dark_grey() } else { (&timerange_text as &str).reset() }, COL_TIME_WIDTH, " ".reset())?;
        // Checkbox column
        let is_task_selected = is_selected && match selected_task_id {
            Some(id) => task.task_id == id,
            None => false
        };
        queue!(stdout, if task.complete {
            let checkbox = "[x]";
            style::PrintStyledContent(if is_task_selected {
                match pane {
                    // TODO: dark grey or black?
                    state::CalendarPane::Month => checkbox.green().on_dark_grey(),
                    state::CalendarPane::Tasks => checkbox.black().on_dark_green(),
                }
            } else {
                checkbox.green()
            })
        } else {
            let checkbox = "[ ]";
            style::PrintStyledContent(if is_task_selected {
                match pane {
                    state::CalendarPane::Month => checkbox.black().on_dark_grey(),
                    state::CalendarPane::Tasks => checkbox.black().on_white(),
                }
            } else {
                checkbox.reset()
            })
        }, style::Print(" "))?;
        // Task text column
        // TODO: multiline
        // TODO: descriptions too?
        // TODO: what if user selects it
        text::padded_text_styled(if task.complete { (&task.title as &str).dark_grey() } else { (&task.title as &str).reset() }, tasks_pane_width - COL_TIME_WIDTH - 6, " ".reset())?;
        queue!(stdout, style::Print("│"))?;
        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
        cursory += 1;
    }
    queue!(stdout, cursor::MoveTo(x, cursory))?;
    text::pad_characters(tasks_pane_width, 0, " ")?;
    queue!(stdout, style::Print("│"))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
    cursory += 1;
    Ok(cursory)
}

pub fn render(currstate: &state::CalendarState, api_handler: &mut ApiHandler) -> io::Result<()> {
    if currstate.making_new_task.is_some() {
        return new_task_form::render(currstate);
    }

    let mut stdout = io::stdout();

    // Organize state
    let selected_date = utils::RicalDate::new(currstate.year, currstate.month, currstate.day);

    // Fetch data
    let calendar_tasks = api_handler.fetch_calendar_tasks_cached(selected_date.year, selected_date.month);

    // Responsive layout
    let viewport_width = get_viewport_width()?;
    let tasks_pane_width = std::cmp::min(TASKS_PANE_WIDTH_MAX, std::cmp::max(TASKS_PANE_WIDTH_MIN, viewport_width - CALENDAR_WIDTH - 1));
    let date_height: u16 = if is_mini_mode()? { 3 } else { 4 };

    // Main layout
    let top_right_str = "(^M) menu/log out | (^S) settings | (^C) quit";
    queue!(stdout, cursor::MoveTo(0, 0))?;
    text::padded_text("[username]'s Calendar ([private])", viewport_width - top_right_str.chars().count() as u16, " ")?;
    queue!(stdout, style::Print(top_right_str))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
    text::println(1, "")?;
    // Titles and top container edge
    queue!(stdout, cursor::MoveTo(0, CALENDAR_MARGIN_TOP), style::Print("┌"))?;
    queue!(stdout, style::PrintStyledContent(match currstate.pane {
        state::CalendarPane::Month => "─".blue(),
        state::CalendarPane::Tasks => "─".reset()
    }))?;
    let calendar_title = format!(" {}/{} - {} ", selected_date.year, utils::fmt_twodigit(selected_date.month), utils::get_month_name(selected_date.month));
    let calendar_title_str: &str = &calendar_title;
    text::padded_text_styled(match currstate.pane {
        state::CalendarPane::Month => calendar_title_str.blue(),
        state::CalendarPane::Tasks => calendar_title_str.dark_grey(),
    }, CALENDAR_WIDTH - 3, match currstate.pane {
        state::CalendarPane::Month => "─".blue(),
        state::CalendarPane::Tasks => "─".reset(),
    })?;
    queue!(stdout, style::Print("┬"))?;
    queue!(stdout, style::PrintStyledContent(match currstate.pane {
        state::CalendarPane::Month => "─".reset(),
        state::CalendarPane::Tasks => "─".blue()
    }))?;
    let tasks_title_str = " Tasks ";
    text::padded_text_styled(match currstate.pane {
        state::CalendarPane::Month => tasks_title_str.dark_grey(),
        state::CalendarPane::Tasks => tasks_title_str.blue(),
    }, tasks_pane_width - 1, match currstate.pane {
        state::CalendarPane::Month => "─".reset(),
        state::CalendarPane::Tasks => "─".blue(),
    })?;
    queue!(stdout, style::Print("┐"))?;
    text::clear_rest_of_line()?;
    queue!(stdout,
        cursor::MoveTo(0, CALENDAR_MARGIN_TOP + 1),
        style::Print("│"),
        style::Print(" Su  Mo  Tu  We  Th  Fr  Sa │")
    )?;
    // Individual sections
    // Calendar
    let calendar_frame = get_calendar_frame(selected_date.year, selected_date.month);
    let mut cursory = CALENDAR_MARGIN_TOP + 2;
    for week in calendar_frame {
        queue!(stdout,
            cursor::MoveTo(0, cursory),
            style::Print("│"),
        )?;
        let mut cursorx = 1;
        for date in week {
            // Date itself
            let is_selected = date == selected_date.day as i32;
            let empty_tasks = vec![];
            let tasks = calendar_tasks.days.get((date - 1) as usize).unwrap_or(&empty_tasks);
            let date_opt = if date > 0 {
                Some(utils::RicalDate::new(selected_date.year, selected_date.month, date as u32))
            } else {
                None
            };
            render_date_square(date_opt, cursorx, cursory, is_selected, tasks, &currstate.pane)?;

            cursorx += DATE_SQUARE_WIDTH;
        }
        for _i in 0..date_height {
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
    cursory = CALENDAR_MARGIN_TOP + 1;
    let cursorx = CALENDAR_WIDTH;
    for date_offset in 0..DAYS_DISPLAYED {
        let date = selected_date.add_days(date_offset);
        if date.month != currstate.month {
            // Displaying tasks from other months overcomplicates the logic
            break;
        }
        let is_selected = date_offset == 0;
        let is_today = date == utils::RicalDate::today();
        let empty_tasks = vec![];
        let tasks = calendar_tasks.days.get((date.day - 1) as usize).unwrap_or(&empty_tasks);
        cursory = render_tasks_date(date, cursorx, cursory, tasks_pane_width, is_selected, currstate.task_id, is_today, tasks, &currstate.pane)?;
        // Divider between selected date and upcoming dates
        if date_offset == 0 {
            queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
            text::padded_text_styled("──── Upcoming ".dark_grey(), tasks_pane_width, "─".dark_grey())?;
            queue!(stdout, style::Print("│"))?;
            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
            cursory += 1;
            queue!(stdout, cursor::MoveTo(cursorx, cursory))?;
            text::pad_characters(tasks_pane_width, 0, " ")?;
            queue!(stdout, style::Print("│"))?;
            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
            cursory += 1;
        }
    }
    // Clear anything below those dates
    for newy in cursory..calendarframe_bottom_y {
        // Clear out this line
        queue!(stdout, cursor::MoveTo(cursorx, newy))?;
        text::pad_characters(tasks_pane_width, 0, " ")?;
        queue!(stdout, style::Print("│"))?;
        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
    }
    cursory = calendarframe_bottom_y;
    // Bottom container edge
    queue!(stdout, cursor::MoveTo(0, cursory), style::Print("└"))?;
    text::pad_characters(CALENDAR_WIDTH - 2, 0, "─")?;
    queue!(stdout, style::Print("┴"))?;
    text::pad_characters(tasks_pane_width, 0, "─")?;
    queue!(stdout, style::Print("┘"))?;
    text::clear_rest_of_line()?;
    cursory += 1;

    // End
    queue!(stdout, cursor::MoveTo(0, cursory))?;
    text::clear_to_end()?;

    Ok(())
}
