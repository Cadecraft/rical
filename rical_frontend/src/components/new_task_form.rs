use std::io;

use crate::state;
use crate::utils::{self, KeyInfo, time_shorthand_to_mins, display_error};
use crate::api::ApiHandler;
use crate::styles;
use crate::types;

use crate::components::inputtext;
use crate::components::form;

// The form for creating a new task

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let formstate = currstate.making_new_task.as_ref().expect("new_task_form should never be used if not making a new task");

    let res = form::handle_input(formstate, key, [
        "start_shorthand",
        "end_shorthand",
        "title",
        "description"
    ], Some([
        |input| match time_shorthand_to_mins(input) {
            Some(_) => Ok(()),
            None => Err(String::new())
        },
        |input| match time_shorthand_to_mins(input) {
            Some(_) => Ok(()),
            None => Err(String::new())
        },
        |_| Ok(()),
        |_| Ok(()),
    ]));
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
        form::FormResult::Submit(result) => {
            let start_min = time_shorthand_to_mins(&result["start_shorthand"]);
            let end_min = time_shorthand_to_mins(&result["end_shorthand"]);
            // TODO: show loading screen
            let new_task = types::TaskData {
                year: currstate.year,
                month: currstate.month as i32,
                day: currstate.day as i32,
                start_min,
                end_min,
                title: result["title"].clone(),
                description: Some(result["description"].clone()),
                complete: false
            };
            match api_handler.post_new_task(&new_task) {
                Ok(_) => state::ScreenState::Calendar(state::CalendarState {
                    making_new_task: None,
                    ..currstate.clone()
                }),
                Err(err) => state::ScreenState::Calendar(state::CalendarState {
                    making_new_task: Some(state::FormState::from_result_message(vec![
                        "Could not create task:".to_string(),
                        format!("  - {}", display_error(err)),
                        "Check that you entered valid times".to_string()
                    ])),
                    ..currstate.clone()
                })
            }
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
                    width: Some(40),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Descr".to_string(),
                styles: styles::Styles {
                    margin_top: 8,
                    width: Some(40),
                    height: Some(3),
                    wrap_text: true,
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
        clear_lines: vec![6, 11],
        hint_y: 12,
    };
    form::render(formdata, render_params)?;
    Ok(())
}
