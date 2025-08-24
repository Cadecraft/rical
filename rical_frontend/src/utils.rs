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

pub struct RicalDate {
    pub year: i32,
    pub month: u32,
    pub day: u32
}

impl RicalDate {
    pub fn new(year: i32, month: u32, day: u32) -> RicalDate {
        if month == 0 || day == 0 {
            panic!("A RicalDate can never have a month or day value of 0. Use 1-based days (i.e. January is 1)");
        }

        RicalDate { year, month, day }
    }

    pub fn from_naive_date(naive: chrono::NaiveDate) -> RicalDate {
        RicalDate {
            year: naive.year(),
            month: naive.month0() + 1,
            day: naive.day0() + 1
        }
    }

    /// Return today's date (in local time)
    pub fn today() -> RicalDate {
        let curr_date = chrono::offset::Local::now().date_naive();
        RicalDate::new(curr_date.year(), curr_date.month0() + 1, curr_date.day0() + 1)
    }
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

type CalendarFrame = Vec<Vec<i32>>;

/// Get the 2d array of days for a calendar month (res[row][weekday] gets you the day number of the month)
/// Negative values will represent days outside of the month
pub fn get_calendar_frame(year: i32, month: u32) -> CalendarFrame {
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

pub enum GridDirection {
    Left,
    Right,
    Up,
    Down
}

/// Navigate visually between days on the calendar grid, possibly going to a previous or next month
/// Return the new year, month, and day
pub fn calendar_grid_navigation(curryear: i32, currmonth: u32, currday: u32, direction: GridDirection) -> RicalDate {
    // New navigation scheme
    match direction {
        GridDirection::Left => {
            RicalDate::from_naive_date(
                NaiveDate::from_ymd_opt(curryear, currmonth, currday)
                    .unwrap()
                    .checked_sub_days(chrono::Days::new(1))
                    .expect("Could not sub days (left)")
            )
        },
        GridDirection::Right => {
            RicalDate::from_naive_date(
                NaiveDate::from_ymd_opt(curryear, currmonth, currday)
                    .unwrap()
                    .checked_add_days(chrono::Days::new(1))
                    .expect("Could not add days (right)")
            )
        },
        GridDirection::Up => {
            RicalDate::from_naive_date(
                NaiveDate::from_ymd_opt(curryear, currmonth, currday)
                    .unwrap()
                    .checked_sub_days(chrono::Days::new(7))
                    .expect("Could not sub days")
            )
        },
        GridDirection::Down => {
            RicalDate::from_naive_date(
                NaiveDate::from_ymd_opt(curryear, currmonth, currday)
                    .unwrap()
                    .checked_add_days(chrono::Days::new(7))
                    .expect("Could not add days")
            )
        }
    }
}
