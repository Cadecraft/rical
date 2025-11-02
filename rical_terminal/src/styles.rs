#[derive(Clone)]
pub struct Styles {
    pub margin_left: u16,
    pub margin_top: u16,
    pub width: Option<u16>,
    pub active: bool,
    /// Whether this item is the last in its row, and thus should safely clear everything to the right
    pub last_in_row: Option<bool>,
    pub gap: Option<u16>,
    pub wrap_text: bool,
    pub height: Option<u16>,
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
            width: None,
            active: false,
            last_in_row: None,
            gap: None,
            wrap_text: false,
            height: None,
        }
    }
}
