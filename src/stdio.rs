use napi_derive::napi;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use colored::*;
use crossterm::execute;

/// Writes text to stdout without a newline.
/// 
/// # Arguments
/// * `text` - The text to write to stdout
/// 
/// # Example
/// ```javascript
/// const { print_stdout } = require('stdio-napi');
/// print_stdout("Hello, World!");
/// ```
#[napi]
pub fn print_stdout(text: String) {
    print!("{}", text);
    let _ = io::stdout().flush();
}

/// Writes text to stderr without a newline.
/// 
/// # Arguments
/// * `text` - The text to write to stderr
/// 
/// # Example
/// ```javascript
/// const { print_stderr } = require('stdio-napi');
/// print_stderr("Error occurred!");
/// ```
#[napi]
pub fn print_stderr(text: String) {
    eprint!("{}", text);
    let _ = io::stderr().flush();
}

/// Prints success message in green bold text to stdout.
/// 
/// # Arguments
/// * `text` - The success message to display
/// 
/// # Example
/// ```javascript
/// const { print_success } = require('stdio-napi');
/// print_success("Operation completed successfully!");
/// ```
#[napi]
pub fn print_success(text: String) {
    println!("{}", text.green().bold());
}

/// Prints error message in red bold text to stderr.
/// 
/// # Arguments
/// * `text` - The error message to display
/// 
/// # Example
/// ```javascript
/// const { print_error } = require('stdio-napi');
/// print_error("An error occurred!");
/// ```
#[napi]
pub fn print_error(text: String) {
    eprintln!("{}", text.red().bold());
}

/// Prints warning message in yellow text to stdout.
/// 
/// # Arguments
/// * `text` - The warning message to display
/// 
/// # Example
/// ```javascript
/// const { print_warning } = require('stdio-napi');
/// print_warning("This is a warning!");
/// ```
#[napi]
pub fn print_warning(text: String) {
    println!("{}", text.yellow());
}

/// Prints info message in blue text to stdout.
/// 
/// # Arguments
/// * `text` - The info message to display
/// 
/// # Example
/// ```javascript
/// const { print_info } = require('stdio-napi');
/// print_info("Here is some information.");
/// ```
#[napi]
pub fn print_info(text: String) {
    println!("{}", text.blue());
}

/// Asynchronously reads a single line from stdin.
/// 
/// Reads input from stdin and returns the trimmed line as a string.
/// 
/// # Returns
/// * `Result<String, napi::Error>` - The trimmed line read from stdin
/// 
/// # Example
/// ```javascript
/// const { read_line } = require('stdio-napi');
/// const input = await read_line();
/// console.log("You entered:", input);
/// ```
#[napi]
pub async fn read_line() -> napi::Result<String> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    
    reader.read_line(&mut line).await.map_err(|e| {
        napi::Error::from_reason(format!("Failed to read line: {}", e))
    })?;
    
    Ok(line.trim().to_string())
}

/// Displays a prompt message and reads user input.
/// 
/// Prints the message followed by ": " and waits for user input.
/// 
/// # Arguments
/// * `message` - The prompt message to display
/// 
/// # Returns
/// * `Result<String, napi::Error>` - The user's input
/// 
/// # Example
/// ```javascript
/// const { prompt } = require('stdio-napi');
/// const name = await prompt("Enter your name");
/// ```
#[napi]
pub async fn prompt(message: String) -> napi::Result<String> {
    print!("{}: ", message.cyan());
    let _ = io::stdout().flush();
    
    read_line().await
}

/// Displays a yes/no confirmation prompt.
/// 
/// # Arguments
/// * `message` - The confirmation message to display
/// * `default` - Optional default value (true for yes, false for no)
/// 
/// # Returns
/// * `Result<bool, napi::Error>` - true for yes, false for no
/// 
/// # Example
/// ```javascript
/// const { confirm } = require('stdio-napi');
/// const result = await confirm("Continue?", true);
/// ```
#[napi]
pub async fn confirm(message: String, default: Option<bool>) -> napi::Result<bool> {
    let def = default.unwrap_or(true);
    let suffix = if def { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", message.cyan(), suffix);
    let _ = io::stdout().flush();
    
    let input = read_line().await?;
    let input = input.trim().to_lowercase();
    
    if input.is_empty() {
        return Ok(def);
    }
    
    if input == "y" || input == "yes" {
        return Ok(true);
    }
    
    if input == "n" || input == "no" {
        return Ok(false);
    }
    
    Ok(def)
}

/// Prints a progress bar to stdout.
/// 
/// # Arguments
/// * `current` - The current progress value
/// * `total` - The total value (100%)
/// * `width` - Optional width of the progress bar (default: 20 characters)
/// 
/// # Example
/// ```javascript
/// const { print_progress } = require('stdio-napi');
/// print_progress(50, 100, 20); // Shows 50% progress
/// ```
#[napi]
pub fn print_progress(current: u32, total: u32, width: Option<u32>) {
    let w = width.unwrap_or(20) as usize;
    let total_f = total as f32;
    let current_f = current as f32;
    let percent = (current_f / total_f).min(1.0);
    let filled = (percent * w as f32) as usize;
    let empty = w - filled;
    
    let bar = format!(
        "\r[{}{}] {:>3}%",
        "=".repeat(filled),
        " ".repeat(empty),
        (percent * 100.0) as u32
    );
    
    print!("{}", bar);
    let _ = io::stdout().flush();
    
    if current >= total {
        println!();
    }
}

/// Returns a spinner animation frame.
/// 
/// # Arguments
/// * `frame` - The frame number (0-9)
/// 
/// # Returns
/// * `String` - The spinner character for the given frame
/// 
/// # Example
/// ```javascript
/// const { get_spinner_frame } = require('stdio-napi');
/// const frame = get_spinner_frame(0); // Returns "⠋"
/// ```
#[napi]
pub fn get_spinner_frame(frame: u32) -> String {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    frames[(frame as usize) % frames.len()].to_string()
}

/// Reads a password from stdin with optional character masking.
/// 
/// Enables raw mode for secure password input.
/// Note: Requires a TTY (terminal). If not running in a terminal, returns an error.
/// 
/// # Arguments
/// * `mask` - Optional character to display for each typed character
/// 
/// # Returns
/// * `Result<String, napi::Error>` - The entered password
/// 
/// # Example
/// ```javascript
/// const { read_password } = require('stdio-napi');
/// const password = await read_password("*"); // Shows * for each character
/// ```
#[napi]
pub async fn read_password(mask: Option<String>) -> napi::Result<String> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal;
    
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_password requires a terminal (TTY). Not running in interactive mode.".to_string()
        ));
    }
    
    let mask_char = mask.as_deref().and_then(|s| s.chars().next());
    let mut password = String::new();
    
    terminal::enable_raw_mode().map_err(|e| {
        napi::Error::from_reason(format!("Failed to enable raw mode: {}", e))
    })?;
    
    let result = async {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).map_err(|e| {
                napi::Error::from_reason(format!("Poll error: {}", e))
            })? {
                if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().map_err(|e| {
                    napi::Error::from_reason(format!("Read error: {}", e))
                })? {
                    match code {
                        KeyCode::Enter => {
                            println!();
                            break;
                        }
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                            return Err(napi::Error::from_reason("Interrupted".to_string()));
                        }
                        KeyCode::Char(c) => {
                            password.push(c);
                            if let Some(m) = mask_char {
                                print!("{}", m);
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Backspace => {
                            if !password.is_empty() {
                                password.pop();
                                if mask_char.is_some() {
                                    print!("\x08 \x08");
                                    let _ = io::stdout().flush();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(password)
    }.await;
    
    let _ = terminal::disable_raw_mode();
    result
}

/// Displays an interactive selection menu in the terminal.
/// 
/// Uses arrow keys to navigate and Enter to select.
/// Note: Requires a TTY (terminal). If not running in a terminal, returns an error.
/// 
/// # Arguments
/// * `message` - The menu prompt message
/// * `options` - Vector of options to display
/// 
/// # Returns
/// * `Result<u32, napi::Error>` - The index of the selected option
/// 
/// # Example
/// ```javascript
/// const { select_menu } = require('stdio-napi');
/// const index = await select_menu("Choose an option", ["Option 1", "Option 2", "Option 3"]);
/// ```
#[napi]
pub async fn select_menu(message: String, options: Vec<String>) -> napi::Result<u32> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent};
    use crossterm::terminal;
    use crossterm::cursor;
    
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "select_menu requires a terminal (TTY). Not running in interactive mode.".to_string()
        ));
    }
    
    if options.is_empty() {
        return Err(napi::Error::from_reason("Options cannot be empty".to_string()));
    }
    
    let mut selected = 0;
    
    println!("{}: ", message.cyan());
    
    terminal::enable_raw_mode().map_err(|e| {
        napi::Error::from_reason(format!("Failed to enable raw mode: {}", e))
    })?;
    
    let result = async {
        loop {
            // Draw options
            for (i, opt) in options.iter().enumerate() {
                if i == selected {
                    println!(" > {}", opt.green().bold());
                } else {
                    println!("   {}", opt);
                }
            }
            
            // Wait for key
            let code = loop {
                if event::poll(std::time::Duration::from_millis(100)).map_err(|e| {
                    napi::Error::from_reason(format!("Poll error: {}", e))
                })? {
                    if let Event::Key(KeyEvent { code, .. }) = event::read().map_err(|e| {
                        napi::Error::from_reason(format!("Read error: {}", e))
                    })? {
                        break code;
                    }
                }
            };
            
            // Clear drawn options
            execute!(io::stdout(), cursor::MoveUp(options.len() as u16)).map_err(|e| {
                napi::Error::from_reason(format!("Cursor error: {}", e))
            })?;
            
            match code {
                KeyCode::Up => {
                    selected = if selected == 0 { options.len() - 1 } else { selected - 1 };
                }
                KeyCode::Down => {
                    selected = (selected + 1) % options.len();
                }
                KeyCode::Enter => {
                    // Redraw one last time with selection and then move cursor past the list
                    for (i, opt) in options.iter().enumerate() {
                        if i == selected {
                            println!(" > {}", opt.green().bold());
                        } else {
                            println!("   {}", opt);
                        }
                    }
                    break;
                }
                KeyCode::Char('c') if event::poll(std::time::Duration::from_millis(0)).is_ok() => {
                    // Simple ctrl-c handle could be added better but this is fine for now
                }
                _ => {}
            }
        }
        Ok(selected as u32)
    }.await;
    
    let _ = terminal::disable_raw_mode();
    result
}

/// Reads multiple lines of input until the delimiter is entered.
/// 
/// # Arguments
/// * `delimiter` - Optional delimiter string (default: "EOF")
/// 
/// # Returns
/// * `Result<String, napi::Error>` - All lines joined with newlines
/// 
/// # Example
/// ```javascript
/// const { read_multiline } = require('stdio-napi');
/// const text = await read_multiline("DONE");
/// ```
#[napi]
pub async fn read_multiline(delimiter: Option<String>) -> napi::Result<String> {
    let delim = delimiter.unwrap_or_else(|| "EOF".to_string());
    println!("(Enter '{}' on a new line to finish)", delim.yellow());
    
    let mut lines = Vec::new();
    loop {
        let line = read_line().await?;
        if line == delim {
            break;
        }
        lines.push(line);
    }
    
    Ok(lines.join("\n"))
}

#[napi]
pub struct BufferedReader {
    reader: std::sync::Arc<tokio::sync::Mutex<BufReader<tokio::io::Stdin>>>,
}

#[napi]
impl BufferedReader {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            reader: std::sync::Arc::new(tokio::sync::Mutex::new(BufReader::new(tokio::io::stdin()))),
        }
    }

    #[napi]
    pub async fn read_line(&self) -> napi::Result<Option<String>> {
        let mut reader = self.reader.lock().await;
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await.map_err(|e| {
            napi::Error::from_reason(format!("Failed to read line: {}", e))
        })?;
        
        if bytes_read == 0 {
            return Ok(None);
        }
        
        Ok(Some(line))
    }

    #[napi]
    pub async fn read_until(&self, delimiter: String) -> napi::Result<Option<String>> {
        let mut reader = self.reader.lock().await;
        let mut buffer = Vec::new();
        let delim_bytes = delimiter.as_bytes();
        if delim_bytes.is_empty() {
            return Err(napi::Error::from_reason("Delimiter cannot be empty".to_string()));
        }
        
        let bytes_read = reader.read_until(delim_bytes[0], &mut buffer).await.map_err(|e| {
            napi::Error::from_reason(format!("Failed to read until delimiter: {}", e))
        })?;
        
        if bytes_read == 0 {
            return Ok(None);
        }
        
        Ok(Some(String::from_utf8_lossy(&buffer).to_string()))
    }

    #[napi]
    pub async fn next(&self) -> napi::Result<Option<String>> {
        self.read_line().await
    }
    
    /// Read with configurable buffer size
    #[napi]
    pub async fn read(&self, size: Option<u32>) -> napi::Result<Option<String>> {
        let mut reader = self.reader.lock().await;
        let buffer_size = size.unwrap_or(8192) as usize;
        let mut buffer = vec![0u8; buffer_size];
        use tokio::io::AsyncReadExt;
        let bytes_read = reader.read(&mut buffer).await.map_err(|e| {
            napi::Error::from_reason(format!("Failed to read: {}", e))
        })?;
        
        if bytes_read == 0 {
            return Ok(None);
        }
        
        buffer.truncate(bytes_read);
        Ok(Some(String::from_utf8_lossy(&buffer).to_string()))
    }
}

impl Default for BufferedReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spinner_frame() {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        for (i, expected) in frames.iter().enumerate() {
            assert_eq!(get_spinner_frame(i as u32), *expected);
        }
    }

    #[test]
    fn test_get_spinner_frame_wraps() {
        assert_eq!(get_spinner_frame(0), get_spinner_frame(10));
        assert_eq!(get_spinner_frame(1), get_spinner_frame(11));
    }

    #[test]
    fn test_print_progress_zero() {
        // Should not panic
        print_progress(0, 100, Some(20));
    }

    #[test]
    fn test_print_progress_full() {
        // Should not panic
        print_progress(100, 100, Some(20));
    }

    #[test]
    fn test_print_progress_over() {
        // Should not panic (clamped to 100%)
        print_progress(150, 100, Some(20));
    }

    #[test]
    fn test_print_progress_custom_width() {
        print_progress(50, 100, Some(10));
        print_progress(50, 100, Some(50));
    }
}
