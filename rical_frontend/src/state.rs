/// Stores the entire hierarchy of state in the app
#[derive(Clone)]
pub struct RicalState {
    pub screen_state: ScreenState
}

#[derive(Clone)]
pub enum ScreenState {
    Calendar {
        month: i32,
        year: i32,
        day: i32
    },
    Menu(MenuState),
    ShouldQuit
}

#[derive(Clone)]
pub enum MenuState {
    MainMenu,
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
        username: String,
        password: String
    }
}

#[derive(Clone)]
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
