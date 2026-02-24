use napi_derive::napi;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use colored::*;

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
