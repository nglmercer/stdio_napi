//! Cross-platform signal handling for processes.
//!
//! This module provides unified signal handling across Unix and Windows platforms,
//! allowing graceful shutdown, process termination, and signal-based communication.

use napi_derive::napi;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[cfg(unix)]
use {
    futures_util::StreamExt,
    signal_hook::consts::signal::{self, SIGINT, SIGTERM, SIGUSR1, SIGUSR2, SIGWINCH},
    signal_hook_tokio::Signals,
};

/// Signal types that can be sent to processes.
#[napi]
pub enum Signal {
    /// Interrupt signal (Ctrl+C)
    Interrupt,
    /// Termination signal
    Terminate,
    /// Kill signal (cannot be caught)
    Kill,
    /// User-defined signal 1 (Unix only)
    User1,
    /// User-defined signal 2 (Unix only)
    User2,
    /// Window size change signal (Unix only)
    WindowChange,
    /// Continue signal (Unix only)
    Continue,
    /// Stop signal (Unix only)
    Stop,
}

/// Signal information structure.
#[napi(object)]
pub struct SignalInfo {
    /// The signal type
    pub signal: String,
    /// Signal number (platform-specific)
    pub number: i32,
    /// Human-readable description
    pub description: String,
}

/// Signal handler for receiving process signals.
///
/// This class provides a way to listen for signals and handle them gracefully.
#[napi]
pub struct SignalHandler {
    #[cfg(unix)]
    signals: Arc<Mutex<Option<Signals>>>,
    #[cfg(unix)]
    shutdown_tx: broadcast::Sender<()>,
    #[cfg(windows)]
    _phantom: (),
}

#[napi]
impl SignalHandler {
    /// Create a new signal handler.
    ///
    /// # Arguments
    /// * `signals` - List of signals to listen for
    ///
    /// # Example
    /// ```javascript
    /// const { SignalHandler, Signal } = require('stdio-napi');
    /// const handler = new SignalHandler(['interrupt', 'terminate']);
    /// ```
    #[napi(constructor)]
    pub fn new(signals: Vec<String>) -> napi::Result<SignalHandler> {
        #[cfg(unix)]
        {
            let mut signal_set = Vec::new();
            for sig in signals.iter() {
                match sig.to_lowercase().as_str() {
                    "interrupt" | "sigint" => signal_set.push(SIGINT),
                    "terminate" | "sigterm" => signal_set.push(SIGTERM),
                    "user1" | "sigusr1" => signal_set.push(SIGUSR1),
                    "user2" | "sigusr2" => signal_set.push(SIGUSR2),
                    "window" | "sigwinch" => signal_set.push(SIGWINCH),
                    _ => return Err(napi::Error::from_reason(format!("Unknown signal: {}", sig))),
                }
            }

            let signals = Signals::new(&signal_set).map_err(|e| {
                napi::Error::from_reason(format!("Failed to create signal handler: {}", e))
            })?;

            let (shutdown_tx, _) = broadcast::channel(1);

            Ok(SignalHandler {
                signals: Arc::new(Mutex::new(Some(signals))),
                shutdown_tx,
            })
        }

        #[cfg(windows)]
        {
            // Windows signal handling is more limited
            let _ = signals; // Suppress unused warning
            Ok(SignalHandler { _phantom: () })
        }
    }

    /// Wait for the next signal.
    ///
    /// # Returns
    /// * `SignalInfo` - Information about the received signal
    #[napi]
    pub async fn wait(&self) -> napi::Result<SignalInfo> {
        #[cfg(unix)]
        {
            let mut signals_guard = self.signals.lock().await;
            if let Some(ref mut signals) = *signals_guard {
                if let Some(sig) = signals.next().await {
                    Ok(signal_to_info(sig))
                } else {
                    Err(napi::Error::from_reason(
                        "Signal handler closed".to_string(),
                    ))
                }
            } else {
                Err(napi::Error::from_reason(
                    "Signal handler not initialized".to_string(),
                ))
            }
        }

        #[cfg(windows)]
        {
            // On Windows, we use a simpler approach
            Err(napi::Error::from_reason(
                "Signal waiting not supported on Windows".to_string(),
            ))
        }
    }

    /// Close the signal handler.
    #[napi]
    pub async fn close(&self) -> napi::Result<()> {
        #[cfg(unix)]
        {
            let mut signals_guard = self.signals.lock().await;
            *signals_guard = None;
            let _ = self.shutdown_tx.send(());
        }
        #[cfg(windows)]
        {
            // Nothing to do on Windows
        }
        Ok(())
    }
}

/// Send a signal to a process by PID.
///
/// # Arguments
/// * `pid` - Process ID to send signal to
/// * `signal` - The signal to send
///
/// # Returns
/// * `bool` - True if signal was sent successfully
///
/// # Example
/// ```javascript
/// const { send_signal, Signal } = require('stdio-napi');
/// send_signal(1234, 'terminate');
/// ```
#[napi]
pub fn send_signal(pid: u32, signal: String) -> napi::Result<bool> {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal as NixSignal};
        use nix::unistd::Pid;

        let nix_signal = match signal.to_lowercase().as_str() {
            "interrupt" | "sigint" => NixSignal::SIGINT,
            "terminate" | "sigterm" => NixSignal::SIGTERM,
            "kill" | "sigkill" => NixSignal::SIGKILL,
            "user1" | "sigusr1" => NixSignal::SIGUSR1,
            "user2" | "sigusr2" => NixSignal::SIGUSR2,
            "continue" | "sigcont" => NixSignal::SIGCONT,
            "stop" | "sigstop" => NixSignal::SIGSTOP,
            _ => {
                return Err(napi::Error::from_reason(format!(
                    "Unknown signal: {}",
                    signal
                )))
            }
        };

        kill(Pid::from_raw(pid as i32), nix_signal)
            .map(|_| true)
            .map_err(|e| napi::Error::from_reason(format!("Failed to send signal: {}", e)))
    }

    #[cfg(windows)]
    {
        // Windows uses different mechanism for process termination
        let _ = pid;
        let _ = signal;
        Err(napi::Error::from_reason(
            "Signal sending not supported on Windows".to_string(),
        ))
    }
}

/// Get information about a signal.
///
/// # Arguments
/// * `signal` - The signal name
///
/// # Returns
/// * `SignalInfo` - Information about the signal
#[napi]
pub fn get_signal_info(signal: String) -> napi::Result<SignalInfo> {
    #[cfg(unix)]
    {
        let sig_num = match signal.to_lowercase().as_str() {
            "interrupt" | "sigint" => signal::SIGINT,
            "terminate" | "sigterm" => signal::SIGTERM,
            "kill" | "sigkill" => signal::SIGKILL,
            "user1" | "sigusr1" => signal::SIGUSR1,
            "user2" | "sigusr2" => signal::SIGUSR2,
            "window" | "sigwinch" => signal::SIGWINCH,
            "continue" | "sigcont" => signal::SIGCONT,
            "stop" | "sigstop" => signal::SIGSTOP,
            _ => {
                return Err(napi::Error::from_reason(format!(
                    "Unknown signal: {}",
                    signal
                )))
            }
        };

        Ok(signal_to_info(sig_num))
    }

    #[cfg(windows)]
    {
        let _ = signal;
        Err(napi::Error::from_reason(
            "Signal info not available on Windows".to_string(),
        ))
    }
}

/// Check if the current process has a TTY stdin.
///
/// # Returns
/// * `bool` - True if stdin is connected to a TTY
#[napi]
pub fn is_stdin_tty_signal() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal()
}

/// Check if the current process is running in the background.
///
/// # Returns
/// * `bool` - True if the process is in the background
#[napi]
pub fn is_background() -> bool {
    #[cfg(unix)]
    {
        use nix::unistd::getpgrp;
        use std::os::unix::io::OwnedFd;

        if let Ok(tty) = std::fs::File::open("/dev/tty") {
            let tty_fd = OwnedFd::from(tty);
            if let Ok(fg_pgrp) = nix::unistd::tcgetpgrp(&tty_fd) {
                return fg_pgrp != getpgrp();
            }
        }
        false
    }

    #[cfg(windows)]
    {
        false
    }
}

/// Get the process group ID.
///
/// # Returns
/// * `u32` - The process group ID
#[napi]
pub fn get_process_group() -> u32 {
    #[cfg(unix)]
    {
        use nix::unistd::getpgrp;
        getpgrp().as_raw() as u32
    }

    #[cfg(windows)]
    {
        std::process::id()
    }
}

/// Set the process group ID for a process.
///
/// # Arguments
/// * `pid` - Process ID (0 for current process)
/// * `pgid` - Process group ID (0 to create new group)
///
/// # Returns
/// * `bool` - True if successful
#[napi]
pub fn set_process_group(pid: u32, pgid: u32) -> napi::Result<bool> {
    #[cfg(unix)]
    {
        use nix::unistd::setpgid;
        use nix::unistd::Pid;

        let pid_val = if pid == 0 {
            Pid::from_raw(0)
        } else {
            Pid::from_raw(pid as i32)
        };
        let pgid_val = if pgid == 0 {
            Pid::from_raw(0)
        } else {
            Pid::from_raw(pgid as i32)
        };

        setpgid(pid_val, pgid_val)
            .map(|_| true)
            .map_err(|e| napi::Error::from_reason(format!("Failed to set process group: {}", e)))
    }

    #[cfg(windows)]
    {
        let _ = pid;
        let _ = pgid;
        Err(napi::Error::from_reason(
            "Process group manipulation not supported on Windows".to_string(),
        ))
    }
}

#[cfg(unix)]
fn signal_to_info(sig: i32) -> SignalInfo {
    let (name, description) = match sig {
        signal::SIGINT => ("SIGINT", "Interrupt signal (Ctrl+C)"),
        signal::SIGTERM => ("SIGTERM", "Termination signal"),
        signal::SIGKILL => ("SIGKILL", "Kill signal (cannot be caught)"),
        signal::SIGUSR1 => ("SIGUSR1", "User-defined signal 1"),
        signal::SIGUSR2 => ("SIGUSR2", "User-defined signal 2"),
        signal::SIGWINCH => ("SIGWINCH", "Window size change"),
        signal::SIGCONT => ("SIGCONT", "Continue signal"),
        signal::SIGSTOP => ("SIGSTOP", "Stop signal"),
        signal::SIGHUP => ("SIGHUP", "Hangup signal"),
        signal::SIGQUIT => ("SIGQUIT", "Quit signal"),
        signal::SIGALRM => ("SIGALRM", "Alarm signal"),
        signal::SIGCHLD => ("SIGCHLD", "Child process status change"),
        _ => ("UNKNOWN", "Unknown signal"),
    };

    SignalInfo {
        signal: name.to_string(),
        number: sig,
        description: description.to_string(),
    }
}

/// Signal constants for reference.
#[napi]
pub fn get_supported_signals() -> Vec<SignalInfo> {
    #[cfg(unix)]
    {
        vec![
            signal_to_info(signal::SIGINT),
            signal_to_info(signal::SIGTERM),
            signal_to_info(signal::SIGKILL),
            signal_to_info(signal::SIGUSR1),
            signal_to_info(signal::SIGUSR2),
            signal_to_info(signal::SIGWINCH),
            signal_to_info(signal::SIGCONT),
            signal_to_info(signal::SIGSTOP),
            signal_to_info(signal::SIGHUP),
            signal_to_info(signal::SIGQUIT),
        ]
    }

    #[cfg(windows)]
    {
        vec![]
    }
}
