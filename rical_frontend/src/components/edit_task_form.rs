use std::io;

use crate::state;
use crate::utils::{KeyInfo, time_shorthand_to_mins};
use crate::api::ApiHandler;
use crate::styles;
use crate::types;

use crate::components::inputtext;
use crate::components::form;

// The form for editing an existing task

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let formstate = currstate.editing_task.as_ref().expect("edit_task_form should never be used if not editing a task");

    // TODO: pass some kind of validator to automatically format a valid time shorthand?
    let res = form::handle_input(&formstate.form, key, [
        "year",
        "month",
        "day",
        "start_shorthand",
        "end_shorthand",
        "title",
        "description",
        "complete"
    ]);
    match res.1 {
        form::FormResult::InProgress => {
            state::ScreenState::Calendar(state::CalendarState {
                editing_task: Some(state::EditTaskState {
                    form: res.0,
                    task_id: formstate.task_id
                }),
                ..currstate.clone()
            })
        },
        form::FormResult::CancelAll => {
            state::ScreenState::Calendar(state::CalendarState {
                editing_task: None,
                ..currstate.clone()
            })
        },
        form::FormResult::Submit(result) => {
            let start_min = time_shorthand_to_mins(&result["start_shorthand"]);
            let end_min = time_shorthand_to_mins(&result["end_shorthand"]);
            // TODO: show loading screen
            // TODO: validation for these params
            let year = match result["year"].parse::<i32>() {
                Ok(y) => y,
                _ => currstate.year
            };
            let month = match result["month"].parse::<i32>() {
                Ok(m) => m,
                _ => currstate.month as i32
            };
            let day = match result["day"].parse::<i32>() {
                Ok(d) => d,
                _ => currstate.day as i32
            };
            let complete = result["complete"] == "Yes";
            // TODO: run with API handler
            let edited_task = types::TaskDataWithId {
                year,
                month,
                day,
                start_min,
                end_min,
                title: result["title"].clone(),
                description: Some(result["description"].clone()),
                complete,
                task_id: formstate.task_id
            };
            state::ScreenState::Calendar(state::CalendarState {
                editing_task: None,
                ..currstate.clone()
            })
        }
    }
}

pub fn render(currstate: &state::CalendarState) -> io::Result<()> {
    let formdata = currstate.editing_task.as_ref().expect("edit_task_form should never be used if not editing a task");

    let render_params = form::FormRenderParameters {
        title: "Edit Task".to_string(),
        fields: [
            form::FormFieldParameters {
                name: "Year".to_string(),
                styles: styles::Styles {
                    margin_top: 4,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Month".to_string(),
                styles: styles::Styles {
                    margin_top: 5,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Day".to_string(),
                styles: styles::Styles {
                    margin_top: 6,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Start".to_string(),
                styles: styles::Styles {
                    margin_top: 8,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "End".to_string(),
                styles: styles::Styles {
                    margin_left: 18,
                    margin_top: 8,
                    width: Some(12),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Title".to_string(),
                styles: styles::Styles {
                    margin_top: 10,
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Description".to_string(),
                styles: styles::Styles {
                    margin_top: 11,
                    width: Some(40),
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
            form::FormFieldParameters {
                name: "Complete".to_string(),
                styles: styles::Styles {
                    margin_top: 13,
                    ..styles::Styles::new()
                },
                input_mode: inputtext::InputMode::Normal
            },
        ],
        decoration_strings: vec![
            form::FormDecorationParameters {
                text: "    ".to_string(),
                x: 14,
                y: 8,
                clear_rest_of_line: false
            }
        ],
        clear_lines: vec![7, 9, 12, 14],
        hint_y: 15,
    };
    form::render(&formdata.form, render_params)?;
    Ok(())
}
