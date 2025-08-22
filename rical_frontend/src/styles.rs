use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::{self, Stylize, Color},
};

pub struct Styles {
    margin_left: u16,
    margin_top: u16,
    color: Option<Color>,
    background_color: Option<Color>
}

impl Styles {
    /// Create the default Styles object
    /// Example:
    /// ```
    /// Styles {
    ///    margin_left: Some(1),
    ///    margin_top: Some(2),
    ///    color: Some(Color::Red),
    ///    ..Styles::new()
    /// }
    /// ```
    pub fn new() -> Styles {
        Styles {
            margin_left: 0,
            margin_top: 0,
            color: None,
            background_color: None
        }
    }
}

/// Compose a parent's styles with a child's
/// using simple specificity rules similar to those of CSS.
/// (e.g. margins will be added together
/// and child colors take precedence over parent colors)
pub fn compose_styles(parent_styles: &Styles, child_styles: &Styles) -> Styles {
    Styles {
        margin_left: parent_styles.margin_left + child_styles.margin_left,
        margin_top: parent_styles.margin_top + child_styles.margin_top,
        color: match child_styles.color {
            Some(c) => Some(c),
            None => parent_styles.color
        },
        background_color: match child_styles.background_color {
            Some(c) => Some(c),
            None => parent_styles.background_color
        }
    }
}
