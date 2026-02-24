use napi_derive::napi;
use std::collections::HashMap;
use std::process::Stdio as StdStdio;
use std::process::Command as StdCommand;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

#[cfg(unix)]
#[allow(unused_imports)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
#[allow(unused_imports)]
use std::os::windows::process::CommandExt;

/// Buffer configuration for stream handling.
///
/// Used to configure buffer sizes for process I/O operations.
#[napi(object)]
pub struct BufferConfig {
    /// Size of read buffer in bytes (default: 8192)
    pub read_size: Option<u32>,
    /// Size of write buffer in bytes (default: 8192)
    pub write_size: Option<u32>,
    /// Maximum buffer size before applying backpressure (default: 65536)
    pub max_size: Option<u32>,
}

impl Default for BufferConfig {
    fn default() -> Self {
        BufferConfig {
            read_size: Some(8192),
            write_size: Some(8192),
            max_size: Some(65536),
        }
    }
}

/// Stream event types for process communication.
#[napi]
pub enum StreamEvent {
    /// stdout data received
    Stdout,
    /// stderr data received
    Stderr,
    /// Process exited
    Exit,
    /// Error occurred
    Error,
}

/// Stream event data for process communication.
#[napi(object)]
pub struct StreamEventData {
    /// The type of event
    pub event: StreamEvent,
    /// Data associated with the event (if any)
    pub data: Option<String>,
    /// Exit code (for Exit events)
    pub code: Option<i32>,
}

/// Managed process handle with streaming capabilities.
///
/// This class provides a higher-level interface for process management
/// with support for streaming stdin/stdout/stderr.
#[napi]
pub struct ManagedProcess {
    pid: u32,
    child: Arc<Mutex<Option<Child>>>,
}

#[napi]
impl ManagedProcess {
    /// Create a new managed process.
    ///
    /// # Arguments
    /// * `command` - The executable to run
    /// * `args` - Command-line arguments
    ///
    /// # Example
    /// ```javascript
    /// const { ManagedProcess } = require('stdio-napi');
    /// const proc = new ManagedProcess("ls", ["-la"]);
    /// ```
    #[napi(constructor)]
    pub fn new(command: String, args: Vec<String>) -> napi::Result<ManagedProcess> {
        let child = Command::new(&command)
            .args(&args)
            .stdout(StdStdio::piped())
            .stderr(StdStdio::piped())
            .stdin(StdStdio::piped())
            .spawn()
            .map_err(|e| napi::Error::from_reason(format!("Failed to spawn '{}': {}", command, e)))?;

        let pid = child.id().unwrap_or(0);

        Ok(ManagedProcess {
            pid,
            child: Arc::new(Mutex::new(Some(child))),
        })
    }

    /// Get the process PID.
    ///
    /// # Returns
    /// * `u32` - The process ID
    #[napi]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Read a line from stdout.
    ///
    /// # Returns
    /// * `Result<Option<String>, napi::Error>` - Line of output or None if closed
    #[napi]
    pub async fn read_stdout_line(&self) -> napi::Result<Option<String>> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stdout) = child.stdout {
                let mut reader = BufReader::new(stdout);
                let mut line = String::new();
                match reader.read_line(&mut line).await {
                    Ok(0) => Ok(None),
                    Ok(_) => Ok(Some(line)),
                    Err(e) => Err(napi::Error::from_reason(format!("Read error: {}", e))),
                }
            } else {
                Err(napi::Error::from_reason("Stdout not piped".to_string()))
            }
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Read a line from stderr.
    ///
    /// # Returns
    /// * `Result<Option<String>, napi::Error>` - Line of error output or None if closed
    #[napi]
    pub async fn read_stderr_line(&self) -> napi::Result<Option<String>> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stderr) = child.stderr {
                let mut reader = BufReader::new(stderr);
                let mut line = String::new();
                match reader.read_line(&mut line).await {
                    Ok(0) => Ok(None),
                    Ok(_) => Ok(Some(line)),
                    Err(e) => Err(napi::Error::from_reason(format!("Read error: {}", e))),
                }
            } else {
                Err(napi::Error::from_reason("Stderr not piped".to_string()))
            }
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Write to stdin.
    ///
    /// # Arguments
    /// * `data` - The string to write
    #[napi]
    pub async fn write_stdin(&self, data: String) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(data.as_bytes()).await
                    .map_err(|e| napi::Error::from_reason(format!("Write error: {}", e)))?;
                Ok(())
            } else {
                Err(napi::Error::from_reason("Stdin not piped".to_string()))
            }
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Wait for process to complete and get exit status.
    ///
    /// # Returns
    /// * `Result<ProcessStatus, napi::Error>` - The process exit status
    #[napi]
    pub async fn wait(&self) -> napi::Result<ProcessStatus> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = child_guard.take() {
            let status = child.wait().await
                .map_err(|e| napi::Error::from_reason(format!("Wait error: {}", e)))?;
            Ok(ProcessStatus {
                pid: self.pid,
                success: status.success(),
                code: status.code(),
            })
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Kill the process.
    #[napi]
    pub async fn kill(&self) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            child.kill().await
                .map_err(|e| napi::Error::from_reason(format!("Kill error: {}", e)))?;
            Ok(())
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }
}

/// Spawn options for process execution.
#[napi(object)]
pub struct SpawnOptions {
    /// The command to execute
    pub command: String,
    /// Command-line arguments
    pub args: Vec<String>,
    /// Working directory (optional)
    pub cwd: Option<String>,
    /// Environment variables (optional)
    pub env: Option<HashMap<String, String>>,
    /// Whether to capture stdout (default: false)
    pub capture_stdout: Option<bool>,
    /// Whether to capture stderr (default: false)
    pub capture_stderr: Option<bool>,
}

/// Process status result containing exit information.
#[napi(object)]
#[derive(Clone)]
pub struct ProcessStatus {
    /// Process ID
    pub pid: u32,
    /// Whether the process exited successfully (code 0)
    pub success: bool,
    /// Exit code (None if still running or terminated by signal)
    pub code: Option<i32>,
}

/// Process output containing stdout and stderr.
#[napi(object)]
pub struct ProcessOutput {
    /// Standard output content
    pub stdout: String,
    /// Standard error content
    pub stderr: String,
    /// Exit code
    pub code: Option<i32>,
    /// Whether the process was successful
    pub success: bool,
}

/// Execute a command and wait for completion (async).
///
/// # Arguments
/// * `command` - The command to execute
/// * `args` - Command-line arguments
///
/// # Returns
/// * `Result<ProcessStatus, napi::Error>` - Process status information
///
/// # Example
/// ```javascript
/// const { exec_command } = require('stdio-napi');
/// const status = await exec_command("ls", ["-la"]);
/// console.log(status.pid, status.code);
/// ```
#[napi]
pub async fn exec_command(command: String, args: Vec<String>) -> napi::Result<ProcessStatus> {
    let mut child = Command::new(&command)
        .args(&args)
        .stdout(StdStdio::inherit())
        .stderr(StdStdio::inherit())
        .spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to spawn '{}': {}", command, e)))?;

    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}

/// Spawn a process with options.
///
/// # Arguments
/// * `options` - Spawn options including command, args, cwd, env
///
/// # Returns
/// * `Result<ProcessStatus, napi::Error>` - Process status information
///
/// # Example
/// ```javascript
/// const { spawn_with_options } = require('stdio-napi');
/// const status = await spawn_with_options({
///   command: "node",
///   args: ["script.js"],
///   cwd: "/path/to/dir",
///   capture_stdout: true
/// });
/// ```
#[napi]
pub async fn spawn_with_options(options: SpawnOptions) -> napi::Result<ProcessStatus> {
    let mut cmd = Command::new(&options.command);
    cmd.args(&options.args);

    if let Some(cwd) = &options.cwd {
        cmd.current_dir(cwd);
    }

    if let Some(env) = &options.env {
        cmd.envs(env);
    }

    let capture_stdout = options.capture_stdout.unwrap_or(false);
    let capture_stderr = options.capture_stderr.unwrap_or(false);

    if capture_stdout {
        cmd.stdout(StdStdio::piped());
    } else {
        cmd.stdout(StdStdio::inherit());
    }

    if capture_stderr {
        cmd.stderr(StdStdio::piped());
    } else {
        cmd.stderr(StdStdio::inherit());
    }

    let mut child = cmd.spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to spawn '{}': {}", options.command, e)))?;

    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}

/// Spawn a process with piped stdio for streaming.
///
/// # Arguments
/// * `options` - Spawn options
///
/// # Returns
/// * `Result<ProcessStatus, napi::Error>` - Process status information
///
/// # Example
/// ```javascript
/// const { spawn_with_pipes } = require('stdio-napi');
/// const status = await spawn_with_pipes({
///   command: "node",
///   args: ["script.js"]
/// });
/// ```
#[napi]
pub async fn spawn_with_pipes(options: SpawnOptions) -> napi::Result<ProcessStatus> {
    let mut cmd = Command::new(&options.command);
    cmd.args(&options.args);

    if let Some(cwd) = &options.cwd {
        cmd.current_dir(cwd);
    }

    if let Some(env) = &options.env {
        cmd.envs(env);
    }

    cmd.stdout(StdStdio::piped());
    cmd.stderr(StdStdio::piped());
    cmd.stdin(StdStdio::piped());

    let mut child = cmd.spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to spawn '{}': {}", options.command, e)))?;

    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}

/// Execute a command synchronously and return output.
///
/// # Arguments
/// * `command` - The command to execute
///
/// # Returns
/// * `Result<ProcessOutput, napi::Error>` - Combined stdout and stderr
///
/// # Example
/// ```javascript
/// const { exec_sync } = require('stdio-napi');
/// const output = exec_sync("ls");
/// console.log(output.stdout);
/// ```
#[napi]
pub fn exec_sync(command: String) -> napi::Result<ProcessOutput> {
    let output = StdCommand::new(&command)
        .output()
        .map_err(|e| napi::Error::from_reason(format!("Failed to execute '{}': {}", command, e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(ProcessOutput {
        stdout,
        stderr,
        code: output.status.code(),
        success: output.status.success(),
    })
}

/// Execute a command with arguments synchronously.
///
/// # Arguments
/// * `command` - The command to execute
/// * `args` - Command-line arguments
///
/// # Returns
/// * `Result<ProcessOutput, napi::Error>` - Combined stdout and stderr
///
/// # Example
/// ```javascript
/// const { exec_sync_with_args } = require('stdio-napi');
/// const output = exec_sync_with_args("ls", ["-la", "/tmp"]);
/// ```
#[napi]
pub fn exec_sync_with_args(command: String, args: Vec<String>) -> napi::Result<ProcessOutput> {
    let output = StdCommand::new(&command)
        .args(&args)
        .output()
        .map_err(|e| napi::Error::from_reason(format!("Failed to execute '{}': {}", command, e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(ProcessOutput {
        stdout,
        stderr,
        code: output.status.code(),
        success: output.status.success(),
    })
}

/// Shell escape a string for safe command execution.
///
/// # Arguments
/// * `input` - The string to escape
///
/// # Returns
/// * `String` - The escaped string
///
/// # Example
/// ```javascript
/// const { shell_escape } = require('stdio-napi');
/// const escaped = shell_escape("hello world"); // Returns 'hello world'
/// ```
#[napi]
pub fn shell_escape(input: String) -> String {
    format!("'{}'", input.replace('\'', "'\\''"))
}

/// Shell escape arguments for safe command execution.
///
/// # Arguments
/// * `args` - Vector of strings to escape
///
/// # Returns
/// * `Vec<String>` - Vector of escaped strings
///
/// # Example
/// ```javascript
/// const { shell_escape_args } = require('stdio-napi');
/// const escaped = shell_escape_args(["arg1", "arg 2"]);
/// ```
#[napi]
pub fn shell_escape_args(args: Vec<String>) -> Vec<String> {
    args.into_iter().map(shell_escape).collect()
}

/// Kill a process by PID.
///
/// # Arguments
/// * `pid` - Process ID to kill
/// * `signal` - Optional signal name ("SIGKILL", "SIGTERM", "SIGINT", "SIGHUP")
///
/// # Returns
/// * `Result<bool, napi::Error>` - True if successful
///
/// # Example
/// ```javascript
/// const { kill_process } = require('stdio-napi');
/// await kill_process(12345, "SIGTERM");
/// ```
#[napi]
pub async fn kill_process(pid: u32, signal: Option<String>) -> napi::Result<bool> {
    #[cfg(unix)]
    {
        let sig = match signal.as_deref() {
            Some("SIGKILL") | Some("KILL") => libc::SIGKILL,
            Some("SIGTERM") | Some("TERM") => libc::SIGTERM,
            Some("SIGINT") | Some("INT") => libc::SIGINT,
            Some("SIGHUP") | Some("HUP") => libc::SIGHUP,
            _ => libc::SIGTERM,
        };

        unsafe {
            let result = libc::kill(pid as libc::pid_t, sig);
            if result == 0 {
                Ok(true)
            } else {
                Err(napi::Error::from_reason(format!("Failed to kill process {}: {}", pid, errno::errno())))
            }
        }
    }

    #[cfg(windows)]
    {
        let force_flag = if signal.as_deref() == Some("KILL") { "/F" } else { "" };
        let output = StdCommand::new("taskkill")
            .args(&[force_flag, "/PID", &pid.to_string()])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to kill process {}: {}", pid, e)))?;

        Ok(output.status.success())
    }
}

/// Wait for a process to complete (by PID)
#[napi]
pub async fn wait_for_process(_pid: u32) -> napi::Result<ProcessStatus> {
    Err(napi::Error::from_reason(
        "wait_for_process requires the Child object. Use spawn_with_pipes and keep the reference.".to_string()
    ))
}

/// Read stdout from a spawned process
#[napi]
pub async fn read_process_stdout(_pid: u32) -> napi::Result<String> {
    Err(napi::Error::from_reason(
        "Use spawn_with_pipes and read from the Child's stdout directly.".to_string()
    ))
}

/// Read stderr from a spawned process
#[napi]
pub async fn read_process_stderr(_pid: u32) -> napi::Result<String> {
    Err(napi::Error::from_reason(
        "Use spawn_with_pipes and read from the Child's stderr directly.".to_string()
    ))
}

/// Write to stdin of a spawned process
#[napi]
pub async fn write_process_stdin(_pid: u32, _input: String) -> napi::Result<()> {
    Err(napi::Error::from_reason(
        "Use spawn_with_pipes and write to the Child's stdin directly.".to_string()
    ))
}
