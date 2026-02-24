//! Form inputs for interactive data collection.
//!
//! This module provides form-based input collection with validation,
//! supporting various field types like text, password, select, confirm, etc.

use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal,
};
use napi_derive::napi;
use std::io::{self, Write};
use tokio::io::AsyncBufReadExt;

/// Field type for form inputs
#[napi]
pub enum FormFieldType {
    /// Text input field
    Text,
    /// Password input field (masked)
    Password,
    /// Confirmation field (yes/no)
    Confirm,
    /// Selection field (choose from options)
    Select,
    /// Multi-select field (choose multiple options)
    MultiSelect,
    /// Number input field
    Number,
}

/// Configuration for a form field
#[napi(object)]
pub struct FormFieldConfig {
    /// Field name/label
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Optional default value
    pub default: Option<String>,
    /// Optional placeholder text
    pub placeholder: Option<String>,
    /// Is this field required?
    pub required: Option<bool>,
    /// Validation regex pattern
    pub pattern: Option<String>,
    /// Minimum length for text fields
    pub min_length: Option<u32>,
    /// Maximum length for text fields
    pub max_length: Option<u32>,
    /// Options for select/multi-select fields
    pub options: Option<Vec<String>>,
    /// Help text to display
    pub help: Option<String>,
}

/// Result of a form field
#[napi(object)]
pub struct FormFieldResult {
    /// Field name
    pub name: String,
    /// Field value
    pub value: String,
    /// Whether the field is valid
    pub valid: bool,
    /// Validation error message if invalid
    pub error: Option<String>,
}

/// Result of a complete form
#[napi(object)]
pub struct FormResult {
    /// All field results
    pub fields: Vec<FormFieldResult>,
    /// Whether all fields are valid
    pub valid: bool,
    /// Number of fields
    pub field_count: u32,
}

/// Displays a text input field and collects user input.
///
/// # Arguments
/// * `config` - Form field configuration
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with value and validation status
///
/// # Example
/// ```javascript
/// const { form_text_input } = require('stdio-napi');
/// const result = await form_text_input({
///   name: "username",
///   field_type: "text",
///   placeholder: "Enter your username",
///   required: true,
///   min_length: 3,
///   max_length: 20
/// });
/// ```
#[napi]
pub async fn form_text_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_text_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let required = config.required.unwrap_or(false);
    let min_len = config.min_length.unwrap_or(0) as usize;
    let max_len = config.max_length.unwrap_or(1024) as usize;

    // Display label
    print!("{}: ", config.name.cyan().bold());
    if let Some(ref placeholder) = config.placeholder {
        print!("{} ", placeholder.dimmed());
    }
    let _ = io::stdout().flush();

    // Read input
    let mut input = String::new();
    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);
    reader
        .read_line(&mut input)
        .await
        .map_err(|e| napi::Error::from_reason(format!("Failed to read input: {}", e)))?;

    let value = input.trim().to_string();

    // Validate
    let mut valid = true;
    let mut error: Option<String> = None;

    if required && value.is_empty() {
        valid = false;
        error = Some("This field is required".to_string());
    } else if value.len() < min_len {
        valid = false;
        error = Some(format!("Minimum length is {} characters", min_len));
    } else if value.len() > max_len {
        valid = false;
        error = Some(format!("Maximum length is {} characters", max_len));
    } else if let Some(ref pattern) = config.pattern {
        if let Ok(re) = regex_lite_match(pattern, &value) {
            if !re {
                valid = false;
                error = Some("Input does not match required pattern".to_string());
            }
        }
    }

    Ok(FormFieldResult {
        name: config.name,
        value,
        valid,
        error,
    })
}

/// Simple pattern matching without full regex crate
fn regex_lite_match(pattern: &str, value: &str) -> Result<bool, String> {
    // Simple pattern matching - supports basic wildcards
    // * matches any characters
    // ? matches single character
    // For full regex, would need to add regex crate

    if pattern.starts_with('^') && pattern.ends_with('$') {
        // Exact match
        let inner = &pattern[1..pattern.len() - 1];
        Ok(value == inner)
    } else if pattern.contains('*') {
        // Wildcard match
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let starts = parts[0];
            let ends = parts[1];
            Ok(value.starts_with(starts) && value.ends_with(ends))
        } else {
            Ok(value.contains(&pattern.replace('*', "")))
        }
    } else {
        Ok(value.contains(pattern))
    }
}

/// Displays a password input field with masking.
///
/// # Arguments
/// * `config` - Form field configuration
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with value and validation status
///
/// # Example
/// ```javascript
/// const { form_password_input } = require('stdio-napi');
/// const result = await form_password_input({
///   name: "password",
///   field_type: "password",
///   required: true,
///   min_length: 8
/// });
/// ```
#[napi]
pub async fn form_password_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_password_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let required = config.required.unwrap_or(false);
    let min_len = config.min_length.unwrap_or(0) as usize;
    let max_len = config.max_length.unwrap_or(128) as usize;

    // Display label
    print!("{}: ", config.name.cyan().bold());
    let _ = io::stdout().flush();

    let mut password = String::new();

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

    let result: Result<String, napi::Error> = async {
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
                            println!();
                            break;
                        }
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                            return Err(napi::Error::from_reason("Interrupted".to_string()));
                        }
                        KeyCode::Char(c) => {
                            if password.len() < max_len {
                                password.push(c);
                                print!("*");
                                let _ = io::stdout().flush();
                            }
                        }
                        KeyCode::Backspace => {
                            if !password.is_empty() {
                                password.pop();
                                print!("\x08 \x08");
                                let _ = io::stdout().flush();
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
    let password = result?;

    // Validate
    let mut valid = true;
    let mut error: Option<String> = None;

    if required && password.is_empty() {
        valid = false;
        error = Some("Password is required".to_string());
    } else if password.len() < min_len {
        valid = false;
        error = Some(format!("Password must be at least {} characters", min_len));
    }

    Ok(FormFieldResult {
        name: config.name,
        value: password,
        valid,
        error,
    })
}

/// Displays a confirmation field (yes/no).
///
/// # Arguments
/// * `config` - Form field configuration
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with value and validation status
///
/// # Example
/// ```javascript
/// const { form_confirm_input } = require('stdio-napi');
/// const result = await form_confirm_input({
///   name: "Continue?",
///   field_type: "confirm",
///   default: "true"
/// });
/// ```
#[napi]
pub async fn form_confirm_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_confirm_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let default_val = config.default.as_deref().unwrap_or("true");
    let default_bool =
        default_val.to_lowercase() == "true" || default_val == "1" || default_val == "yes";
    let suffix = if default_bool { "[Y/n]" } else { "[y/N]" };

    print!("{} {}: ", config.name.cyan().bold(), suffix);
    let _ = io::stdout().flush();

    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .await
        .map_err(|e| napi::Error::from_reason(format!("Failed to read input: {}", e)))?;

    let input = input.trim().to_lowercase();
    let value = if input.is_empty() {
        default_bool.to_string()
    } else if input == "y" || input == "yes" {
        "true".to_string()
    } else if input == "n" || input == "no" {
        "false".to_string()
    } else {
        default_bool.to_string()
    };

    Ok(FormFieldResult {
        name: config.name,
        value,
        valid: true,
        error: None,
    })
}

/// Displays a selection field with arrow key navigation.
///
/// # Arguments
/// * `config` - Form field configuration with options
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with selected value
///
/// # Example
/// ```javascript
/// const { form_select_input } = require('stdio-napi');
/// const result = await form_select_input({
///   name: "Choose option",
///   field_type: "select",
///   options: ["Option 1", "Option 2", "Option 3"]
/// });
/// ```
#[napi]
pub async fn form_select_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_select_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let options = config.options.as_ref().ok_or_else(|| {
        napi::Error::from_reason("Options are required for select field".to_string())
    })?;

    if options.is_empty() {
        return Err(napi::Error::from_reason(
            "Options cannot be empty".to_string(),
        ));
    }

    let mut selected = 0;

    println!("{}: ", config.name.cyan().bold());

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

    let result: Result<usize, napi::Error> = async {
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
                    // Redraw one last time with selection
                    for (i, opt) in options.iter().enumerate() {
                        if i == selected {
                            println!(" > {}", opt.green().bold());
                        } else {
                            println!("   {}", opt);
                        }
                    }
                    break;
                }
                _ => {}
            }
        }
        Ok(selected)
    }
    .await;

    let _ = terminal::disable_raw_mode();
    let selected = result?;

    Ok(FormFieldResult {
        name: config.name,
        value: options[selected].clone(),
        valid: true,
        error: None,
    })
}

/// Displays a multi-select field with checkbox-style selection.
///
/// # Arguments
/// * `config` - Form field configuration with options
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with comma-separated selected values
///
/// # Example
/// ```javascript
/// const { form_multi_select_input } = require('stdio-napi');
/// const result = await form_multi_select_input({
///   name: "Select features",
///   field_type: "multiSelect",
///   options: ["Feature 1", "Feature 2", "Feature 3"]
/// });
/// ```
#[napi]
pub async fn form_multi_select_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_multi_select_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let options = config.options.as_ref().ok_or_else(|| {
        napi::Error::from_reason("Options are required for multi-select field".to_string())
    })?;

    if options.is_empty() {
        return Err(napi::Error::from_reason(
            "Options cannot be empty".to_string(),
        ));
    }

    let mut selected_idx = 0;
    let mut selected: Vec<bool> = vec![false; options.len()];

    println!(
        "{}: (Space to toggle, Enter to confirm)",
        config.name.cyan().bold()
    );

    terminal::enable_raw_mode()
        .map_err(|e| napi::Error::from_reason(format!("Failed to enable raw mode: {}", e)))?;

    let result: Result<String, napi::Error> = async {
        loop {
            // Draw options
            for (i, opt) in options.iter().enumerate() {
                let checkbox = if selected[i] { "[x]" } else { "[ ]" };
                if i == selected_idx {
                    println!(
                        " > {} {}",
                        checkbox.green().bold(),
                        opt.green().bold()
                    );
                } else {
                    println!("   {} {}", checkbox, opt);
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
                    selected_idx = if selected_idx == 0 {
                        options.len() - 1
                    } else {
                        selected_idx - 1
                    };
                }
                KeyCode::Down => {
                    selected_idx = (selected_idx + 1) % options.len();
                }
                KeyCode::Char(' ') => {
                    selected[selected_idx] = !selected[selected_idx];
                }
                KeyCode::Enter => {
                    // Redraw one last time
                    for (i, opt) in options.iter().enumerate() {
                        let checkbox = if selected[i] { "[x]" } else { "[ ]" };
                        if i == selected_idx {
                            println!(
                                " > {} {}",
                                ">",
                                checkbox.green().bold(),
                                opt.green().bold()
                            );
                        } else {
                            println!("   {} {}", checkbox, opt);
                        }
                    }
                    break;
                }
                _ => {}
            }
        }

        // Collect selected values
        let selected_values: Vec<String> = options
            .iter()
            .enumerate()
            .filter(|(i, _)| selected[*i])
            .map(|(_, opt)| opt.clone())
            .collect();

        Ok(selected_values.join(","))
    }
    .await;

    let _ = terminal::disable_raw_mode();
    let value = result?;

    Ok(FormFieldResult {
        name: config.name,
        value,
        valid: true,
        error: None,
    })
}

/// Displays a number input field with validation.
///
/// # Arguments
/// * `config` - Form field configuration
///
/// # Returns
/// * `Result<FormFieldResult, napi::Error>` - The field result with numeric value
///
/// # Example
/// ```javascript
/// const { form_number_input } = require('stdio-napi');
/// const result = await form_number_input({
///   name: "Age",
///   field_type: "number",
///   required: true,
///   min_length: 0,
///   max_length: 150
/// });
/// ```
#[napi]
pub async fn form_number_input(config: FormFieldConfig) -> napi::Result<FormFieldResult> {
    // Check if stdin is a TTY
    if !atty::is(atty::Stream::Stdin) {
        return Err(napi::Error::from_reason(
            "form_number_input requires a terminal (TTY). Not running in interactive mode."
                .to_string(),
        ));
    }

    let required = config.required.unwrap_or(false);
    let min_val = config.min_length.map(|v| v as i64);
    let max_val = config.max_length.map(|v| v as i64);

    // Display label
    print!("{}: ", config.name.cyan().bold());
    if let Some(ref placeholder) = config.placeholder {
        print!("{} ", placeholder.dimmed());
    }
    let _ = io::stdout().flush();

    // Read input
    let mut input = String::new();
    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);
    reader
        .read_line(&mut input)
        .await
        .map_err(|e| napi::Error::from_reason(format!("Failed to read input: {}", e)))?;

    let value = input.trim().to_string();

    // Validate
    let mut valid = true;
    let mut error: Option<String> = None;

    if required && value.is_empty() {
        valid = false;
        error = Some("This field is required".to_string());
    } else if !value.is_empty() {
        // Check if it's a valid number
        match value.parse::<i64>() {
            Ok(num) => {
                if let Some(min) = min_val {
                    if num < min {
                        valid = false;
                        error = Some(format!("Value must be at least {}", min));
                    }
                }
                if let Some(max) = max_val {
                    if num > max {
                        valid = false;
                        error = Some(format!("Value must be at most {}", max));
                    }
                }
            }
            Err(_) => {
                valid = false;
                error = Some("Please enter a valid number".to_string());
            }
        }
    }

    Ok(FormFieldResult {
        name: config.name,
        value,
        valid,
        error,
    })
}

/// Collects multiple form fields in sequence.
///
/// # Arguments
/// * `fields` - Vector of form field configurations
///
/// # Returns
/// * `Result<FormResult, napi::Error>` - Complete form result with all field values
///
/// # Example
/// ```javascript
/// const { collect_form } = require('stdio-napi');
/// const result = await collect_form([
///   { name: "Name", field_type: "text", required: true },
///   { name: "Email", field_type: "text", required: true },
///   { name: "Age", field_type: "number" }
/// ]);
/// console.log(result.fields);
/// ```
#[napi]
pub async fn collect_form(fields: Vec<FormFieldConfig>) -> napi::Result<FormResult> {
    let mut results = Vec::new();

    for field in fields {
        let result = match field.field_type.as_str() {
            "text" => form_text_input(field).await?,
            "password" => form_password_input(field).await?,
            "confirm" => form_confirm_input(field).await?,
            "select" => form_select_input(field).await?,
            "multiSelect" => form_multi_select_input(field).await?,
            "number" => form_number_input(field).await?,
            _ => {
                // Default to text input
                form_text_input(field).await?
            }
        };
        results.push(result);
    }

    let all_valid = results.iter().all(|r| r.valid);
    let field_count = results.len() as u32;

    Ok(FormResult {
        fields: results,
        valid: all_valid,
        field_count,
    })
}

/// Displays a form with a title and collects all fields.
///
/// # Arguments
/// * `title` - Form title to display
/// * `fields` - Vector of form field configurations
///
/// # Returns
/// * `Result<FormResult, napi::Error>` - Complete form result with all field values
///
/// # Example
/// ```javascript
/// const { display_form } = require('stdio-napi');
/// const result = await display_form("User Registration", [
///   { name: "Username", field_type: "text", required: true },
///   { name: "Password", field_type: "password", required: true, min_length: 8 },
///   { name: "Accept Terms", field_type: "confirm" }
/// ]);
/// ```
#[napi]
pub async fn display_form(title: String, fields: Vec<FormFieldConfig>) -> napi::Result<FormResult> {
    // Display form title
    println!("\n{}", title.green().bold().underline());
    println!("{}\n", "─".repeat(title.len()));

    let result = collect_form(fields).await?;

    // Display summary
    println!("\n{}", "Form Summary".green().bold());
    println!("{}", "─".repeat(13));
    for field in &result.fields {
        let status = if field.valid {
            "✓".green()
        } else {
            "✗".red()
        };
        let value = if field.name.to_lowercase().contains("password") {
            "*".repeat(field.value.len())
        } else {
            field.value.clone()
        };
        println!("{} {}: {}", status, field.name, value);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_lite_match_exact() {
        assert!(regex_lite_match("^test$", "test").unwrap());
        assert!(!regex_lite_match("^test$", "test2").unwrap());
    }

    #[test]
    fn test_regex_lite_match_wildcard() {
        assert!(regex_lite_match("test*", "testing").unwrap());
        assert!(regex_lite_match("*test", "mytest").unwrap());
        assert!(regex_lite_match("te*st", "te123st").unwrap());
    }

    #[test]
    fn test_regex_lite_match_contains() {
        assert!(regex_lite_match("test", "mytestvalue").unwrap());
        assert!(!regex_lite_match("xyz", "test").unwrap());
    }

    #[test]
    fn test_form_field_config_defaults() {
        let config = FormFieldConfig {
            name: "test".to_string(),
            field_type: "text".to_string(),
            default: None,
            placeholder: None,
            required: None,
            pattern: None,
            min_length: None,
            max_length: None,
            options: None,
            help: None,
        };

        assert_eq!(config.name, "test");
        assert_eq!(config.field_type, "text");
        assert!(config.default.is_none());
        assert!(config.required.is_none());
    }
}
