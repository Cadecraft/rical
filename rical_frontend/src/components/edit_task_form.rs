use std::io;

use crate::state;
use crate::utils::{KeyInfo, time_shorthand_to_mins};
use crate::api::ApiHandler;
use crate::styles;
use crate::types;

use crate::components::form;

// The form for editing an existing task

pub fn handle_input(currstate: &state::CalendarState, key: &KeyInfo, api_handler: &mut ApiHandler) -> state::ScreenState {
    let formstate = currstate.editing_task.as_ref().expect("edit_task_form should never be used if not editing a task");

    let res = form::handle_input(&formstate.form, key, [
        "year",
        "month",
        "day",
        "start_shorthand",
        "end_shorthand",
        "title",
        "description",
        "complete"
    ], Some([
        |input| match input.parse::<i32>() {
            Ok(_) => Ok(()),
            _ => Err("2000".to_string())
        },
        |input| match input.parse::<i32>() {
            Ok(y) if y > 0 && y <= 12 => Ok(()),
            _ => Err("1".to_string())
        },
        |input| match input.parse::<i32>() {
            Ok(y) if y > 0 && y <= 31 => Ok(()),
            _ => Err("1".to_string())
        },
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
        |input| {
            if input == "Yes" || input == "No" {
                Ok(())
            } else {
                Err("No".to_string())
            }
        },
    ]));
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
            // Because of validators (above), we can assume this processing will be valid
            let year = result["year"].parse::<i32>().unwrap();
            let month = result["month"].parse::<i32>().unwrap();
            let day = result["day"].parse::<i32>().unwrap();
            let complete = result["complete"] == "Yes";
            let new_task = types::TaskDataWithId {
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
            match api_handler.update_task(&new_task) {
                Ok(date_changed) => state::ScreenState::Calendar(state::CalendarState {
                    // Prevent referencing a task that's been moved to a different day
                    task_id: if date_changed { None } else { currstate.task_id },
                    editing_task: None,
                    ..currstate.clone()
                }),
                Err(_) => state::ScreenState::Calendar(state::CalendarState {
                    editing_task: Some(state::EditTaskState {
                        task_id: formstate.task_id,
                        form: state::FormState::from_result_message(vec![
                            "This task could not be edited. Make sure you entered valid times".to_string()
                        ])
                    }),
                    ..currstate.clone()
                })
            }
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
                    gap: Some(2),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Month".to_string(),
                styles: styles::Styles {
                    margin_top: 5,
                    width: Some(14),
                    gap: Some(1),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Day".to_string(),
                styles: styles::Styles {
                    margin_top: 6,
                    width: Some(14),
                    gap: Some(3),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Start".to_string(),
                styles: styles::Styles {
                    margin_top: 8,
                    width: Some(14),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "End".to_string(),
                styles: styles::Styles {
                    margin_left: 18,
                    margin_top: 8,
                    width: Some(12),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Title".to_string(),
                styles: styles::Styles {
                    margin_top: 10,
                    width: Some(40),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Descr".to_string(),
                styles: styles::Styles {
                    margin_top: 11,
                    width: Some(40),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
            },
            form::FormFieldParameters {
                name: "Complete".to_string(),
                styles: styles::Styles {
                    margin_top: 13,
                    width: Some(7),
                    ..styles::Styles::new()
                },
                ..form::FormFieldParameters::default()
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
