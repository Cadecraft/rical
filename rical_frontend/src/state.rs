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
    Login(FormState<2>),
    Signup(FormState<2>)
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

/// The info for any form
#[derive(Clone)]
pub struct FormState<const N: usize> {
    pub form_pos: usize,
    pub fields: [TextInputState; N],
    /// If some, then an error has occured
    /// This can be multiple lines (multiple elements in the Vec)
    pub error_message: Option<Vec<String>>
}

impl<const N: usize> FormState<N> {
    /// Create
    pub fn new() -> FormState<N> {
        FormState {
            form_pos: 0,
            fields: core::array::from_fn::<TextInputState, N, _>(|_| TextInputState::new()),
            error_message: None
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
    pub making_new_task: Option<FormState<4>>,
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
pub enum CalendarPane {
    Month,
    Tasks
}
