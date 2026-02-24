use napi_derive::napi;
use crossterm::{
    terminal::{self, size},
    cursor,
    execute,
};
use std::io::stdout;

#[napi]
pub struct TerminalSize {
    pub columns: u16,
    pub rows: u16,
}

#[napi]
pub fn get_terminal_size() -> napi::Result<TerminalSize> {
    let (columns, rows) = size().map_err(|e| {
        napi::Error::from_reason(format!("Failed to get terminal size: {}", e))
    })?;
    
    Ok(TerminalSize { columns, rows })
}

#[napi]
pub fn clear_screen() -> napi::Result<()> {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    ).map_err(|e| {
        napi::Error::from_reason(format!("Failed to clear screen: {}", e))
    })
}

#[napi]
pub fn move_cursor(column: u16, row: u16) -> napi::Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(column, row)
    ).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor: {}", e))
    })
}
