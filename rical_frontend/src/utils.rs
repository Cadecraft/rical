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

/// Represent a date internally to Rical
pub struct RicalDate {
    pub year: i32,
    pub month: u32,
    pub day: u32
}

impl RicalDate {
    pub fn new(year: i32, month: u32, day: u32) -> RicalDate {
        // Validate with chrono
        if chrono::NaiveDate::from_ymd_opt(year, month, day).is_none() {
            panic!("This date is invalid (be sure to use 1-based months and days): {}/{}/{}", year, month, day);
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

    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        // self should be valid because of checks in the constructors
        chrono::NaiveDate::from_ymd_opt(self.year, self.month, self.day).expect("The RicalDate is not valid")
    }

    /// Return today's date (in local time)
    pub fn today() -> RicalDate {
        let curr_date = chrono::offset::Local::now().date_naive();
        RicalDate::new(curr_date.year(), curr_date.month0() + 1, curr_date.day0() + 1)
    }

    /// Add a certain number of days and return the new date
    /// Might cross over into a different month or year
    pub fn add_days(&self, days: u64) -> RicalDate {
        RicalDate::from_naive_date(
            self.to_naive_date()
                .checked_add_days(chrono::Days::new(days))
                .expect("Could not add days")
        )
    }

    /// Subtract a certain number of days and return the new date
    /// Might cross over into a different month or year
    pub fn sub_days(&self, days: u64) -> RicalDate {
        RicalDate::from_naive_date(
            self.to_naive_date()
            .checked_sub_days(chrono::Days::new(days))
            .expect("Could not subtract days")
        )
    }

    /// Format the date as a YYYY/MM/DD string
    pub fn format(&self) -> String {
        format!("{}/{}/{}", self.year, fmt_twodigit(self.month), fmt_twodigit(self.day))
    }

    pub fn weekday_name(&self) -> String {
        const WEEKDAY_NAMES: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let weekday = self.to_naive_date().weekday().number_from_sunday() - 1;
        WEEKDAY_NAMES[weekday as usize].to_string()
    }
}

fn next_month(year: i32, month: u32) -> (i32, u32) {
    match month {
        12 => (year + 1, 1),
        _ => (year, month + 1)
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

pub enum GridDirection {
    Left,
    Right,
    Up,
    Down
}

/// Navigate visually between days on the calendar grid, possibly going to a previous or next month
/// Return the new year, month, and day
pub fn calendar_grid_navigation(current_date: &RicalDate, direction: GridDirection) -> RicalDate {
    // New navigation scheme
    match direction {
        GridDirection::Left => current_date.sub_days(1),
        GridDirection::Right => current_date.add_days(1),
        GridDirection::Up => current_date.sub_days(7),
        GridDirection::Down => current_date.add_days(7)
    }
}

/// Format a two-digit number with a leading zero
pub fn fmt_twodigit(number: u32) -> String {
    if number < 10 {
        let mut s = "0".to_string();
        s.push_str(&number.to_string());
        s
    }  else {
        number.to_string()
    }
}

/// Turn minutes into a 24-hour HR:MN format, or an empty string if None
fn fmt_mins(mins_opt: Option<i32>) -> String {
    match mins_opt {
        Some(mins) => format!("{}:{}", fmt_twodigit(mins as u32 / 60), fmt_twodigit(mins as u32 % 60)),
        None => String::new()
    }
}

/// Format a time range of minutes
pub fn fmt_timerange(start_min: Option<i32>, end_min: Option<i32>) -> String {
    if start_min.is_none() && end_min.is_none() {
        return String::new()
    }
    format!("{}-{}", fmt_mins(start_min), fmt_mins(end_min))
}
