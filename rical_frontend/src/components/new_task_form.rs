use std::io;
use crossterm::{
    queue,
    cursor,
    event::{KeyCode, KeyModifiers},
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};
use crate::api::ApiHandler;
use crate::styles;

use crate::components::text;
use crate::components::inputtext;

// The form for creating a new task

// TODO: refactor into a Form component with its own state type
enum FormAction {
    CancelAll,
    NextField,
    PrevField,
    Submit
}

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let formdata = currstate.making_new_task.as_ref().expect("new_task_form should never be used if not making a new task");

    // Form navigation behavior is shared across all inputs
    const NUM_FIELDS: u32 = 4;
    let action = if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
        Some(FormAction::CancelAll)
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Enter) {
        if formdata.form_pos == NUM_FIELDS - 1 {
            Some(FormAction::Submit)
        } else {
            Some(FormAction::NextField)
        }
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Up) {
        // Used up arrow because Shift+Tab may not be accessible in terminals
        if formdata.form_pos > 0 {
            Some(FormAction::PrevField)
        } else {
            None
        }
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Tab)
        || key_pressed(&key, KeyModifiers::NONE, KeyCode::Down) {
        if formdata.form_pos != NUM_FIELDS - 1 {
            Some(FormAction::NextField)
        } else {
            None
        }
    } else {
        None
    };

    if action.is_some() { match action.unwrap() {
        FormAction::CancelAll => {
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: None,
                ..currstate.clone()
            });
        },
        FormAction::Submit => {
            // TODO: impl
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: None,
                ..currstate.clone()
            })
        },
        FormAction::NextField => {
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState {
                    form_pos: formdata.form_pos + 1,
                    ..formdata.clone()
                }),
                ..currstate.clone()
            });
        },
        FormAction::PrevField => {
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState {
                    form_pos: formdata.form_pos - 1,
                    ..formdata.clone()
                }),
                ..currstate.clone()
            });
        }
    }};

    match formdata.form_pos {
        0 => {
            // Do the first input
            let res = inputtext::handle_input(&formdata.start_time, key);
            // TODO: form validation
            // TODO: render the validation error
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState {
                    form_pos: formdata.form_pos + if res.1 { 1 } else { 0 },
                    start_time: res.0,
                    ..formdata.clone()
                }),
                ..currstate.clone()
            })
        },
        1 => {
            let res = inputtext::handle_input(&formdata.end_time, key);
            // TODO: form validation
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState {
                    form_pos: formdata.form_pos + if res.1 { 1 } else { 0 },
                    end_time: res.0,
                    ..formdata.clone()
                }),
                ..currstate.clone()
            })
        },
        2 => {
            let res = inputtext::handle_input(&formdata.title, key);
            return state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(state::CalendarNewTaskState {
                    form_pos: formdata.form_pos + if res.1 { 1 } else { 0 },
                    title: res.0,
                    ..formdata.clone()
                }),
                ..currstate.clone()
            })
        },
        _ => {
            let res = inputtext::handle_input(&formdata.descr, key);
            if res.1 {
                // TODO: submit
                return state::ScreenState::Calendar(state::CalendarState {
                    making_new_task: None,
                    ..currstate.clone()
                })
            } else {
                return state::ScreenState::Calendar(state::CalendarState {
                    making_new_task: Some(state::CalendarNewTaskState {
                        form_pos: formdata.form_pos,
                        descr: res.0,
                        ..formdata.clone()
                    }),
                    ..currstate.clone()
                })
            }
        },
    }
}

pub fn render(currstate: &state::CalendarState) -> io::Result<()> {
    let mut stdout = io::stdout();

    let formdata = currstate.making_new_task.as_ref().expect("new_task_form should never be used if not making a new task");
    text::println(0, "(esc) back")?;
    text::println(1, "")?;
    text::println(2, "New Task")?;
    text::println(3, "")?;
    text::println(4, &format!("{} - {}", formdata.date.format(), formdata.date.weekday_name()))?;
    inputtext::render("Start", &formdata.start_time, &styles::Styles {
        margin_top: 5,
        width: Some(14),
        active: formdata.form_pos == 0,
        last_in_row: Some(false),
        ..styles::Styles::new()
    }, &inputtext::InputMode::Normal)?;

    queue!(stdout, cursor::MoveTo(14, 5))?;
    text::pad_characters(4, 0, " ")?;

    inputtext::render("End", &formdata.end_time, &styles::Styles {
        margin_left: 18,
        margin_top: 5,
        width: Some(12),
        active: formdata.form_pos == 1,
        ..styles::Styles::new()
    }, &inputtext::InputMode::Normal)?;
    text::println(6, "")?;
    inputtext::render("Title", &formdata.title, &styles::Styles {
        margin_top: 7,
        active: formdata.form_pos == 2,
        ..styles::Styles::new()
    }, &inputtext::InputMode::Normal)?;
    // TODO: use a multi-line input
    inputtext::render("Description", &formdata.descr, &styles::Styles {
        margin_top: 8,
        width: Some(40),
        active: formdata.form_pos == 3,
        ..styles::Styles::new()
    }, &inputtext::InputMode::Normal)?;
    text::println(9, "")?;
    // TODO: render errors
    text::println(10, if formdata.form_pos == 3 { "(enter) Submit" } else { "(enter) Next field" })?;
    text::clear_to_end()?;

    Ok(())
}
