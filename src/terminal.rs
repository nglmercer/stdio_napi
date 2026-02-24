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
