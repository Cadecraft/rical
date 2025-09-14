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

/// The state for any text input
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
    /// Create an input from the initial contents and place the cursor at the end of the contents
    pub fn from_contents(contents: String) -> TextInputState {
        TextInputState {
            cursor_pos: contents.chars().count(),
            contents
        }
    }
}

/// The state for any form
#[derive(Clone)]
pub struct FormState<const N: usize> {
    pub form_pos: usize,
    pub fields: [TextInputState; N],
    /// If some, then an error has occured
    /// This can be multiple lines (multiple elements in the Vec)
    pub error_message: Option<Vec<String>>
}

impl<const N: usize> FormState<N> {
    pub fn new() -> FormState<N> {
        FormState {
            form_pos: 0,
            fields: core::array::from_fn::<TextInputState, N, _>(|_| TextInputState::new()),
            error_message: None
        }
    }

    pub fn from_field_contents(form_pos: usize, contents: [String; N]) -> FormState<N> {
        FormState {
            form_pos,
            fields: contents.map(|c| TextInputState::from_contents(c)),
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
    pub editing_task: Option<EditTaskState>,
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
            making_new_task: None,
            editing_task: None,
        }
    }
}

#[derive(Clone)]
pub struct EditTaskState {
    pub task_id: i64,
    pub form: FormState<8>
}

#[derive(Clone)]
pub enum CalendarPane {
    Month,
    Tasks
}
