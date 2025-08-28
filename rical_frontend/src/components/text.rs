use std::io;
use crossterm::{
    queue,
    cursor,
    style::{self},
    terminal::{Clear, ClearType}
};

/// Print text at a specific line and clear the rest of that line
pub fn println(y: u16, line: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout,
        // TODO: use my own wrappers around these functionalities
        cursor::MoveTo(0,y),
        style::Print(line),
        Clear(ClearType::UntilNewLine),
    )?;
    Ok(())
}

pub fn cleartoend() -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout,
        Clear(ClearType::FromCursorDown),
    )?;
    Ok(())
}

/// Pad characters to the remaining space
pub fn pad_characters(total_width: u16, taken_up: u16, ch: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    if taken_up >= total_width {
        return Ok(());
    }
    for _i in taken_up..total_width {
        queue!(stdout, style::Print(ch))?;
    }
    Ok(())
}

/// Render text padded to a constant width at wherever the cursor currently is
pub fn padded_text(text: &str, total_width: u16, ch: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    let len = text.len();
    queue!(stdout, style::Print(text))?;
    pad_characters(total_width, len as u16, ch)?;
    Ok(())
}

/// Render styled text padded to a constant width
pub fn padded_text_styled() {
    let mut stdout = io::stdout();
}
