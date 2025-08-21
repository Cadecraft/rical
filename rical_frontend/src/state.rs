/// Stores the entire hierarchy of state in the app
pub struct RicalState {
    pub screen_state: ScreenState
}

pub enum ScreenState {
    Calendar {
        month: i32,
        year: i32,
        day: i32
    },
    Menu(MenuState)
}

pub enum MenuState {
    MainMenu,
    Login(LoginState),
    Signup(SignupState)
}

pub enum LoginState {
    Failed {
        error_message: String
    },
    EnteringInfo {
        form_pos: u32,
        username: String,
        password: String
    }
}

pub enum SignupState {
    Failed {
        error_message: String
    },
    EnteringInfo {
        form_pos: u32,
        username: String,
        password: String
    }
}
