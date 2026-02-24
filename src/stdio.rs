use napi_derive::napi;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use colored::*;
use crossterm::execute;

#[napi]
pub fn print_stdout(text: String) {
    print!("{}", text);
    let _ = io::stdout().flush();
}

#[napi]
pub fn print_stderr(text: String) {
    eprint!("{}", text);
    let _ = io::stderr().flush();
}

#[napi]
pub fn print_success(text: String) {
    println!("{}", text.green().bold());
}

#[napi]
pub fn print_error(text: String) {
    eprintln!("{}", text.red().bold());
}

#[napi]
pub fn print_warning(text: String) {
    println!("{}", text.yellow());
}

#[napi]
pub fn print_info(text: String) {
    println!("{}", text.blue());
}

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

#[napi]
pub async fn prompt(message: String) -> napi::Result<String> {
    print!("{}: ", message.cyan());
    let _ = io::stdout().flush();
    
    read_line().await
}

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

#[napi]
pub fn get_spinner_frame(frame: u32) -> String {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    frames[(frame as usize) % frames.len()].to_string()
}

#[napi]
pub async fn read_password(mask: Option<String>) -> napi::Result<String> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal;
    
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

#[napi]
pub async fn select_menu(message: String, options: Vec<String>) -> napi::Result<u32> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent};
    use crossterm::terminal;
    use crossterm::cursor;
    
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
