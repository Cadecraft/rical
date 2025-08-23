use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

use chrono::{DateTime, NaiveDate, Local, TimeZone, Datelike};

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
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1
        },
        1
    ).signed_duration_since(NaiveDate::from_ymd(year, month, 1)).num_days() as u32
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

pub enum DayCoordsResult {
    PrevMonth,
    SameMonth(i32),
    NextMonth
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

    for row in 0..frame.len() {
        for weekday in 0..frame[row].len() {
            if frame[row][weekday] == currday {
                match direction {
                    GridDirection::Left => {
                        if weekday == 0 {
                            // TODO: fine-tune which day to go to, specifically
                            return DayCoordsResult::PrevMonth;
                        }
                        let frameres = frame[row][weekday - 1];
                        if frameres < 1 {
                            return DayCoordsResult::PrevMonth;
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Right => {
                        if weekday == 6 {
                            return DayCoordsResult::NextMonth;
                        }
                        let frameres = frame[row][weekday + 1];
                        if frameres < 1 {
                            return DayCoordsResult::NextMonth;
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Up => {
                        if row == 0 {
                            return DayCoordsResult::PrevMonth;
                        }
                        let frameres = frame[row - 1][weekday];
                        if frameres < 1 {
                            return DayCoordsResult::PrevMonth;
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                    GridDirection::Down => {
                        if row == frame.len() - 1 {
                            return DayCoordsResult::NextMonth;
                        }
                        let frameres = frame[row + 1][weekday];
                        if frameres < 1 {
                            return DayCoordsResult::PrevMonth;
                        }
                        return DayCoordsResult::SameMonth(frameres);
                    },
                }
            }
        }
    }
    DayCoordsResult::SameMonth(currday)
}
