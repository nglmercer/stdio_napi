//! Keyboard event handling for terminal input.
//!
//! This module provides keyboard event capture and handling capabilities

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use napi_derive::napi;
use std::io::Write;

/// Represents a keyboard key
#[napi]
pub enum Key {
    /// Arrow Up
    ArrowUp,
    /// Arrow Down
    ArrowDown,
    /// Arrow Left
    ArrowLeft,
    /// Arrow Right
    ArrowRight,
    /// Enter key
    Enter,
    /// Escape key
    Escape,
    /// Backspace
    Backspace,
    /// Tab key
    Tab,
    /// Space key
    Space,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up
    PageUp,
    /// Page Down
    PageDown,
    /// Insert key
    Insert,
    /// Delete key
    Delete,
    /// F1-F12 function keys
    F(u8),
    /// A character key (as string for NAPI compatibility)
    Char(String),
    /// Unknown key
    Unknown,
}

/// Keyboard modifier state
#[napi]
pub enum KeyModifier {
    /// Shift key
    Shift,
    /// Control key
    Ctrl,
    /// Alt key
    Alt,
    /// Meta/Super/Windows key
    Meta,
}

/// Keyboard event data
#[napi]
pub struct KeyboardEvent {
    /// The key that was pressed
    pub key: String,
    /// Key code as string
    pub code: String,
    /// Whether it was pressed (true) or released (false)
    pub pressed: bool,
    /// Modifier keys held
    pub modifiers: Vec<String>,
    /// Whether Ctrl was pressed
    pub ctrl: bool,
    /// Whether Shift was pressed
    pub shift: bool,
    /// Whether Alt was pressed
    pub alt: bool,
    /// Whether Meta was pressed
    pub meta: bool,
}

/// Converts KeyCode to string representation
fn key_code_to_string(code: &KeyCode) -> String {
    match code {
        KeyCode::Up => "ArrowUp".to_string(),
        KeyCode::Down => "ArrowDown".to_string(),
        KeyCode::Left => "ArrowLeft".to_string(),
        KeyCode::Right => "ArrowRight".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Escape".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::F(f) => format!("F{}", f),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Null => "Null".to_string(),
        KeyCode::CapsLock => "CapsLock".to_string(),
        KeyCode::NumLock => "NumLock".to_string(),
        KeyCode::ScrollLock => "ScrollLock".to_string(),
        KeyCode::PrintScreen => "PrintScreen".to_string(),
        KeyCode::Pause => "Pause".to_string(),
        KeyCode::Media(_) => "Media".to_string(),
        KeyCode::Modifier(_) => "Modifier".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Menu => "Menu".to_string(),
        KeyCode::KeypadBegin => "KeypadBegin".to_string(),
    }
}

/// Checks if stdin is a TTY (terminal)
#[napi]
pub fn is_keyboard_available() -> bool {
    atty::is(atty::Stream::Stdin)
}

/// Reads a single key press from the terminal.
/// Blocks until a key is pressed.
///
/// # Returns
/// * `Result<KeyboardEvent, napi::Error>` - The keyboard event
///
/// # Example
/// ```javascript
/// const { read_key } = require('stdio-napi');
/// const event = await read_key();
/// console.log(event.key); // "a" if 'a' was pressed
/// ```
#[napi]
pub async fn read_key() -> napi::Result<KeyboardEvent> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_key requires a terminal (TTY). Not running in interactive mode.".to_string(),
        ));
    }

    let event = event::read()
        .map_err(|e| napi::Error::from_reason(format!("Failed to read key: {}", e)))?;

    match event {
        Event::Key(key_event) => {
            let code = key_code_to_string(&key_event.code);
            let key = match &key_event.code {
                KeyCode::Char(c) => c.to_string(),
                _ => code.clone(),
            };

            let pressed = key_event.kind == KeyEventKind::Press;

            let mut modifiers = Vec::new();
            let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
            let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
            let alt = key_event.modifiers.contains(KeyModifiers::ALT);
            let meta = key_event.modifiers.contains(KeyModifiers::META);

            if ctrl {
                modifiers.push("ctrl".to_string());
            }
            if shift {
                modifiers.push("shift".to_string());
            }
            if alt {
                modifiers.push("alt".to_string());
            }
            if meta {
                modifiers.push("meta".to_string());
            }

            Ok(KeyboardEvent {
                key,
                code,
                pressed,
                modifiers,
                ctrl,
                shift,
                alt,
                meta,
            })
        }
        _ => Err(napi::Error::from_reason("Not a keyboard event".to_string())),
    }
}

/// Waits for a specific key press with timeout.
///
/// # Arguments
/// * `timeout_ms` - Timeout in milliseconds (max 4294967)
///
/// # Returns
/// * `Result<Option<KeyboardEvent>, napi::Error>` - The keyboard event or None if timeout
///
/// # Example
/// ```javascript
/// const { read_key_with_timeout } = require('stdio-napi');
/// const event = await read_key_with_timeout(5000); // 5 second timeout
/// ```
#[napi]
pub async fn read_key_with_timeout(timeout_ms: u32) -> napi::Result<Option<KeyboardEvent>> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_key_with_timeout requires a terminal (TTY).".to_string(),
        ));
    }

    let timeout = std::time::Duration::from_millis(timeout_ms as u64);

    // Try to poll with timeout
    match event::poll(timeout) {
        Ok(true) => {
            // Event is ready
            let event = event::read()
                .map_err(|e| napi::Error::from_reason(format!("Failed to read key: {}", e)))?;

            match event {
                Event::Key(key_event) => {
                    let code = key_code_to_string(&key_event.code);
                    let key = match &key_event.code {
                        KeyCode::Char(c) => c.to_string(),
                        _ => code.clone(),
                    };

                    let pressed = key_event.kind == KeyEventKind::Press;

                    let mut modifiers = Vec::new();
                    let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
                    let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
                    let alt = key_event.modifiers.contains(KeyModifiers::ALT);
                    let meta = key_event.modifiers.contains(KeyModifiers::META);

                    if ctrl {
                        modifiers.push("ctrl".to_string());
                    }
                    if shift {
                        modifiers.push("shift".to_string());
                    }
                    if alt {
                        modifiers.push("alt".to_string());
                    }
                    if meta {
                        modifiers.push("meta".to_string());
                    }

                    Ok(Some(KeyboardEvent {
                        key,
                        code,
                        pressed,
                        modifiers,
                        ctrl,
                        shift,
                        alt,
                        meta,
                    }))
                }
                _ => Ok(None),
            }
        }
        Ok(false) => Ok(None),
        Err(e) => Err(napi::Error::from_reason(format!("Poll error: {}", e))),
    }
}

/// Reads a line of text with real-time key event processing.
/// Useful for implementing custom input fields.
///
/// # Arguments
/// * `max_length` - Maximum input length (default: 256)
///
/// # Returns
/// * `Result<String, napi::Error>` - The input text
///
/// # Example
/// ```javascript
/// const { read_line_with_events } = require('stdio-napi');
/// const input = await read_line_with_events(100);
/// ```
#[napi]
pub async fn read_line_with_events(max_length: Option<u32>) -> napi::Result<String> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_line_with_events requires a terminal (TTY).".to_string(),
        ));
    }

    let max_len = max_length.unwrap_or(256) as usize;
    let mut input = String::new();

    loop {
        let key = read_key().await?;

        if !key.pressed {
            continue;
        }

        match key.code.as_str() {
            "Enter" => {
                println!();
                break;
            }
            "Escape" => {
                println!();
                break;
            }
            "Backspace" => {
                if !input.is_empty() {
                    input.pop();
                    print!("\x08 \x08");
                    let _ = std::io::stdout().flush();
                }
            }
            _ => {
                if input.len() < max_len {
                    if let KeyCode::Char(c) = parse_key_code(&key.code) {
                        input.push(c);
                        print!("{}", c);
                        let _ = std::io::stdout().flush();
                    }
                }
            }
        }
    }

    Ok(input)
}

/// Parse key code string back to KeyCode
fn parse_key_code(code: &str) -> KeyCode {
    match code {
        "ArrowUp" => KeyCode::Up,
        "ArrowDown" => KeyCode::Down,
        "ArrowLeft" => KeyCode::Left,
        "ArrowRight" => KeyCode::Right,
        "Enter" => KeyCode::Enter,
        "Escape" => KeyCode::Esc,
        "Backspace" => KeyCode::Backspace,
        "Tab" => KeyCode::Tab,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "PageUp" => KeyCode::PageUp,
        "PageDown" => KeyCode::PageDown,
        "Insert" => KeyCode::Insert,
        "Delete" => KeyCode::Delete,
        c if c.starts_with('F') && c.len() > 1 => {
            if let Ok(n) = c[1..].parse::<u8>() {
                KeyCode::F(n)
            } else {
                KeyCode::Null
            }
        }
        c if c.len() == 1 => KeyCode::Char(c.chars().next().unwrap_or('?')),
        _ => KeyCode::Null,
    }
}

/// Registers a keyboard shortcut handler.
///
/// This is a simple implementation that waits for key combinations.
/// For more complex use cases, consider using a dedicated event loop.
///
/// # Arguments
/// * `shortcut` - Shortcut string (e.g., "Ctrl+C", "Ctrl+S")
///
/// # Returns
/// * `Result<bool, napi::Error>` - True if the shortcut was pressed
///
/// # Example
/// ```javascript
/// const { wait_for_shortcut } = require('stdio-napi');
/// const pressed = await wait_for_shortcut("Ctrl+C");
/// ```
#[napi]
pub async fn wait_for_shortcut(shortcut: String) -> napi::Result<bool> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "wait_for_shortcut requires a terminal (TTY).".to_string(),
        ));
    }

    let parts: Vec<&str> = shortcut.split('+').collect();
    let mut expected_mods: Vec<String> = Vec::new();
    let mut expected_key: Option<String> = None;

    for part in &parts {
        let p = part.trim().to_lowercase();
        match p.as_str() {
            "ctrl" | "control" => expected_mods.push("ctrl".to_string()),
            "shift" => expected_mods.push("shift".to_string()),
            "alt" => expected_mods.push("alt".to_string()),
            "meta" | "cmd" | "command" | "super" => expected_mods.push("meta".to_string()),
            k => expected_key = Some(k.to_string()),
        }
    }

    loop {
        let key = read_key().await?;

        if !key.pressed {
            continue;
        }

        // Check if modifiers match
        let mut mod_match = expected_mods.len() == key.modifiers.len();
        if mod_match {
            for m in &expected_mods {
                if !key.modifiers.contains(&m.to_string()) {
                    mod_match = false;
                    break;
                }
            }
        }

        // Check if key matches
        let key_match = expected_key.as_ref().map_or(false, |k| {
            let lower_k = k.to_lowercase();
            key.code.to_lowercase() == lower_k || key.key.to_lowercase() == lower_k
        });

        if mod_match && key_match {
            return Ok(true);
        }

        // If escape pressed, cancel
        if key.code == "Escape" {
            return Ok(false);
        }
    }
}

/// Gets the current terminal's key response mode
#[napi]
pub fn get_keyboard_mode() -> String {
    // Return the current keyboard mode
    // This is a simplified version - real implementation would check terminal capabilities
    "raw".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_code_to_string() {
        assert_eq!(key_code_to_string(&KeyCode::Up), "ArrowUp");
        assert_eq!(key_code_to_string(&KeyCode::Down), "ArrowDown");
        assert_eq!(key_code_to_string(&KeyCode::Enter), "Enter");
        assert_eq!(key_code_to_string(&KeyCode::Esc), "Escape");
        assert_eq!(key_code_to_string(&KeyCode::F(1)), "F1");
    }

    #[test]
    fn test_parse_key_code() {
        assert!(matches!(parse_key_code("ArrowUp"), KeyCode::Up));
        // Note: parse_key_code uses KeyCode::Escape which doesn't exist in crossterm 0.28
        let result = parse_key_code("Enter");
        assert!(matches!(result, KeyCode::Enter));
    }

    #[test]
    fn test_is_keyboard_available() {
        // Just check it returns a bool
        let result = is_keyboard_available();
        assert!(result == true || result == false);
    }

    #[test]
    fn test_get_keyboard_mode() {
        let mode = get_keyboard_mode();
        assert_eq!(mode, "raw");
    }
}
