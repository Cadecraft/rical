use std::io;

use crate::state;
use crate::utils::{self, KeyInfo, time_shorthand_to_mins};
use crate::api::ApiHandler;
use crate::styles;
use crate::types;

use crate::components::inputtext;
use crate::components::form;

// The form for creating a new task

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let formstate = currstate.making_new_task.as_ref().expect("new_task_form should never be used if not making a new task");

    // TODO: pass some kind of validator to automatically format a valid time shorthand?
    let res = form::handle_input(formstate, key);
    match res.1 {
        form::FormResult::InProgress => {
            state::ScreenState::Calendar(state::CalendarState {
                making_new_task: Some(res.0),
                ..currstate.clone()
            })
        },
        form::FormResult::CancelAll => {
            state::ScreenState::Calendar(state::CalendarState {
                making_new_task: None,
                ..currstate.clone()
            })
        },
        form::FormResult::Submit => {
            // TODO: use named return value map (from a form refactor)
            let start_min = time_shorthand_to_mins(&formstate.fields[0].contents);
            let end_min = time_shorthand_to_mins(&formstate.fields[1].contents);
            // TODO: show loading screen
            // TODO: run with API handler
            let new_task = types::TaskData {
                year: currstate.year,
                month: currstate.month as i32,
                day: currstate.day as i32,
                start_min,
                end_min,
                title: formstate.fields[2].contents.clone(),
                description: Some(formstate.fields[2].contents.clone()),
                complete: false
            };
            state::ScreenState::Calendar(state::CalendarState {
                making_new_task: None,
                ..currstate.clone()
            })
        }
    }
}

pub fn render(currstate: &state::CalendarState) -> io::Result<()> {
    let formdata = currstate.making_new_task.as_ref().expect("new_task_form should never be used if not making a new task");

    let currdate = utils::RicalDate {
        year: currstate.year,
        month: currstate.month,
        day: currstate.day
    };

    let render_params = form::FormRenderParameters {
        title: "New Task".to_string(),
        hint_y: 10,
        fields: [
            form::FormFieldParameters {
                name: "Start".to_string(),
                styles: styles::Styles {
                    margin_top: 5,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "End".to_string(),
                styles: styles::Styles {
                    margin_left: 18,
                    margin_top: 5,
                    width: Some(12),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Title".to_string(),
                styles: styles::Styles {
                    margin_top: 7,
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Description".to_string(),
                styles: styles::Styles {
                    margin_top: 8,
                    width: Some(40),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
        ],
        decoration_strings: vec![
            form::FormDecorationParameters {
                text: format!("{} - {}", currdate.format(), currdate.weekday_name()),
                x: 0,
                y: 4,
                clear_rest_of_line: true
            },
            form::FormDecorationParameters {
                text: "    ".to_string(),
                x: 14,
                y: 5,
                clear_rest_of_line: false
            }
        ],
        clear_lines: vec![6, 9]
    };
    form::render(formdata, render_params)?;
    Ok(())
}
