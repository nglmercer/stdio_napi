//! macOS-specific APIs for terminal integration and system utilities.
//!
//! This module provides macOS-specific functionality for:
//! - Terminal integration (Terminal.app, iTerm2)
//! - macOS system utilities
//! - Keychain integration for secure storage

use napi_derive::napi;

/// Terminal application types on macOS.
#[napi]
pub enum MacOSTerminalApp {
    /// Default macOS Terminal.app
    Terminal,
    /// iTerm2
    ITerm2,
    /// VS Code integrated terminal
    VSCode,
    /// Hyper terminal
    Hyper,
    /// Alacritty terminal
    Alacritty,
}

/// macOS notification options.
#[napi(object)]
pub struct NotificationOptions {
    /// Notification title
    pub title: String,
    /// Notification body
    pub body: Option<String>,
    /// Sound to play
    pub sound: Option<String>,
    /// Bundle identifier (for app icon)
    pub bundle_id: Option<String>,
}

/// Check if running on macOS.
#[napi]
pub fn is_macos() -> bool {
    #[cfg(target_os = "macos")]
    {
        true
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

/// Get the current terminal application.
#[napi]
pub fn get_terminal_app() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        // Check for various terminal environment variables
        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            return Some(term_program);
        }

        // Check for iTerm2
        if std::env::var("ITERM_SESSION_ID").is_ok() {
            return Some("iTerm.app".to_string());
        }

        // Check for Terminal.app
        if std::env::var("TERM_SESSION_TOKEN").is_ok() {
            return Some("Apple_Terminal".to_string());
        }

        None
    }

    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// Open a URL in the default browser (uses macOS open command).
#[napi]
pub fn open_url(url: String) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("open")
            .arg(&url)
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to open URL: {}", e)))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to open URL: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = url;
        Err(napi::Error::from_reason(
            "open_url is only available on macOS".to_string(),
        ))
    }
}

/// Open a file or directory in Finder (macOS).
#[napi]
pub fn reveal_in_finder(path: String) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("open")
            .args(["-R", &path])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to reveal in Finder: {}", e)))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to reveal in Finder: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        Err(napi::Error::from_reason(
            "reveal_in_finder is only available on macOS".to_string(),
        ))
    }
}

/// Set the clipboard content (macOS version using pbcopy).
#[napi]
pub fn set_clipboard(content: String) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;

        let mut child = std::process::Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| napi::Error::from_reason(format!("Failed to set clipboard: {}", e)))?;

        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(content.as_bytes()).map_err(|e| {
                napi::Error::from_reason(format!("Failed to write to clipboard: {}", e))
            })?;
        }

        let status = child.wait().map_err(|e| {
            napi::Error::from_reason(format!("Failed to wait for clipboard: {}", e))
        })?;

        if status.success() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(
                "Failed to set clipboard".to_string(),
            ))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = content;
        Err(napi::Error::from_reason(
            "set_clipboard is only available on macOS".to_string(),
        ))
    }
}

/// Get the clipboard content (macOS version using pbpaste).
#[napi]
pub fn get_clipboard() -> napi::Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("pbpaste")
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get clipboard: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to get clipboard: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_clipboard is only available on macOS".to_string(),
        ))
    }
}

/// Get macOS version information.
#[napi]
pub fn get_macos_version() -> napi::Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get macOS version: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(napi::Error::from_reason(
                "Failed to get macOS version".to_string(),
            ))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_macos_version is only available on macOS".to_string(),
        ))
    }
}

/// Get the macOS Darwin kernel version.
#[napi]
pub fn get_darwin_version() -> napi::Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("uname")
            .arg("-r")
            .output()
            .map_err(|e| {
                napi::Error::from_reason(format!("Failed to get Darwin version: {}", e))
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(napi::Error::from_reason(
                "Failed to get Darwin version".to_string(),
            ))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_darwin_version is only available on macOS".to_string(),
        ))
    }
}

/// Get the current user's home directory (handles special paths like ~/Library/Mail).
#[napi]
pub fn get_home_directory() -> String {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "/Users".to_string())
}

/// Get the Applications folder path.
#[napi]
pub fn get_applications_folder() -> String {
    #[cfg(target_os = "macos")]
    {
        "/Applications".to_string()
    }

    #[cfg(not(target_os = "macos"))]
    {
        String::new()
    }
}

/// Get the user Library folder path.
#[napi]
pub fn get_library_folder() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().map(|h| format!("{}/Library", h.display()))
    }

    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// Show a macOS notification using osascript.
#[napi]
pub fn show_notification(
    title: String,
    body: Option<String>,
    sound: Option<bool>,
) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let mut script = format!(
            "display notification \"{}\" with title \"{}\"",
            body.as_deref().unwrap_or(""),
            title
        );

        if sound.unwrap_or(true) {
            script.push_str(" sound name \"default\"");
        }

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to show notification: {}", e)))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to show notification: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = title;
        let _ = body;
        let _ = sound;
        Err(napi::Error::from_reason(
            "show_notification is only available on macOS".to_string(),
        ))
    }
}

/// Keychain item attributes.
#[napi(object)]
pub struct KeychainItem {
    /// Service name (e.g., "MyApp")
    pub service: String,
    /// Account name
    pub account: String,
    /// Password (if retrieved)
    pub password: Option<String>,
}

/// Store a password in the macOS Keychain.
#[napi]
pub fn keychain_store(service: String, account: String, password: String) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "set password to \"{}\"\n\
             do shell script \"security add-internet-password -s '{}' -a '{}' -w $password\"",
            password.replace("\"", "\\\""),
            service.replace("\"", "\\\""),
            account.replace("\"", "\\\"")
        );

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| {
                napi::Error::from_reason(format!("Failed to store keychain item: {}", e))
            })?;

        if output.status.success() {
            Ok(())
        } else {
            // Try alternative method using security command
            let output = std::process::Command::new("security")
                .args([
                    "add-internet-password",
                    "-s",
                    &service,
                    "-a",
                    &account,
                    "-w",
                    &password,
                ])
                .output()
                .map_err(|e| {
                    napi::Error::from_reason(format!("Failed to store keychain item: {}", e))
                })?;

            if output.status.success() {
                Ok(())
            } else {
                Err(napi::Error::from_reason(format!(
                    "Failed to store keychain item: {}",
                    String::from_utf8_lossy(&output.stderr)
                )))
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = service;
        let _ = account;
        let _ = password;
        Err(napi::Error::from_reason(
            "keychain_store is only available on macOS".to_string(),
        ))
    }
}

/// Retrieve a password from the macOS Keychain.
#[napi]
pub fn keychain_retrieve(service: String, account: String) -> napi::Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("security")
            .args([
                "find-internet-password",
                "-s",
                &service,
                "-a",
                &account,
                "-w",
            ])
            .output()
            .map_err(|e| {
                napi::Error::from_reason(format!("Failed to retrieve keychain item: {}", e))
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to retrieve keychain item: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = service;
        let _ = account;
        Err(napi::Error::from_reason(
            "keychain_retrieve is only available on macOS".to_string(),
        ))
    }
}

/// Delete a password from the macOS Keychain.
#[napi]
pub fn keychain_delete(service: String, account: String) -> napi::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("security")
            .args(["delete-internet-password", "-s", &service, "-a", &account])
            .output()
            .map_err(|e| {
                napi::Error::from_reason(format!("Failed to delete keychain item: {}", e))
            })?;

        if output.status.success() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Failed to delete keychain item: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = service;
        let _ = account;
        Err(napi::Error::from_reason(
            "keychain_delete is only available on macOS".to_string(),
        ))
    }
}

/// Get system memory information (macOS).
#[napi]
pub fn get_system_memory() -> napi::Result<SystemMemoryInfo> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get memory info: {}", e)))?;

        let total_memory: u32 = if output.status.success() {
            let bytes: u64 = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse()
                .unwrap_or(0);
            (bytes / (1024 * 1024)) as u32 // Convert to MB
        } else {
            0
        };

        Ok(SystemMemoryInfo {
            total_mb: total_memory,
            // Free/used memory requires parsing vm_stat which is complex
            // Return 0 for now as placeholder
            free_mb: 0,
            available_mb: 0,
            used_mb: 0,
        })
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_system_memory is only available on macOS".to_string(),
        ))
    }
}

/// System memory information.
#[napi(object)]
pub struct SystemMemoryInfo {
    /// Total memory in MB
    pub total_mb: u32,
    /// Free memory in MB
    pub free_mb: u32,
    /// Available memory in MB
    pub available_mb: u32,
    /// Used memory in MB
    pub used_mb: u32,
}

/// Get CPU information (macOS).
#[napi]
pub fn get_cpu_info() -> napi::Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get CPU info: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            // Try alternative
            let output = std::process::Command::new("sysctl")
                .args(["-n", "hw.model"])
                .output()
                .map_err(|e| napi::Error::from_reason(format!("Failed to get CPU info: {}", e)))?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(napi::Error::from_reason(
                    "Failed to get CPU info".to_string(),
                ))
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_cpu_info is only available on macOS".to_string(),
        ))
    }
}

/// Get the number of CPU cores (macOS).
#[napi]
pub fn get_cpu_count() -> napi::Result<u32> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sysctl")
            .args(["-n", "hw.ncpu"])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get CPU count: {}", e)))?;

        if output.status.success() {
            let count: u32 = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse()
                .unwrap_or(1);
            Ok(count)
        } else {
            Ok(1)
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(napi::Error::from_reason(
            "get_cpu_count is only available on macOS".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_macos() {
        let result = is_macos();
        #[cfg(target_os = "macos")]
        assert!(result);
        #[cfg(not(target_os = "macos"))]
        assert!(!result);
    }

    #[test]
    fn test_macos_terminal_app_enum() {
        let _ = MacOSTerminalApp::Terminal;
        let _ = MacOSTerminalApp::ITerm2;
        let _ = MacOSTerminalApp::VSCode;
    }

    #[test]
    fn test_system_memory_info() {
        let info = SystemMemoryInfo {
            total: 16_000_000_000,
            free: 8_000_000_000,
            available: 12_000_000_000,
            used: 4_000_000_000,
        };
        assert!(info.total > 0);
    }
}
