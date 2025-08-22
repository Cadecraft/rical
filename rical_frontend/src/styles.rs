use crossterm::style::Color;

pub struct Styles {
    pub margin_left: u16,
    pub margin_top: u16,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub width: Option<u16>,
    pub active: bool
}

impl Styles {
    /// Create the default Styles object
    /// Can be used for easily constructing styles, e.g.
    /// ```
    /// Styles {
    ///    margin_left: 1,
    ///    margin_top: 2,
    ///    color: Some(Color::Red),
    ///    ..Styles::new()
    /// }
    /// ```
    pub fn new() -> Styles {
        Styles {
            margin_left: 0,
            margin_top: 0,
            color: None,
            background_color: None,
            width: None,
            active: false
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
        },
        width: match child_styles.width {
            Some(w) => Some(w),
            None => parent_styles.width
        },
        active: child_styles.active
    }
}
