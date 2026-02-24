use colored::*;
use crossterm::execute;
use napi_derive::napi;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};

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

    reader
        .read_line(&mut line)
        .await
        .map_err(|e| napi::Error::from_reason(format!("Failed to read line: {}", e)))?;

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
/// * `value` - Optional default value (true for yes, false for no)
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
pub async fn confirm(message: String, value: Option<bool>) -> napi::Result<bool> {
    let def = value.unwrap_or(true);
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
    use crossterm::cursor::MoveToColumn;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal;
    use crossterm::QueueableCommand;

    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "read_password requires a terminal (TTY). Not running in interactive mode.".to_string(),
        ));
    }

    let mask_char = mask.as_deref().and_then(|s| s.chars().next());
    let mut password = String::new();

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

    let result = async {
        loop {
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
            {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()
                    .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?
                {
                    match code {
                        KeyCode::Enter => {
                            // Move to beginning of line and clear it
                            let _ = io::stdout().queue(MoveToColumn(0));
                            let _ = io::stdout().queue(crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::UntilNewLine,
                            ));
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
    }
    .await;

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
    use crossterm::cursor;
    use crossterm::event::{self, Event, KeyCode, KeyEvent};
    use crossterm::terminal;

    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "select_menu requires a terminal (TTY). Not running in interactive mode.".to_string(),
        ));
    }

    if options.is_empty() {
        return Err(napi::Error::from_reason(
            "Options cannot be empty".to_string(),
        ));
    }

    let mut selected = 0;

    println!("{}: ", message.cyan());

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

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
                if event::poll(std::time::Duration::from_millis(100))
                    .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
                {
                    if let Event::Key(KeyEvent { code, .. }) = event::read()
                        .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?
                    {
                        break code;
                    }
                }
            };

            // Clear drawn options
            execute!(io::stdout(), cursor::MoveUp(options.len() as u16))
                .map_err(|e| napi::Error::from_reason(format!("Cursor error: {}", e)))?;

            match code {
                KeyCode::Up => {
                    selected = if selected == 0 {
                        options.len() - 1
                    } else {
                        selected - 1
                    };
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
    }
    .await;

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

/// Reads a line with interactive editing features.
///
/// Enables raw mode for full line editing capabilities:
/// - Arrow keys for cursor navigation (left/right)
/// - Arrow keys for history navigation (up/down)
/// - Backspace/Delete for editing
/// - Home/End for line start/end
/// - Ctrl+C to cancel
///
/// Note: Requires a TTY (terminal). If not running in a terminal, falls back to simple read_line.
///
/// # Arguments
/// * `prompt` - Optional prompt string to display
/// * `history` - Optional array of history entries for up/down navigation
///
/// # Returns
/// * `Result<String, napi::Error>` - The entered line (trimmed)
///
/// # Example
/// ```javascript
/// const { read_line_interactive } = require('stdio-napi');
/// const input = await read_line_interactive("> ", ["previous command 1", "previous command 2"]);
/// ```
#[napi]
pub async fn read_line_interactive(
    prompt: Option<String>,
    history: Option<Vec<String>>,
) -> napi::Result<String> {
    use crossterm::cursor::{MoveLeft, MoveRight, MoveToColumn};
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal;
    use crossterm::QueueableCommand;

    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        // Fallback to simple read_line for non-TTY
        if let Some(p) = &prompt {
            print!("{}", p);
            let _ = io::stdout().flush();
        }
        return read_line().await;
    }

    let prompt_str = prompt.unwrap_or_default();
    let mut history = history.unwrap_or_default();
    // Add empty string at the end to represent "current" position
    history.push(String::new());
    let mut history_index = history.len() - 1;

    let mut input = String::new();
    let mut cursor_pos = 0; // Cursor position within input string (in characters)

    // Print prompt
    print!("{}", prompt_str);
    let _ = io::stdout().flush();

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

    let result = async {
        loop {
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
            {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()
                    .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?
                {
                    match code {
                        KeyCode::Enter => {
                            // Move to beginning of line and clear it before exiting
                            let _ = io::stdout().queue(MoveToColumn(0));
                            let _ = io::stdout().queue(crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::UntilNewLine,
                            ));
                            println!();
                            break;
                        }
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                            // Move to beginning of line before exiting
                            let _ = io::stdout().queue(MoveToColumn(0));
                            println!("^C");
                            return Ok(String::new());
                        }
                        KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                            // Move to beginning of line before exiting
                            let _ = io::stdout().queue(MoveToColumn(0));
                            println!("^D");
                            return Ok(String::new());
                        }
                        KeyCode::Char(c) => {
                            // Insert character at cursor position
                            if cursor_pos >= input.len() {
                                input.push(c);
                            } else {
                                input.insert(cursor_pos, c);
                            }
                            cursor_pos += c.len_utf8();

                            // Redraw from cursor
                            let suffix: String = input.chars().skip(cursor_pos).collect();
                            print!("{}", c);
                            if !suffix.is_empty() {
                                print!("{}", suffix);
                                // Move cursor back to correct position
                                let _ = io::stdout().queue(MoveLeft(suffix.chars().count() as u16));
                            }
                            let _ = io::stdout().flush();
                        }
                        KeyCode::Backspace => {
                            if cursor_pos > 0 && !input.is_empty() {
                                // Find the character before cursor
                                let char_pos =
                                    input.char_indices().nth(cursor_pos - 1).map(|(i, _)| i);
                                if let Some(pos) = char_pos {
                                    input.remove(pos);
                                    cursor_pos -= 1;

                                    // Redraw
                                    let _ = io::stdout().queue(MoveLeft(1));
                                    let suffix: String = input.chars().skip(cursor_pos).collect();
                                    print!("{}", suffix);
                                    print!(" "); // Clear the deleted character
                                    let move_back = suffix.chars().count() + 1;
                                    let _ = io::stdout().queue(MoveLeft(move_back as u16));
                                    let _ = io::stdout().flush();
                                }
                            }
                        }
                        KeyCode::Delete => {
                            if cursor_pos < input.len() {
                                // Delete character at cursor
                                let char_pos = input.char_indices().nth(cursor_pos).map(|(i, _)| i);
                                if let Some(pos) = char_pos {
                                    input.remove(pos);

                                    // Redraw
                                    let suffix: String = input.chars().skip(cursor_pos).collect();
                                    print!("{}", suffix);
                                    print!(" "); // Clear the deleted character
                                    let move_back = suffix.chars().count() + 1;
                                    let _ = io::stdout().queue(MoveLeft(move_back as u16));
                                    let _ = io::stdout().flush();
                                }
                            }
                        }
                        KeyCode::Left => {
                            if cursor_pos > 0 {
                                cursor_pos -= 1;
                                let _ = io::stdout().queue(MoveLeft(1));
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Right => {
                            if cursor_pos < input.len() {
                                cursor_pos += 1;
                                let _ = io::stdout().queue(MoveRight(1));
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Home => {
                            if cursor_pos > 0 {
                                let _ = io::stdout().queue(MoveLeft(cursor_pos as u16));
                                cursor_pos = 0;
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::End => {
                            let chars_count = input.chars().count();
                            if cursor_pos < chars_count {
                                let move_right = (chars_count - cursor_pos) as u16;
                                let _ = io::stdout().queue(MoveRight(move_right));
                                cursor_pos = chars_count;
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Up => {
                            if history_index > 0 {
                                // Save current input
                                history[history_index] = input.clone();

                                // Move to previous history entry
                                history_index -= 1;
                                input = history[history_index].clone();

                                // Clear line and redraw
                                let _ = io::stdout().queue(MoveToColumn(0));
                                print!("{}{}", prompt_str, input);
                                // Clear rest of line
                                let old_len = history[history_index + 1].chars().count();
                                let new_len = input.chars().count();
                                if old_len > new_len {
                                    print!("{}", " ".repeat(old_len - new_len));
                                    let _ =
                                        io::stdout().queue(MoveLeft((old_len - new_len) as u16));
                                }
                                cursor_pos = input.chars().count();
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Down => {
                            if history_index < history.len() - 1 {
                                // Save current input
                                history[history_index] = input.clone();

                                // Move to next history entry
                                history_index += 1;
                                input = history[history_index].clone();

                                // Clear line and redraw
                                let _ = io::stdout().queue(MoveToColumn(0));
                                print!("{}{}", prompt_str, input);
                                // Clear rest of line
                                let old_len = history[history_index - 1].chars().count();
                                let new_len = input.chars().count();
                                if old_len > new_len {
                                    print!("{}", " ".repeat(old_len - new_len));
                                    let _ =
                                        io::stdout().queue(MoveLeft((old_len - new_len) as u16));
                                }
                                cursor_pos = input.chars().count();
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Tab => {
                            // Insert tab as spaces
                            let spaces = "    ";
                            for c in spaces.chars() {
                                if cursor_pos >= input.len() {
                                    input.push(c);
                                } else {
                                    input.insert(cursor_pos, c);
                                }
                                cursor_pos += 1;
                            }
                            print!("{}", spaces);
                            let _ = io::stdout().flush();
                        }
                        KeyCode::Esc => {
                            // Clear current input
                            let chars_count = input.chars().count();
                            if chars_count > 0 {
                                let _ = io::stdout().queue(MoveLeft(chars_count as u16));
                                print!("{}", " ".repeat(chars_count));
                                let _ = io::stdout().queue(MoveLeft(chars_count as u16));
                                let _ = io::stdout().flush();
                            }
                            input.clear();
                            cursor_pos = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(input.trim().to_string())
    }
    .await;

    let _ = terminal::disable_raw_mode();
    result
}

#[napi]
pub struct BufferedReader {
    reader: std::sync::Arc<tokio::sync::Mutex<BufReader<tokio::io::Stdin>>>,
    is_raw_mode: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[napi]
impl BufferedReader {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            reader: std::sync::Arc::new(tokio::sync::Mutex::new(
                BufReader::new(tokio::io::stdin()),
            )),
            is_raw_mode: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Enables raw mode for terminal input.
    /// This allows reading individual key presses including arrow keys.
    /// Note: Requires a TTY (terminal). Returns error if not running in a terminal.
    #[napi]
    pub fn enable_raw_mode(&self) -> napi::Result<()> {
        use crossterm::terminal;

        // Check if stdin is a TTY
        if !atty::is(atty::Stream::Stdin) {
            return Err(napi::Error::from_reason(
                "enable_raw_mode requires a terminal (TTY). Not running in interactive mode."
                    .to_string(),
            ));
        }

        terminal::enable_raw_mode()
            .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

        self.is_raw_mode
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    /// Disables raw mode and restores normal terminal behavior.
    #[napi]
    pub fn disable_raw_mode(&self) -> napi::Result<()> {
        use crossterm::terminal;

        terminal::disable_raw_mode()
            .map_err(|e| napi::Error::from_reason(format!("Failed to disable raw mode: {}", e)))?;

        self.is_raw_mode
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    /// Reads a single key press and returns the key as a string.
    /// Handles arrow keys, function keys, and special keys.
    /// Requires raw mode to be enabled first.
    #[napi]
    pub async fn read_key(&self) -> napi::Result<Option<String>> {
        use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

        if !self.is_raw_mode.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(napi::Error::from_reason(
                "read_key requires raw mode to be enabled. Call enable_raw_mode() first."
                    .to_string(),
            ));
        }

        // Wait for a key event
        loop {
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
            {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()
                    .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?
                {
                    // Handle Ctrl+C
                    if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                        return Ok(Some("\x03".to_string()));
                    }

                    // Handle Ctrl+D
                    if code == KeyCode::Char('d') && modifiers.contains(KeyModifiers::CONTROL) {
                        return Ok(Some("\x04".to_string()));
                    }

                    match code {
                        KeyCode::Enter => return Ok(Some("\n".to_string())),
                        KeyCode::Backspace => return Ok(Some("\x7f".to_string())),
                        KeyCode::Tab => return Ok(Some("\t".to_string())),
                        KeyCode::Esc => return Ok(Some("\x1b".to_string())),
                        KeyCode::Up => return Ok(Some("\x1b[A".to_string())),
                        KeyCode::Down => return Ok(Some("\x1b[B".to_string())),
                        KeyCode::Right => return Ok(Some("\x1b[C".to_string())),
                        KeyCode::Left => return Ok(Some("\x1b[D".to_string())),
                        KeyCode::Home => return Ok(Some("\x1b[H".to_string())),
                        KeyCode::End => return Ok(Some("\x1b[F".to_string())),
                        KeyCode::PageUp => return Ok(Some("\x1b[5~".to_string())),
                        KeyCode::PageDown => return Ok(Some("\x1b[6~".to_string())),
                        KeyCode::Insert => return Ok(Some("\x1b[2~".to_string())),
                        KeyCode::Delete => return Ok(Some("\x1b[3~".to_string())),
                        KeyCode::Char(c) => return Ok(Some(c.to_string())),
                        KeyCode::F(n) => return Ok(Some(format!("\x1b[{}~", n + 1))),
                        _ => continue,
                    }
                }
            }
        }
    }

    /// Reads a line with full terminal editing support (requires raw mode).
    /// Supports:
    /// - Arrow keys for navigation (left/right)
    /// - Up/Down for command history
    /// - Backspace/Delete for editing
    /// - Home/End to jump to line start/end
    #[napi]
    pub async fn read_line_raw(&self) -> napi::Result<Option<String>> {
        if !self.is_raw_mode.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(napi::Error::from_reason(
                "read_line_raw requires raw mode to be enabled. Call enable_raw_mode() first."
                    .to_string(),
            ));
        }

        use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
        use std::io::Write;

        let mut line = String::new();
        let mut cursor_pos = 0;
        let mut history: Vec<String> = Vec::new();
        let mut history_index: Option<usize> = None;
        let mut current_input = String::new();

        loop {
            // Wait for a key event
            let key = loop {
                if event::poll(std::time::Duration::from_millis(100))
                    .map_err(|e| napi::Error::from_reason(format!("Poll error: {}", e)))?
                {
                    if let Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) = event::read()
                        .map_err(|e| napi::Error::from_reason(format!("Read error: {}", e)))?
                    {
                        break (code, modifiers);
                    }
                }
            };

            match key {
                (KeyCode::Enter, _) => {
                    // User pressed Enter - return the line
                    println!();
                    break;
                }
                (KeyCode::Backspace, _) => {
                    // Handle backspace
                    if cursor_pos > 0 {
                        line.remove(cursor_pos - 1);
                        cursor_pos -= 1;
                        // Redraw line
                        print!("\r\x1b[K");
                        print!("> {}", line);
                        // Move cursor to correct position
                        let remaining = line.len() - cursor_pos;
                        if remaining > 0 {
                            for _ in 0..remaining {
                                print!("\x1b[D");
                            }
                        }
                        let _ = std::io::stdout().flush();
                    }
                }
                (KeyCode::Delete, _) => {
                    // Handle Delete key
                    if cursor_pos < line.len() {
                        line.remove(cursor_pos);
                        // Redraw line
                        print!("\r\x1b[K");
                        print!("> {}", line);
                        // Move cursor to correct position
                        let remaining = line.len() - cursor_pos;
                        if remaining > 0 {
                            for _ in 0..remaining {
                                print!("\x1b[D");
                            }
                        }
                        let _ = std::io::stdout().flush();
                    }
                }
                (KeyCode::Left, _) => {
                    // Move cursor left
                    if cursor_pos > 0 {
                        print!("\x1b[D");
                        cursor_pos -= 1;
                        let _ = std::io::stdout().flush();
                    }
                }
                (KeyCode::Right, _) => {
                    // Move cursor right
                    if cursor_pos < line.len() {
                        print!("\x1b[C");
                        cursor_pos += 1;
                        let _ = std::io::stdout().flush();
                    }
                }
                (KeyCode::Home, _) => {
                    // Move to beginning of line
                    while cursor_pos > 0 {
                        print!("\x1b[D");
                        cursor_pos -= 1;
                    }
                    let _ = std::io::stdout().flush();
                }
                (KeyCode::End, _) => {
                    // Move to end of line
                    while cursor_pos < line.len() {
                        print!("\x1b[C");
                        cursor_pos += 1;
                    }
                    let _ = std::io::stdout().flush();
                }
                (KeyCode::Up, _) => {
                    // History navigation - previous command
                    if !history.is_empty() {
                        let idx = history_index.unwrap_or(history.len());
                        if idx > 0 {
                            let new_idx = idx - 1;
                            // Save current input if at bottom of history
                            if history_index.is_none() {
                                current_input = line.clone();
                            }
                            history_index = Some(new_idx);
                            // Clear current line and show history item
                            line = history[new_idx].clone();
                            cursor_pos = line.len();
                            print!("\r\x1b[K> {}", line);
                            let _ = std::io::stdout().flush();
                        } else if history_index.is_some() {
                            // Go back to current input
                            history_index = None;
                            line = current_input.clone();
                            cursor_pos = line.len();
                            print!("\r\x1b[K> {}", line);
                            let _ = std::io::stdout().flush();
                        }
                    }
                }
                (KeyCode::Down, _) => {
                    // History navigation - next command
                    if let Some(idx) = history_index {
                        if idx < history.len() - 1 {
                            let new_idx = idx + 1;
                            history_index = Some(new_idx);
                            line = history[new_idx].clone();
                            cursor_pos = line.len();
                            print!("\r\x1b[K> {}", line);
                            let _ = std::io::stdout().flush();
                        } else {
                            // Go back to current input
                            history_index = None;
                            line = current_input.clone();
                            cursor_pos = line.len();
                            print!("\r\x1b[K> {}", line);
                            let _ = std::io::stdout().flush();
                        }
                    }
                }
                (KeyCode::Char(c), mods) if !mods.contains(KeyModifiers::CONTROL) => {
                    // Insert character at cursor position
                    line.insert(cursor_pos, c);
                    cursor_pos += 1;
                    // Redraw line
                    print!("\r\x1b[K> {}", line);
                    // Move cursor to correct position
                    let remaining = line.len() - cursor_pos;
                    if remaining > 0 {
                        for _ in 0..remaining {
                            print!("\x1b[D");
                        }
                    }
                    let _ = std::io::stdout().flush();
                }
                (KeyCode::Esc, _) => {
                    // Escape key - ignore
                }
                _ => {}
            }
        }

        // Add to history if non-empty
        if !line.trim().is_empty() {
            history.push(line.clone());
        }

        if line.is_empty() {
            Ok(None)
        } else {
            Ok(Some(line))
        }
    }

    #[napi]
    pub async fn read_line(&self) -> napi::Result<Option<String>> {
        // If raw mode is enabled, use the raw line reader
        if self.is_raw_mode.load(std::sync::atomic::Ordering::SeqCst) {
            return self.read_line_raw().await;
        }

        let mut reader = self.reader.lock().await;
        let mut line = String::new();
        let bytes_read = reader
            .read_line(&mut line)
            .await
            .map_err(|e| napi::Error::from_reason(format!("Failed to read line: {}", e)))?;

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
            return Err(napi::Error::from_reason(
                "Delimiter cannot be empty".to_string(),
            ));
        }

        let bytes_read = reader
            .read_until(delim_bytes[0], &mut buffer)
            .await
            .map_err(|e| {
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
        let bytes_read = reader
            .read(&mut buffer)
            .await
            .map_err(|e| napi::Error::from_reason(format!("Failed to read: {}", e)))?;

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

    /// Test that get_spinner_frame returns correct frames for indices 0-9
    #[test]
    fn test_get_spinner_frame_all_frames() {
        let expected_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        for (i, expected) in expected_frames.iter().enumerate() {
            let frame = get_spinner_frame(i as u32);
            assert_eq!(frame, *expected, "Frame {} should be {}", i, expected);
        }
    }

    /// Test that get_spinner_frame wraps around correctly
    #[test]
    fn test_get_spinner_frame_wrap_around() {
        // Frame 10 should equal frame 0
        assert_eq!(get_spinner_frame(0), get_spinner_frame(10));
        // Frame 11 should equal frame 1
        assert_eq!(get_spinner_frame(1), get_spinner_frame(11));
        // Frame 20 should equal frame 0
        assert_eq!(get_spinner_frame(0), get_spinner_frame(20));
    }

    /// Test that get_spinner_frame handles large frame numbers
    #[test]
    fn test_get_spinner_frame_large_numbers() {
        // 100 % 10 = 0
        assert_eq!(get_spinner_frame(100), get_spinner_frame(0));
        // 123 % 10 = 3
        assert_eq!(get_spinner_frame(123), get_spinner_frame(3));
        // 1000 % 10 = 0
        assert_eq!(get_spinner_frame(1000), get_spinner_frame(0));
    }

    /// Test print_progress with basic inputs
    #[test]
    fn test_print_progress_basic() {
        // This test verifies the function doesn't panic
        print_progress(50, 100, Some(20));
        print_progress(0, 100, Some(20));
        print_progress(100, 100, Some(20));
    }

    /// Test print_progress with default width
    #[test]
    fn test_print_progress_default_width() {
        // Should use default width of 20
        print_progress(50, 100, None);
    }

    /// Test print_progress edge cases
    #[test]
    fn test_print_progress_edge_cases() {
        // Zero progress
        print_progress(0, 100, Some(10));

        // Full progress
        print_progress(100, 100, Some(10));

        // Progress exceeds total (should cap at 100%)
        print_progress(150, 100, Some(10));

        // Small total
        print_progress(1, 2, Some(10));
    }

    /// Test print_progress with various widths
    #[test]
    fn test_print_progress_various_widths() {
        print_progress(50, 100, Some(10));
        print_progress(50, 100, Some(30));
        print_progress(50, 100, Some(50));
    }

    /// Test print_stdout doesn't panic with various inputs
    #[test]
    fn test_print_stdout() {
        print_stdout("Hello, World!".to_string());
        print_stdout("".to_string());
        print_stdout("Unicode: 你好世界 🌍".to_string());
        print_stdout("Numbers: 12345".to_string());
    }

    /// Test print_stderr doesn't panic with various inputs
    #[test]
    fn test_print_stderr() {
        print_stderr("Error message".to_string());
        print_stderr("".to_string());
        print_stderr("Unicode error: 错误 ❌".to_string());
    }

    /// Test print_success doesn't panic
    #[test]
    fn test_print_success() {
        print_success("Operation successful".to_string());
        print_success("".to_string());
    }

    /// Test print_error doesn't panic
    #[test]
    fn test_print_error() {
        print_error("An error occurred".to_string());
        print_error("".to_string());
    }

    /// Test print_warning doesn't panic
    #[test]
    fn test_print_warning() {
        print_warning("This is a warning".to_string());
        print_warning("".to_string());
    }

    /// Test print_info doesn't panic
    #[test]
    fn test_print_info() {
        print_info("Information message".to_string());
        print_info("".to_string());
    }

    /// Test BufferedReader creation and default
    #[test]
    fn test_buffered_reader_new() {
        let _reader = BufferedReader::new();
        let _default_reader = BufferedReader::default();
        // Just verify they can be created
    }

    /// Test with empty string inputs
    #[test]
    fn test_empty_string_inputs() {
        print_stdout("".to_string());
        print_stderr("".to_string());
        print_success("".to_string());
        print_error("".to_string());
        print_warning("".to_string());
        print_info("".to_string());
    }

    /// Test with unicode inputs
    #[test]
    fn test_unicode_inputs() {
        let unicode_text = "你好世界 Hello World 🌍🎉✨";
        print_stdout(unicode_text.to_string());
        print_stderr(unicode_text.to_string());
        print_success(unicode_text.to_string());
        print_error(unicode_text.to_string());
        print_warning(unicode_text.to_string());
        print_info(unicode_text.to_string());
    }

    /// Test with large inputs
    #[test]
    fn test_large_inputs() {
        let large_text = "x".repeat(10000);
        print_stdout(large_text.clone());
        print_stderr(large_text.clone());
        print_progress(5000, 10000, Some(50));
    }

    /// Test with special characters
    #[test]
    fn test_special_characters() {
        let special_chars = "Tab:\t Newline:\n Carriage:\r Quotes:\"' Backslash:\\";
        print_stdout(special_chars.to_string());
        print_stderr(special_chars.to_string());
    }

    /// Test with ANSI escape sequences (should pass through)
    #[test]
    fn test_ansi_sequences() {
        let ansi_text = "\x1b[31mRed Text\x1b[0m \x1b[1mBold\x1b[0m";
        print_stdout(ansi_text.to_string());
    }
}
