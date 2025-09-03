use crate::utils::RicalDate;

/// Stores the entire hierarchy of state in the app
/// `screen_state` deals with the state of the UI
/// Other fields can be added to represent important global state
#[derive(Clone)]
pub struct RicalState {
    pub screen_state: ScreenState
}

#[derive(Clone)]
pub enum ScreenState {
    Calendar (CalendarState),
    Menu(MenuState),
    ShouldQuit
}

#[derive(Clone)]
pub enum MenuState {
    MainMenu,
    About,
    Login(LoginState),
    Signup(SignupState)
}

#[derive(Clone)]
pub enum LoginState {
    Failed {
        error_message: String
    },
    EnteringInfo {
        form_pos: u32,
        username: TextInputState,
        password: TextInputState
    }
}

#[derive(Clone)]
pub enum SignupState {
    Failed {
        error_message: String
    },
    EnteringInfo {
        form_pos: u32,
        username: TextInputState,
        password: TextInputState
    }
}

/// The info for any text input
#[derive(Clone)]
pub struct TextInputState {
    pub cursor_pos: usize,
    pub contents: String
}

impl TextInputState {
    pub fn new() -> TextInputState {
        TextInputState {
            cursor_pos: 0,
            contents: String::new()
        }
    }
}

#[derive(Clone)]
pub struct CalendarState {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub task_id: Option<i64>,
    pub pane: CalendarPane,
    pub making_new_task: Option<CalendarNewTaskState>,
}

impl CalendarState {
    pub fn new() -> CalendarState {
        // TODO: get current year/day, or ask for those as parameters
        CalendarState {
            year: 2025,
            month: 8,
            day: 5,
            task_id: None,
            pane: CalendarPane::Month,
            making_new_task: None
        }
    }
}

#[derive(Clone)]
pub struct CalendarNewTaskState {
    pub form_pos: u32,
    pub date: RicalDate,
    pub start_time: TextInputState,
    pub end_time: TextInputState,
    pub title: TextInputState,
    pub descr: TextInputState
}

impl CalendarNewTaskState {
    pub fn new(current_date: RicalDate) -> CalendarNewTaskState {
        CalendarNewTaskState {
            form_pos: 0,
            date: current_date.clone(),
            start_time: TextInputState::new(),
            end_time: TextInputState::new(),
            title: TextInputState::new(),
            descr: TextInputState::new()
        }
    }
}

#[derive(Clone)]
pub enum CalendarPane {
    Month,
    Tasks
}
