use napi_derive::napi;
use crossterm::{
    terminal::{self, size},
    cursor,
    execute,
    Command,
};
use std::io::{stdout, Write};

/// Cursor shape options for terminal
#[napi]
pub enum CursorShape {
    /// Block cursor (default on most terminals)
    Block,
    /// Blinking block cursor
    BlinkingBlock,
    /// Underline cursor
    Underline,
    /// Blinking underline cursor
    BlinkingUnderline,
    /// Vertical bar cursor (I-beam)
    Bar,
    /// Blinking bar cursor
    BlinkingBar,
}

/// Custom scroll region command for crossterm
struct SetScrollRegion {
    top: u16,
    bottom: u16,
}

impl Command for SetScrollRegion {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "\x1b[{};{}r", self.top + 1, self.bottom + 1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        // Scroll region is not well supported on Windows legacy console
        Ok(())
    }
}

#[napi]
pub struct TerminalSize {
    pub columns: u16,
    pub rows: u16,
}

#[napi(object)]
pub struct TerminalInfo {
    pub terminal_type: String,
    pub color_support: String,
}

#[napi]
pub fn get_terminal_info() -> TerminalInfo {
    let terminal_type = std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let color_support = std::env::var("COLORTERM").unwrap_or_else(|_| {
        if terminal_type.contains("256color") {
            "256color".to_string()
        } else {
            "basic".to_string()
        }
    });

    TerminalInfo {
        terminal_type,
        color_support,
    }
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

#[napi]
pub fn show_cursor() -> napi::Result<()> {
    execute!(stdout(), cursor::Show).map_err(|e| {
        napi::Error::from_reason(format!("Failed to show cursor: {}", e))
    })
}

#[napi]
pub fn hide_cursor() -> napi::Result<()> {
    execute!(stdout(), cursor::Hide).map_err(|e| {
        napi::Error::from_reason(format!("Failed to hide cursor: {}", e))
    })
}

#[napi]
pub fn set_terminal_title(title: String) -> napi::Result<()> {
    execute!(stdout(), terminal::SetTitle(title)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to set terminal title: {}", e))
    })
}

#[napi]
pub fn enter_alternate_screen() -> napi::Result<()> {
    execute!(stdout(), terminal::EnterAlternateScreen).map_err(|e| {
        napi::Error::from_reason(format!("Failed to enter alternate screen: {}", e))
    })
}

#[napi]
pub fn leave_alternate_screen() -> napi::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen).map_err(|e| {
        napi::Error::from_reason(format!("Failed to leave alternate screen: {}", e))
    })
}

#[napi]
pub fn enable_raw_mode() -> napi::Result<()> {
    terminal::enable_raw_mode().map_err(|e| {
        napi::Error::from_reason(format!("Failed to enable raw mode: {}", e))
    })
}

#[napi]
pub fn disable_raw_mode() -> napi::Result<()> {
    terminal::disable_raw_mode().map_err(|e| {
        napi::Error::from_reason(format!("Failed to disable raw mode: {}", e))
    })
}

/// Set the cursor shape using ANSI escape codes
#[napi]
pub fn set_cursor_shape(shape: CursorShape) -> napi::Result<()> {
    let ansi_code = match shape {
        CursorShape::Block => "\x1b[0 q",
        CursorShape::BlinkingBlock => "\x1b[1 q",
        CursorShape::Underline => "\x1b[2 q",
        CursorShape::BlinkingUnderline => "\x1b[3 q",
        CursorShape::Bar => "\x1b[4 q",
        CursorShape::BlinkingBar => "\x1b[5 q",
    };
    
    print!("{}", ansi_code);
    let _ = stdout().flush();
    
    Ok(())
}

/// Set the scroll region (top and bottom rows)
#[napi]
pub fn set_scroll_region(top: u16, bottom: u16) -> napi::Result<()> {
    execute!(stdout(), SetScrollRegion { top, bottom }).map_err(|e| {
        napi::Error::from_reason(format!("Failed to set scroll region: {}", e))
    })
}

/// Reset the scroll region to full terminal
#[napi]
pub fn reset_scroll_region() -> napi::Result<()> {
    let (_, rows) = size().map_err(|e| {
        napi::Error::from_reason(format!("Failed to get terminal size: {}", e))
    })?;
    execute!(stdout(), SetScrollRegion { top: 0, bottom: rows.saturating_sub(1) }).map_err(|e| {
        napi::Error::from_reason(format!("Failed to reset scroll region: {}", e))
    })
}

/// Scroll the screen up by n lines
#[napi]
pub fn scroll_up(n: u16) -> napi::Result<()> {
    execute!(stdout(), terminal::ScrollUp(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to scroll up: {}", e))
    })
}

/// Scroll the screen down by n lines
#[napi]
pub fn scroll_down(n: u16) -> napi::Result<()> {
    execute!(stdout(), terminal::ScrollDown(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to scroll down: {}", e))
    })
}

/// Clear the current line
#[napi]
pub fn clear_current_line() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to clear current line: {}", e))
    })
}

/// Clear from cursor to end of line
#[napi]
pub fn clear_until_newline() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::UntilNewLine)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to clear until newline: {}", e))
    })
}

/// Clear from cursor to beginning of line
#[napi]
pub fn clear_from_cursor() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::FromCursorUp)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to clear from cursor: {}", e))
    })
}

/// Save the current cursor position
#[napi]
pub fn save_cursor_position() -> napi::Result<()> {
    execute!(stdout(), cursor::SavePosition).map_err(|e| {
        napi::Error::from_reason(format!("Failed to save cursor position: {}", e))
    })
}

/// Restore the saved cursor position
#[napi]
pub fn restore_cursor_position() -> napi::Result<()> {
    execute!(stdout(), cursor::RestorePosition).map_err(|e| {
        napi::Error::from_reason(format!("Failed to restore cursor position: {}", e))
    })
}

/// Move cursor up by n rows
#[napi]
pub fn move_cursor_up(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveUp(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor up: {}", e))
    })
}

/// Move cursor down by n rows
#[napi]
pub fn move_cursor_down(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveDown(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor down: {}", e))
    })
}

/// Move cursor left by n columns
#[napi]
pub fn move_cursor_left(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveLeft(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor left: {}", e))
    })
}

/// Move cursor right by n columns
#[napi]
pub fn move_cursor_right(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveRight(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor right: {}", e))
    })
}

/// Move cursor to the beginning of the next line (column 0)
#[napi]
pub fn move_cursor_next_line(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToNextLine(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor to next line: {}", e))
    })
}

/// Move cursor to the beginning of the previous line (column 0)
#[napi]
pub fn move_cursor_previous_line(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToPreviousLine(n)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor to previous line: {}", e))
    })
}

/// Move cursor to a specific column on the current row
#[napi]
pub fn move_cursor_to_column(column: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToColumn(column)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor to column: {}", e))
    })
}

/// Move cursor to a specific row at column 0
#[napi]
pub fn move_cursor_to_row(row: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToRow(row)).map_err(|e| {
        napi::Error::from_reason(format!("Failed to move cursor to row: {}", e))
    })
}

/// Check if stdout is connected to a terminal (TTY)
#[napi]
pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Check if stderr is connected to a terminal (TTY)
#[napi]
pub fn is_stderr_tty() -> bool {
    atty::is(atty::Stream::Stderr)
}

/// Check if stdin is connected to a terminal (TTY)
#[napi]
pub fn is_stdin_tty() -> bool {
    atty::is(atty::Stream::Stdin)
}
