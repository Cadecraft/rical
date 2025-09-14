use std::io;
use crossterm::{
    queue,
    cursor,
    terminal,
    event::{KeyCode, KeyModifiers},
    style
};

use crate::state;
use crate::utils::{KeyInfo, key_pressed};
use crate::styles;

use crate::components::text;
use crate::components::inputtext;

use std::collections::HashMap;

// A reusable form with multiple inputs

enum FormAction {
    CancelAll,
    NextField,
    PrevField,
    Submit,
    NormalTyping
}

pub enum FormResult {
    InProgress,
    /// Result label name: value (the content the user inputted)
    Submit(HashMap<String, String>),
    CancelAll
}

/// Validates the user's inputted string.
/// If okay, returns Ok; if not okay, returns a valid "default" value as the error
pub type FieldValidator = fn(&str) -> Result<(), String>;

pub fn validate_selected<const N: usize>(currstate: &state::FormState<N>, validators: Option<[FieldValidator; N]>) -> [state::TextInputState; N] {
    let selected = &currstate.fields[currstate.form_pos];

    if validators.is_none() {
        return currstate.fields.clone();
    }

    let valid_contents = match validators.unwrap()[currstate.form_pos](&selected.contents) {
        Ok(_) => selected.contents.clone(),
        Err(changed_contents) => changed_contents
    };
    let valid_input = state::TextInputState {
        cursor_pos: valid_contents.chars().count(),
        contents: valid_contents
    };

    let mut res_all = currstate.fields.clone();
    res_all[currstate.form_pos] = valid_input;
    res_all
}

pub fn handle_input<const N: usize>(
    currstate: &state::FormState<N>,
    key: &KeyInfo,
    label_names: [&str; N],
    validators: Option<[FieldValidator; N]>
) -> (state::FormState<N>, FormResult) {
    if currstate.error_message.is_some() {
        if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
            return (currstate.clone(), FormResult::CancelAll);
        } else {
            return (currstate.clone(), FormResult::InProgress);
        }
    }

    // Form navigation behavior is shared across all inputs
    let num_fields = currstate.fields.len();
    let action = if key_pressed(&key, KeyModifiers::NONE, KeyCode::Esc) {
        FormAction::CancelAll
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Enter) {
        if currstate.form_pos == num_fields - 1 {
            FormAction::Submit
        } else {
            FormAction::NextField
        }
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Up) {
        // Used up arrow because Shift+Tab may not be accessible in terminals
        if currstate.form_pos > 0 {
            FormAction::PrevField
        } else {
            FormAction::NormalTyping
        }
    } else if key_pressed(&key, KeyModifiers::NONE, KeyCode::Tab)
        || key_pressed(&key, KeyModifiers::NONE, KeyCode::Down) {
        if currstate.form_pos != num_fields - 1 {
            FormAction::NextField
        } else {
            FormAction::NormalTyping
        }
    } else {
        FormAction::NormalTyping
    };

    match action {
        FormAction::CancelAll => {
            return (currstate.clone(), FormResult::CancelAll);
        },
        FormAction::Submit => {
            let mut results: HashMap<String, String> = HashMap::new();
            let final_validated_fields = validate_selected(currstate, validators);
            for i in 0..N {
                let this_value = final_validated_fields[i].contents.clone();
                results.insert(label_names[i].to_string(), this_value);
            }
            return (currstate.clone(), FormResult::Submit(results));
        },
        FormAction::NextField => {
            return (state::FormState {
                form_pos: currstate.form_pos + 1,
                fields: validate_selected(currstate, validators),
                ..currstate.clone()
            }, FormResult::InProgress)
        },
        FormAction::PrevField => {
            return (state::FormState {
                form_pos: currstate.form_pos - 1,
                fields: validate_selected(currstate, validators),
                ..currstate.clone()
            }, FormResult::InProgress)
        },
        FormAction::NormalTyping => ()
    };

    // Handle input for the selected field (because the user hasn't performed a navigation action)
    let selected = &currstate.fields[currstate.form_pos];
    let res_selected = inputtext::handle_input(selected, key);
    let mut res_all = currstate.fields.clone();
    res_all[currstate.form_pos] = res_selected;
    (state::FormState {
        fields: res_all,
        ..currstate.clone()
    }, FormResult::InProgress)
}

pub struct FormFieldParameters {
    pub name: String,
    pub styles: styles::Styles,
    pub input_mode: inputtext::InputMode
}

pub struct FormDecorationParameters {
    pub text: String,
    pub x: u16,
    pub y: u16,
    pub clear_rest_of_line: bool
}

pub struct FormRenderParameters<const N: usize> {
    /// The string that appears at the top of the form
    pub title: String,
    /// The y-position of the input hint line (e.g. `(enter) Submit`). No elements should be below this
    pub hint_y: u16,
    /// The length of `fields` (N) must equal the number of fields in the state
    /// so that fields[i] must describe the same field as `currstate.fields[i]`
    pub fields: [FormFieldParameters; N],
    /// Additional strings to render on certain lines (e.g. to pad between two input fields on one line)
    pub decoration_strings: Vec<FormDecorationParameters>,
    /// The line numbers to make blank, no less than line 4 as lines 0..=3 are prehandled
    pub clear_lines: Vec<u16>
}

pub fn render<const N: usize>(currstate: &state::FormState<N>, render_params: FormRenderParameters<N>) -> io::Result<()> {
    let num_fields = currstate.fields.len();

    let mut stdout = io::stdout();

    text::println(0, "(esc) back")?;
    text::println(1, "")?;
    text::println(2, &render_params.title)?;
    text::println(3, "")?;

    match &currstate.error_message {
        Some(error) => {
            for (i, error_line) in error.iter().enumerate() {
                text::println(4 + i as u16, &error_line)?;
            }
            text::clear_to_end()?;
        },
        None => {
            for (i, field) in currstate.fields.iter().enumerate() {
                let field_params = &render_params.fields[i];
                let field_styles = styles::Styles {
                    active: i == currstate.form_pos,
                    ..field_params.styles.clone()
                };
                inputtext::render(&field_params.name, &field, &field_styles, &field_params.input_mode)?;
            }
            for y in render_params.clear_lines {
                text::println(y, "")?;
            }
            for decoration in render_params.decoration_strings {
                queue!(
                    stdout,
                    cursor::MoveTo(decoration.x, decoration.y),
                    style::Print(decoration.text)
                )?;
                if decoration.clear_rest_of_line {
                    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
                }
            }

            // TODO: allow a multi-line input?
            text::println(render_params.hint_y, if currstate.form_pos == num_fields - 1 {
                "(enter) Submit"
            } else {
                "(enter) Next field"
            })?;
            text::clear_to_end()?;
        }
    };
    Ok(())
}
