use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

use chrono::{NaiveDate, Datelike};

pub struct KeyInfo {
    pub modifiers: KeyModifiers,
    pub code: KeyCode
}

pub fn read_key_event(event: KeyEvent) -> KeyInfo {
    KeyInfo {
        modifiers: event.modifiers, code: event.code
    }
}

pub fn key_pressed(key: &KeyInfo, modifiers: KeyModifiers, code: KeyCode) -> bool {
    key.code == code && key.modifiers == modifiers
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        next_month(year, month).0,
        next_month(year, month).1,
        1
    )
        .expect("Could not obtain previous month (calculating days in month)")
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect("Could not obtain days in month")).num_days() as u32
}

/// Get the 2d array of days for a calendar month (res[row][weekday] gets you the day number of the month)
/// Negative values will represent days outside of the month
pub fn get_calendar_frame(year: i32, month: u32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let days_in_month = get_days_in_month(year, month);
    const DAYS_PER_WEEK: usize = 7;

    for date in 1..=days_in_month {
        let naive = NaiveDate::from_ymd_opt(year, month as u32, date).expect("Could not generate a chrono::NaiveDate");
        // If naive is a Sunday, weekday will be 0
        let weekday = naive.weekday().number_from_sunday() - 1;
        // If start of a new week, add that new week to the result
        if res.len() == 0 || weekday == 0 {
            res.push(vec![-1; DAYS_PER_WEEK]);
        }
        // Insert in the correct position
        let last_week_index = res.len() - 1;
        res[last_week_index][weekday as usize] = date as i32;
    }

    res
}

pub fn prev_month(year: i32, month: u32) -> (i32, u32) {
    match month {
        1 => (year - 1, 12),
        _ => (year, month - 1)
    }
}

pub fn next_month(year: i32, month: u32) -> (i32, u32) {
    match month {
        12 => (year + 1, 1),
        _ => (year, month + 1)
    }
}

/// The direction of the month, and the day to go to
pub enum DayCoordsResult {
    PrevMonth(i32),
    SameMonth(i32),
    NextMonth(i32)
}

pub enum GridDirection {
    Left,
    Right,
    Up,
    Down
}

/// Navigate visually between days on the calendar grid, possibly going to a previous or next month
pub fn calendar_grid_navigation(curryear: i32, currmonth: i32, currday: i32, direction: GridDirection) -> DayCoordsResult {
    let frame = get_calendar_frame(curryear, currmonth as u32);

    // TODO: make this code look less cooked
    for row in 0..frame.len() {
        for weekday in 0..frame[row].len() {
            if frame[row][weekday] == currday {
                match direction {
                    GridDirection::Left => {
                        if weekday == 0 {
                            let prev = prev_month(curryear, currmonth as u32);
                            let prev_frame = get_calendar_frame(prev.0, prev.1);
                            let new_day = if row >= prev_frame.len() {
                                prev_frame[prev_frame.len() - 1][6]
                            } else {
                                prev_frame[row][6]
                            };
                            return DayCoordsResult::PrevMonth(new_day);
                        }
                        let frameres = frame[row][weekday - 1];
                        if frameres < 1 {
                            return DayCoordsResult::PrevMonth(28);
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Right => {
                        if weekday == 6 {
                            let next = next_month(curryear, currmonth as u32);
                            let next_frame = get_calendar_frame(next.0, next.1);
                            let new_day = if row >= next_frame.len() {
                                next_frame[next_frame.len() - 1][0]
                            } else {
                                next_frame[row][0]
                            };
                            return DayCoordsResult::NextMonth(new_day);
                        }
                        let frameres = frame[row][weekday + 1];
                        if frameres < 1 {
                            return DayCoordsResult::NextMonth(1);
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Up => {
                        if row == 0 {
                            let prev = prev_month(curryear, currmonth as u32);
                            let prev_frame = get_calendar_frame(prev.0, prev.1);
                            let new_day = prev_frame[prev_frame.len() - 1][weekday];
                            return DayCoordsResult::PrevMonth(new_day);
                        }
                        let frameres = frame[row - 1][weekday];
                        if frameres < 1 {
                            return DayCoordsResult::PrevMonth(28);
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Down => {
                        if row == frame.len() - 1 {
                            let next = next_month(curryear, currmonth as u32);
                            let next_frame = get_calendar_frame(next.0, next.1);
                            let new_day = next_frame[0][weekday];
                            return DayCoordsResult::NextMonth(new_day);
                        }
                        let frameres = frame[row + 1][weekday];
                        if frameres < 1 {
                            return DayCoordsResult::NextMonth(1);
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                }
            }
        }
    }
    DayCoordsResult::SameMonth(currday)
}
