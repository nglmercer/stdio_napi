use crossterm::{
    cursor, execute,
    terminal::{self, size},
    Command,
};
use napi_derive::napi;
use std::io::{stdout, Write};

/// Cursor shape options for terminal.
///
/// Use this enum to change the cursor appearance in the terminal.
/// Different terminals support different cursor shapes.
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

/// Terminal size information.
///
/// Contains the number of columns and rows in the terminal.
#[napi]
pub struct TerminalSize {
    /// Number of columns (width)
    pub columns: u16,
    /// Number of rows (height)
    pub rows: u16,
}

/// Terminal information object.
///
/// Contains details about the terminal type and color capabilities.
#[napi(object)]
pub struct TerminalInfo {
    /// The terminal type (e.g., "xterm-256color")
    pub terminal_type: String,
    /// Color support level (e.g., "256color", "basic")
    pub color_support: String,
}

/// Gets information about the terminal.
///
/// Returns the terminal type from the TERM environment variable
/// and color support level from COLORTERM.
///
/// # Returns
/// * `TerminalInfo` - Terminal type and color support information
///
/// # Example
/// ```javascript
/// const { get_terminal_info } = require('stdio-napi');
/// const info = get_terminal_info();
/// console.log(info.terminal_type); // "xterm-256color"
/// ```
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

/// Gets the current terminal size.
///
/// # Returns
/// * `Result<TerminalSize, napi::Error>` - Columns and rows of the terminal
///
/// # Example
/// ```javascript
/// const { get_terminal_size } = require('stdio-napi');
/// const { columns, rows } = get_terminal_size();
/// console.log(`Terminal is ${columns}x${rows}`);
/// ```
#[napi]
pub fn get_terminal_size() -> napi::Result<TerminalSize> {
    // Check if stdout is a TTY before trying to get terminal size
    if !is_stdout_tty() {
        return Err(napi::Error::from_reason(
            "get_terminal_size requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let (columns, rows) = size()
        .map_err(|e| napi::Error::from_reason(format!("Failed to get terminal size: {}", e)))?;

    Ok(TerminalSize { columns, rows })
}

/// Clears the entire screen and moves cursor to home position (0, 0).
///
/// # Example
/// ```javascript
/// const { clear_screen } = require('stdio-napi');
/// clear_screen();
/// ```
#[napi]
pub fn clear_screen() -> napi::Result<()> {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .map_err(|e| napi::Error::from_reason(format!("Failed to clear screen: {}", e)))
}

/// Moves the cursor to the specified position.
///
/// # Arguments
/// * `column` - The column position (0-indexed)
/// * `row` - The row position (0-indexed)
///
/// # Example
/// ```javascript
/// const { move_cursor } = require('stdio-napi');
/// move_cursor(10, 5); // Move to column 10, row 5
/// ```
#[napi]
pub fn move_cursor(column: u16, row: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveTo(column, row))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor: {}", e)))
}

/// Shows the cursor (after it was hidden).
///
/// # Example
/// ```javascript
/// const { show_cursor } = require('stdio-napi');
/// show_cursor();
/// ```
#[napi]
pub fn show_cursor() -> napi::Result<()> {
    execute!(stdout(), cursor::Show)
        .map_err(|e| napi::Error::from_reason(format!("Failed to show cursor: {}", e)))
}

/// Hides the cursor.
///
/// # Example
/// ```javascript
/// const { hide_cursor } = require('stdio-napi');
/// hide_cursor();
/// ```
#[napi]
pub fn hide_cursor() -> napi::Result<()> {
    execute!(stdout(), cursor::Hide)
        .map_err(|e| napi::Error::from_reason(format!("Failed to hide cursor: {}", e)))
}

/// Sets the terminal window title.
///
/// # Arguments
/// * `title` - The title to set
///
/// # Example
/// ```javascript
/// const { set_terminal_title } = require('stdio-napi');
/// set_terminal_title("My Application");
/// ```
#[napi]
pub fn set_terminal_title(title: String) -> napi::Result<()> {
    execute!(stdout(), terminal::SetTitle(title))
        .map_err(|e| napi::Error::from_reason(format!("Failed to set terminal title: {}", e)))
}

/// Enters the alternate screen buffer.
///
/// The alternate screen provides a separate buffer for full-screen applications.
/// Use `leave_alternate_screen()` to return to the main screen.
///
/// # Example
/// ```javascript
/// const { enter_alternate_screen } = require('stdio-napi');
/// enter_alternate_screen();
/// ```
#[napi]
pub fn enter_alternate_screen() -> napi::Result<()> {
    execute!(stdout(), terminal::EnterAlternateScreen)
        .map_err(|e| napi::Error::from_reason(format!("Failed to enter alternate screen: {}", e)))
}

/// Leaves the alternate screen buffer and returns to the main screen.
///
/// # Example
/// ```javascript
/// const { leave_alternate_screen } = require('stdio-napi');
/// leave_alternate_screen();
/// ```
#[napi]
pub fn leave_alternate_screen() -> napi::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)
        .map_err(|e| napi::Error::from_reason(format!("Failed to leave alternate screen: {}", e)))
}

/// Enables raw mode for the terminal.
///
/// In raw mode, input is available character-by-character without line buffering.
/// Use `disable_raw_mode()` to restore normal terminal behavior.
///
/// # Example
/// ```javascript
/// const { enable_raw_mode } = require('stdio-napi');
/// enable_raw_mode();
/// ```
#[napi]
pub fn enable_raw_mode() -> napi::Result<()> {
    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))
}

/// Disables raw mode and restores normal terminal behavior.
///
/// # Example
/// ```javascript
/// const { disable_raw_mode } = require('stdio-napi');
/// disable_raw_mode();
/// ```
#[napi]
pub fn disable_raw_mode() -> napi::Result<()> {
    terminal::disable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to disable raw mode: {}", e)))
}

/// Sets the cursor shape using ANSI escape codes.
///
/// Different cursor shapes can improve visibility or indicate different modes
/// in your application (e.g., insert mode vs. normal mode).
///
/// # Arguments
/// * `shape` - The cursor shape to set (Block, BlinkingBlock, Underline, etc.)
///
/// # Example
/// ```javascript
/// const { set_cursor_shape, CursorShape } = require('stdio-napi');
/// set_cursor_shape(CursorShape.Bar); // Set I-beam cursor
/// ```
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

/// Sets the scroll region to a specific area of the terminal.
///
/// After setting a scroll region, scrolling operations only affect lines
/// within the region. This is useful for creating fixed headers/footers.
///
/// # Arguments
/// * `top` - The top row of the scroll region (0-indexed)
/// * `bottom` - The bottom row of the scroll region (0-indexed)
///
/// # Example
/// ```javascript
/// const { set_scroll_region } = require('stdio-napi');
/// set_scroll_region(2, 20); // Scroll region from row 2 to row 20
/// ```
#[napi]
pub fn set_scroll_region(top: u16, bottom: u16) -> napi::Result<()> {
    execute!(stdout(), SetScrollRegion { top, bottom })
        .map_err(|e| napi::Error::from_reason(format!("Failed to set scroll region: {}", e)))
}

/// Resets the scroll region to the full terminal.
///
/// This restores normal scrolling behavior after a scroll region was set.
///
/// # Example
/// ```javascript
/// const { reset_scroll_region } = require('stdio-napi');
/// reset_scroll_region();
/// ```
#[napi]
pub fn reset_scroll_region() -> napi::Result<()> {
    let (_, rows) = size()
        .map_err(|e| napi::Error::from_reason(format!("Failed to get terminal size: {}", e)))?;
    execute!(
        stdout(),
        SetScrollRegion {
            top: 0,
            bottom: rows.saturating_sub(1)
        }
    )
    .map_err(|e| napi::Error::from_reason(format!("Failed to reset scroll region: {}", e)))
}

/// Scrolls the screen up by n lines.
///
/// New blank lines appear at the bottom of the screen.
///
/// # Arguments
/// * `n` - Number of lines to scroll up
///
/// # Example
/// ```javascript
/// const { scroll_up } = require('stdio-napi');
/// scroll_up(5); // Scroll up 5 lines
/// ```
#[napi]
pub fn scroll_up(n: u16) -> napi::Result<()> {
    execute!(stdout(), terminal::ScrollUp(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to scroll up: {}", e)))
}

/// Scrolls the screen down by n lines.
///
/// New blank lines appear at the top of the screen.
///
/// # Arguments
/// * `n` - Number of lines to scroll down
///
/// # Example
/// ```javascript
/// const { scroll_down } = require('stdio-napi');
/// scroll_down(5); // Scroll down 5 lines
/// ```
#[napi]
pub fn scroll_down(n: u16) -> napi::Result<()> {
    execute!(stdout(), terminal::ScrollDown(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to scroll down: {}", e)))
}

/// Clears the entire current line where the cursor is positioned.
///
/// # Example
/// ```javascript
/// const { clear_current_line } = require('stdio-napi');
/// clear_current_line();
/// ```
#[napi]
pub fn clear_current_line() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine))
        .map_err(|e| napi::Error::from_reason(format!("Failed to clear current line: {}", e)))
}

/// Clears from the cursor position to the end of the current line.
///
/// The cursor position remains unchanged.
///
/// # Example
/// ```javascript
/// const { clear_until_newline } = require('stdio-napi');
/// clear_until_newline();
/// ```
#[napi]
pub fn clear_until_newline() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::UntilNewLine))
        .map_err(|e| napi::Error::from_reason(format!("Failed to clear until newline: {}", e)))
}

/// Clears from the cursor position to the beginning of the current line.
///
/// The cursor position remains unchanged.
///
/// # Example
/// ```javascript
/// const { clear_from_cursor } = require('stdio-napi');
/// clear_from_cursor();
/// ```
#[napi]
pub fn clear_from_cursor() -> napi::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::FromCursorUp))
        .map_err(|e| napi::Error::from_reason(format!("Failed to clear from cursor: {}", e)))
}

/// Saves the current cursor position.
///
/// Use `restore_cursor_position()` to return to the saved position.
/// Only one position can be saved at a time.
///
/// # Example
/// ```javascript
/// const { save_cursor_position, restore_cursor_position } = require('stdio-napi');
/// save_cursor_position();
/// // ... do some cursor movements
/// restore_cursor_position(); // Return to saved position
/// ```
#[napi]
pub fn save_cursor_position() -> napi::Result<()> {
    execute!(stdout(), cursor::SavePosition)
        .map_err(|e| napi::Error::from_reason(format!("Failed to save cursor position: {}", e)))
}

/// Restore the saved cursor position
#[napi]
pub fn restore_cursor_position() -> napi::Result<()> {
    execute!(stdout(), cursor::RestorePosition)
        .map_err(|e| napi::Error::from_reason(format!("Failed to restore cursor position: {}", e)))
}

/// Move cursor up by n rows
#[napi]
pub fn move_cursor_up(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveUp(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor up: {}", e)))
}

/// Move cursor down by n rows
#[napi]
pub fn move_cursor_down(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveDown(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor down: {}", e)))
}

/// Move cursor left by n columns
#[napi]
pub fn move_cursor_left(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveLeft(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor left: {}", e)))
}

/// Move cursor right by n columns
#[napi]
pub fn move_cursor_right(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveRight(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor right: {}", e)))
}

/// Move cursor to the beginning of the next line (column 0)
#[napi]
pub fn move_cursor_next_line(n: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToNextLine(n))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor to next line: {}", e)))
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
    execute!(stdout(), cursor::MoveToColumn(column))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor to column: {}", e)))
}

/// Move cursor to a specific row at column 0
#[napi]
pub fn move_cursor_to_row(row: u16) -> napi::Result<()> {
    execute!(stdout(), cursor::MoveToRow(row))
        .map_err(|e| napi::Error::from_reason(format!("Failed to move cursor to row: {}", e)))
}

/// Check if stdout is connected to a terminal (TTY)
#[napi]
pub fn is_stdout_tty() -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_shape_variants() {
        // Test all cursor shape variants exist
        let _ = CursorShape::Block;
        let _ = CursorShape::BlinkingBlock;
        let _ = CursorShape::Underline;
        let _ = CursorShape::BlinkingUnderline;
        let _ = CursorShape::Bar;
        let _ = CursorShape::BlinkingBar;
    }

    #[test]
    fn test_get_terminal_info() {
        let info = get_terminal_info();
        assert!(!info.terminal_type.is_empty());
        assert!(!info.color_support.is_empty());
    }

    #[test]
    fn test_tty_functions_return_bool() {
        let _stdout_tty = is_stdout_tty();
        let _stderr_tty = is_stderr_tty();
        let _stdin_tty = is_stdin_tty();
    }

    #[test]
    fn test_set_cursor_shape_block() {
        let result = set_cursor_shape(CursorShape::Block);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cursor_shape_blinking_block() {
        let result = set_cursor_shape(CursorShape::BlinkingBlock);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cursor_shape_underline() {
        let result = set_cursor_shape(CursorShape::Underline);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cursor_shape_blinking_underline() {
        let result = set_cursor_shape(CursorShape::BlinkingUnderline);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cursor_shape_bar() {
        let result = set_cursor_shape(CursorShape::Bar);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cursor_shape_blinking_bar() {
        let result = set_cursor_shape(CursorShape::BlinkingBar);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_terminal_title() {
        let result = set_terminal_title("Test Title".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_terminal_title_empty() {
        let result = set_terminal_title("".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_terminal_title_unicode() {
        let result = set_terminal_title("测试标题 🎉".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_terminal_title_long() {
        let long_title = "x".repeat(1000);
        let result = set_terminal_title(long_title);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_zero() {
        let result = move_cursor(0, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_basic() {
        let result = move_cursor(10, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_max_values() {
        let result = move_cursor(u16::MAX, u16::MAX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_up_basic() {
        let result = move_cursor_up(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_up_zero() {
        let result = move_cursor_up(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_up_max() {
        let result = move_cursor_up(u16::MAX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_down_basic() {
        let result = move_cursor_down(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_down_zero() {
        let result = move_cursor_down(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_left_basic() {
        let result = move_cursor_left(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_right_basic() {
        let result = move_cursor_right(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_next_line() {
        let result = move_cursor_next_line(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_previous_line() {
        let result = move_cursor_previous_line(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_to_column() {
        let result = move_cursor_to_column(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_cursor_to_row() {
        let result = move_cursor_to_row(5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_cursor() {
        let result = show_cursor();
        assert!(result.is_ok());
    }

    #[test]
    fn test_hide_cursor() {
        let result = hide_cursor();
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_restore_cursor_position() {
        let save_result = save_cursor_position();
        assert!(save_result.is_ok());

        let restore_result = restore_cursor_position();
        assert!(restore_result.is_ok());
    }

    #[test]
    fn test_scroll_up_basic() {
        let result = scroll_up(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scroll_up_zero() {
        let result = scroll_up(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scroll_down_basic() {
        let result = scroll_down(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scroll_down_zero() {
        let result = scroll_down(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_current_line() {
        let result = clear_current_line();
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_until_newline() {
        let result = clear_until_newline();
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_from_cursor() {
        let result = clear_from_cursor();
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_scroll_region() {
        let result = set_scroll_region(0, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_scroll_region_same_top_bottom() {
        let result = set_scroll_region(5, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_terminal_size_struct() {
        // Test that TerminalSize struct can be created
        let size = TerminalSize {
            columns: 80,
            rows: 24,
        };
        assert_eq!(size.columns, 80);
        assert_eq!(size.rows, 24);
    }

    #[test]
    fn test_terminal_info_struct() {
        // Test that TerminalInfo struct can be created
        let info = TerminalInfo {
            terminal_type: "xterm-256color".to_string(),
            color_support: "256color".to_string(),
        };
        assert_eq!(info.terminal_type, "xterm-256color");
        assert_eq!(info.color_support, "256color");
    }

    #[test]
    fn test_set_scroll_region_command_ansi() {
        let cmd = SetScrollRegion { top: 0, bottom: 10 };
        let mut output = String::new();
        cmd.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[1;11r");
    }

    #[test]
    fn test_set_scroll_region_command_ansi_non_zero() {
        let cmd = SetScrollRegion { top: 5, bottom: 20 };
        let mut output = String::new();
        cmd.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[6;21r");
    }
}
