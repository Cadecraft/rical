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
