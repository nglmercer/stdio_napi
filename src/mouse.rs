//! Mouse event support for terminal applications.
//!
//! This module provides mouse event capture and handling capabilities
//! for terminal applications that support mouse reporting.

use crossterm::{
    event::{self, Event, MouseButton, MouseEvent, MouseEventKind},
    execute,
};
use napi_derive::napi;
use std::io::stdout;

/// Mouse button types
#[napi]
pub enum MouseButtonEnum {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// No button (mouse movement)
    None,
}

/// Mouse event types
#[napi]
pub enum MouseEventType {
    /// Button was pressed down
    Down,
    /// Button was released
    Up,
    /// Button was clicked (pressed and released)
    Click,
    /// Mouse was dragged while button held
    Drag,
    /// Mouse moved without button
    Move,
    /// Scroll up
    ScrollUp,
    /// Scroll down
    ScrollDown,
}

/// Mouse event data
#[napi(object)]
pub struct MouseEventInfo {
    /// Event type (down, up, click, drag, move, scrollUp, scrollDown)
    pub event_type: String,
    /// Mouse button involved
    pub button: String,
    /// Column position (0-indexed)
    pub column: u16,
    /// Row position (0-indexed)
    pub row: u16,
    /// Whether Ctrl was pressed
    pub ctrl: bool,
    /// Whether Alt was pressed
    pub alt: bool,
    /// Whether Shift was pressed
    pub shift: bool,
}

/// Enables mouse event reporting in the terminal.
///
/// After calling this function, the terminal will report mouse events
/// that can be read with `read_mouse_event`.
///
/// # Example
/// ```javascript
/// const { enable_mouse } = require('stdio-napi');
/// enable_mouse();
/// // Terminal now reports mouse events
/// ```
#[napi]
pub fn enable_mouse() -> napi::Result<()> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "enable_mouse requires a terminal (TTY).".to_string(),
        ));
    }

    execute!(stdout(), event::EnableMouseCapture)
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable mouse capture: {}", e)))
}

/// Disables mouse event reporting in the terminal.
///
/// Call this function when you're done capturing mouse events
/// to restore normal terminal behavior.
///
/// # Example
/// ```javascript
/// const { disable_mouse } = require('stdio-napi');
/// disable_mouse();
/// ```
#[napi]
pub fn disable_mouse() -> napi::Result<()> {
    execute!(stdout(), event::DisableMouseCapture)
        .map_err(|e| napi::Error::from_reason(format!("Failed to disable mouse capture: {}", e)))
}

/// Reads a single mouse event from the terminal.
///
/// This function blocks until a mouse event is received.
/// Make sure to call `enable_mouse()` first.
///
/// # Returns
/// * `Result<MouseEventInfo, napi::Error>` - The mouse event information
///
/// # Example
/// ```javascript
/// const { enable_mouse, read_mouse_event, disable_mouse } = require('stdio-napi');
/// enable_mouse();
/// try {
///   const event = await read_mouse_event();
///   console.log(`Mouse clicked at (${event.column}, ${event.row})`);
/// } finally {
///   disable_mouse();
/// }
/// ```
#[napi]
pub async fn read_mouse_event() -> napi::Result<MouseEventInfo> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_mouse_event requires a terminal (TTY).".to_string(),
        ));
    }

    loop {
        if event::poll(std::time::Duration::from_millis(100))
            .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
        {
            let event = event::read()
                .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?;

            if let Event::Mouse(mouse_event) = event {
                return Ok(mouse_event_to_info(mouse_event));
            }
        }
    }
}

/// Reads a mouse event with a timeout.
///
/// # Arguments
/// * `timeout_ms` - Timeout in milliseconds
///
/// # Returns
/// * `Result<Option<MouseEventInfo>, napi::Error>` - Mouse event or None if timeout
///
/// # Example
/// ```javascript
/// const { enable_mouse, read_mouse_event_timeout, disable_mouse } = require('stdio-napi');
/// enable_mouse();
/// const event = await read_mouse_event_timeout(5000); // 5 second timeout
/// disable_mouse();
/// if (event) {
///   console.log(`Mouse event at (${event.column}, ${event.row})`);
/// }
/// ```
#[napi]
pub async fn read_mouse_event_timeout(timeout_ms: u32) -> napi::Result<Option<MouseEventInfo>> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_mouse_event_timeout requires a terminal (TTY).".to_string(),
        ));
    }

    let timeout = std::time::Duration::from_millis(timeout_ms as u64);
    let start = std::time::Instant::now();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= timeout {
            return Ok(None);
        }

        let remaining = timeout - elapsed;
        if event::poll(remaining)
            .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
        {
            let event = event::read()
                .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?;

            if let Event::Mouse(mouse_event) = event {
                return Ok(Some(mouse_event_to_info(mouse_event)));
            }
        }
    }
}

/// Waits for a mouse click at a specific position.
///
/// # Arguments
/// * `column` - Expected column position
/// * `row` - Expected row position
/// * `tolerance` - Position tolerance in cells (default: 0)
///
/// # Returns
/// * `Result<bool, napi::Error>` - True if click was at the specified position
///
/// # Example
/// ```javascript
/// const { enable_mouse, wait_for_click_at, disable_mouse } = require('stdio-napi');
/// enable_mouse();
/// const clicked = await wait_for_click_at(10, 5, 2); // Click near (10, 5) with tolerance of 2
/// disable_mouse();
/// ```
#[napi]
pub async fn wait_for_click_at(
    column: u16,
    row: u16,
    tolerance: Option<u16>,
) -> napi::Result<bool> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "wait_for_click_at requires a terminal (TTY).".to_string(),
        ));
    }

    let tol = tolerance.unwrap_or(0);

    loop {
        let event = read_mouse_event().await?;

        if event.event_type == "click" || event.event_type == "up" {
            let col_match = event.column >= column.saturating_sub(tol)
                && event.column <= column.saturating_add(tol);
            let row_match =
                event.row >= row.saturating_sub(tol) && event.row <= row.saturating_add(tol);

            if col_match && row_match {
                return Ok(true);
            }
        }
    }
}

/// Waits for a mouse click within a rectangular region.
///
/// # Arguments
/// * `start_column` - Left edge of region
/// * `start_row` - Top edge of region
/// * `end_column` - Right edge of region
/// * `end_row` - Bottom edge of region
///
/// # Returns
/// * `Result<MouseEventInfo, napi::Error>` - The click event
///
/// # Example
/// ```javascript
/// const { enable_mouse, wait_for_click_in_region, disable_mouse } = require('stdio-napi');
/// enable_mouse();
/// const event = await wait_for_click_in_region(0, 0, 20, 10);
/// disable_mouse();
/// console.log(`Clicked at (${event.column}, ${event.row})`);
/// ```
#[napi]
pub async fn wait_for_click_in_region(
    start_column: u16,
    start_row: u16,
    end_column: u16,
    end_row: u16,
) -> napi::Result<MouseEventInfo> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "wait_for_click_in_region requires a terminal (TTY).".to_string(),
        ));
    }

    loop {
        let event = read_mouse_event().await?;

        if event.event_type == "click" || event.event_type == "up" {
            let in_region = event.column >= start_column
                && event.column <= end_column
                && event.row >= start_row
                && event.row <= end_row;

            if in_region {
                return Ok(event);
            }
        }
    }
}

/// Mouse event listener for continuous mouse monitoring.
#[napi]
pub struct MouseEventListener {
    running: std::sync::Arc<tokio::sync::Mutex<bool>>,
}

#[napi]
impl MouseEventListener {
    /// Creates a new mouse event listener.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            running: std::sync::Arc::new(tokio::sync::Mutex::new(true)),
        }
    }

    /// Starts listening for mouse events.
    /// Returns the next mouse event.
    #[napi]
    pub async fn listen(&self) -> napi::Result<MouseEventInfo> {
        // Check if stdin is a TTY
        if !atty::is(atty::Stream::Stdin) {
            return Err(napi::Error::from_reason(
                "listen requires a terminal (TTY).".to_string(),
            ));
        }

        read_mouse_event().await
    }

    /// Stops the listener.
    #[napi]
    pub async fn stop(&self) -> napi::Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        Ok(())
    }

    /// Checks if the listener is running.
    #[napi]
    pub async fn is_running(&self) -> napi::Result<bool> {
        let running = self.running.lock().await;
        Ok(*running)
    }
}

impl Default for MouseEventListener {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if mouse events are supported in the current terminal.
///
/// # Returns
/// * `bool` - True if mouse events are likely supported
#[napi]
pub fn is_mouse_supported() -> bool {
    // Check if we're in a terminal
    if !atty::is(atty::Stream::Stdin) {
        return false;
    }

    // Check terminal type for known mouse-supporting terminals
    if let Ok(term) = std::env::var("TERM") {
        // Most modern terminals support mouse events
        matches!(
            term.as_str(),
            "xterm"
                | "xterm-256color"
                | "screen"
                | "screen-256color"
                | "tmux"
                | "tmux-256color"
                | "rxvt"
                | "rxvt-unicode"
                | "vt100"
                | "vt220"
                | "linux"
        )
    } else {
        false
    }
}

/// Gets the current mouse cursor position (if supported).
///
/// Note: This may not work on all terminals.
///
/// # Returns
/// * `Result<(u16, u16), napi::Error>` - Column and row position
#[napi]
pub fn get_mouse_position() -> napi::Result<(u16, u16)> {
    // This is a placeholder - getting mouse position requires
    // terminal-specific escape sequences and is not universally supported
    Err(napi::Error::from_reason(
        "get_mouse_position is not implemented - terminal support varies".to_string(),
    ))
}

// Helper functions

fn mouse_event_to_info(event: MouseEvent) -> MouseEventInfo {
    let (event_type, button) = match event.kind {
        MouseEventKind::Down(btn) => {
            let btn_str = mouse_button_to_string(&btn);
            ("down".to_string(), btn_str)
        }
        MouseEventKind::Up(btn) => {
            let btn_str = mouse_button_to_string(&btn);
            ("up".to_string(), btn_str)
        }
        MouseEventKind::Drag(btn) => {
            let btn_str = mouse_button_to_string(&btn);
            ("drag".to_string(), btn_str)
        }
        MouseEventKind::Moved => ("move".to_string(), "none".to_string()),
        MouseEventKind::ScrollDown => ("scrollDown".to_string(), "none".to_string()),
        MouseEventKind::ScrollUp => ("scrollUp".to_string(), "none".to_string()),
        MouseEventKind::ScrollLeft => ("scrollLeft".to_string(), "none".to_string()),
        MouseEventKind::ScrollRight => ("scrollRight".to_string(), "none".to_string()),
    };

    MouseEventInfo {
        event_type,
        button,
        column: event.column,
        row: event.row,
        ctrl: event.modifiers.contains(event::KeyModifiers::CONTROL),
        alt: event.modifiers.contains(event::KeyModifiers::ALT),
        shift: event.modifiers.contains(event::KeyModifiers::SHIFT),
    }
}

fn mouse_button_to_string(button: &MouseButton) -> String {
    match button {
        MouseButton::Left => "left".to_string(),
        MouseButton::Right => "right".to_string(),
        MouseButton::Middle => "middle".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mouse_supported() {
        // Just check it returns a bool
        let _result = is_mouse_supported();
    }

    #[test]
    fn test_mouse_event_listener_creation() {
        let _listener = MouseEventListener::new();
        // Should be able to create without error
    }

    #[test]
    fn test_mouse_button_to_string() {
        assert_eq!(mouse_button_to_string(&MouseButton::Left), "left");
        assert_eq!(mouse_button_to_string(&MouseButton::Right), "right");
        assert_eq!(mouse_button_to_string(&MouseButton::Middle), "middle");
    }

    #[test]
    fn test_get_mouse_position() {
        // Should return error as not implemented
        let result = get_mouse_position();
        assert!(result.is_err());
    }
}
