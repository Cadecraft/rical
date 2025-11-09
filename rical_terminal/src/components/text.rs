use crossterm::{
    cursor, queue, style,
    terminal::{Clear, ClearType},
};
use std::io;

/// Print text at a specific line and clear the rest of that line
pub fn println(y: u16, line: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        // TODO: use my own wrappers around these functionalities
        cursor::MoveTo(0, y),
        style::Print(line),
        Clear(ClearType::UntilNewLine),
    )?;
    Ok(())
}

/// Clear the screen from the cursor down
pub fn clear_to_end() -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout, Clear(ClearType::FromCursorDown),)?;
    Ok(())
}

/// Clear one line from the cursor to the end
pub fn clear_rest_of_line() -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout, Clear(ClearType::UntilNewLine),)?;
    Ok(())
}

/// Pad characters to the remaining space
pub fn pad_characters(total_width: u16, taken_up: u16, ch: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    for _i in taken_up..total_width {
        queue!(stdout, style::Print(ch))?;
    }
    Ok(())
}

/// Pad characters to the remaining space (styled)
pub fn pad_characters_styled(
    total_width: u16,
    taken_up: u16,
    ch: style::StyledContent<&str>,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    for _i in taken_up..total_width {
        queue!(stdout, style::PrintStyledContent(ch))?;
    }
    Ok(())
}

/// Render text padded to a constant width at wherever the cursor currently is
pub fn padded_text(text: &str, total_width: u16, ch: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    let len = text.chars().count();
    queue!(stdout, style::Print(text))?;
    pad_characters(total_width, len as u16, ch)?;
    Ok(())
}

/// Render text padded to a constant width at wherever the cursor currently is (styled)
pub fn padded_text_styled(
    text: style::StyledContent<&str>,
    total_width: u16,
    ch: style::StyledContent<&str>,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let len = text.content().chars().count();
    queue!(stdout, style::PrintStyledContent(text))?;
    pad_characters_styled(total_width, len as u16, ch)?;
    Ok(())
}
