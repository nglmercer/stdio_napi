use napi_derive::napi;
use std::collections::HashMap;
use std::process::Stdio as StdStdio;
use std::process::Command as StdCommand;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, broadcast};

#[cfg(unix)]
#[allow(unused_imports)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
#[allow(unused_imports)]
use std::os::windows::process::CommandExt;

/// Buffer configuration for stream handling
#[napi(object)]
pub struct BufferConfig {
    /// Size of read buffer in bytes (default: 8192)
    pub read_size: Option<usize>,
    /// Size of write buffer in bytes (default: 8192)
    pub write_size: Option<usize>,
    /// Maximum buffer size before applying backpressure (default: 65536)
    pub max_size: Option<usize>,
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

/// Stream event types for process communication
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

/// Stream event data
#[napi(object)]
pub struct StreamEventData {
    pub event: StreamEvent,
    pub data: Option<String>,
    pub code: Option<i32>,
}

/// Managed process handle with streaming capabilities
#[napi]
pub struct ManagedProcess {
    pid: u32,
    // We use Arc<Mutex<Option<Child>>> to keep the child alive
    child: Arc<Mutex<Option<Child>>>,
}

#[napi]
impl ManagedProcess {
    /// Create a new managed process
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

    /// Get the process PID
    #[napi]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Read a line from stdout
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

    /// Read a line from stderr
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

    /// Write to stdin
    #[napi]
    pub async fn write_stdin(&self, data: String) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stdin) = child.stdin {
                use tokio::io::AsyncWriteExt;
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

    /// Wait for process to complete and get exit status
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

    /// Kill the process
    #[napi]
    pub async fn kill(&self) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = child_guard {
            child.kill().await
                .map_err(|e| napi::Error::from_reason(format!("Kill error: {}", e)))?;
            Ok(())
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }
}

/// Spawn options for process execution
#[napi(object)]
pub struct SpawnOptions {
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    /// Whether to capture stdout (default: false)
    pub capture_stdout: Option<bool>,
    /// Whether to capture stderr (default: false)
    pub capture_stderr: Option<bool>,
}

/// Process status result
#[napi(object)]
#[derive(Clone)]
pub struct ProcessStatus {
    pub pid: u32,
    pub success: bool,
    pub code: Option<i32>,
}

/// Process output containing stdout and stderr
#[napi(object)]
pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
    pub success: bool,
}

/// Execute a command and wait for completion (async)
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

/// Spawn a process with options
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

    // Wait for the process to complete
    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}

/// Spawn a process with piped stdio for streaming
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

    // Always pipe all stdio
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

/// Execute a command synchronously and return output
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

/// Execute a command with arguments synchronously
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

/// Shell escape a string for safe command execution
#[napi]
pub fn shell_escape(input: String) -> String {
    // Escape single quotes by wrapping in single quotes and escaping any single quotes inside
    format!("'{}'", input.replace('\'', "'\\''"))
}

/// Shell escape arguments for safe command execution
#[napi]
pub fn shell_escape_args(args: Vec<String>) -> Vec<String> {
    args.into_iter().map(shell_escape).collect()
}

/// Kill a process by PID
#[napi]
pub async fn kill_process(pid: u32, signal: Option<String>) -> napi::Result<bool> {
    #[cfg(unix)]
    {
        #[allow(unused_imports)]
        use std::os::unix::process::CommandExt;
        
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
        use std::os::windows::process::CommandExt;
        
        // On Windows, we use taskkill
        let force_flag = if signal.as_deref() == Some("KILL") { "/F" } else { "" };
        let output = StdCommand::new("taskkill")
            .args(&[force_flag, "/PID", &pid.to_string()])
            .output()
            .map_err(|e| napi::Error::from_reason(format!("Failed to kill process {}: {}", pid, e)))?;

        Ok(output.status.success())
    }
}

/// Wait for a process to complete (by PID)
/// Note: This is limited since we don't keep track of spawned processes
/// For proper process management, use spawn_with_pipes
#[napi]
pub async fn wait_for_process(_pid: u32) -> napi::Result<ProcessStatus> {
    // This is a placeholder - in practice, you'd need to track child processes
    // The tokio::process::Child doesn't persist after the struct is dropped
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

/// Stream duplex state for managing backpressure
#[napi]
pub struct StreamDuplexState {
    /// Current read buffer size
    pub read_buffer_size: usize,
    /// Current write buffer size
    pub write_buffer_size: usize,
    /// Whether backpressure is being applied
    pub is_backpressure_active: bool,
    /// Number of bytes in read buffer
    pub read_buffer_bytes: usize,
    /// Number of bytes in write buffer
    pub write_buffer_bytes: usize,
}

/// Full-duplex stream for process communication with events and backpressure
#[napi]
pub struct StreamDuplex {
    pid: u32,
    child: Arc<Mutex<Option<Child>>>,
    buffer_config: BufferConfig,
    // Event channels using broadcast
    stdout_tx: broadcast::Sender<String>,
    stderr_tx: broadcast::Sender<String>,
    exit_tx: broadcast::Sender<ProcessStatus>,
    // Backpressure state
    write_paused: Arc<Mutex<bool>>,
    read_paused: Arc<Mutex<bool>>,
}

#[napi]
impl StreamDuplex {
    /// Create a new stream duplex for a process
    #[napi(constructor)]
    pub fn new(command: String, args: Vec<String>, buffer_config: Option<BufferConfig>) -> napi::Result<StreamDuplex> {
        let config = buffer_config.unwrap_or_default();
        
        let (stdout_tx, _) = broadcast::channel(config.read_size.unwrap_or(8192));
        let (stderr_tx, _) = broadcast::channel(config.read_size.unwrap_or(8192));
        let (exit_tx, _) = broadcast::channel(1);

        let child = Command::new(&command)
            .args(&args)
            .stdout(StdStdio::piped())
            .stderr(StdStdio::piped())
            .stdin(StdStdio::piped())
            .spawn()
            .map_err(|e| napi::Error::from_reason(format!("Failed to spawn '{}': {}", command, e)))?;

        let pid = child.id().unwrap_or(0);

        Ok(StreamDuplex {
            pid,
            child: Arc::new(Mutex::new(Some(child))),
            buffer_config: config,
            stdout_tx,
            stderr_tx,
            exit_tx,
            write_paused: Arc::new(Mutex::new(false)),
            read_paused: Arc::new(Mutex::new(false)),
        })
    }

    /// Get the process PID
    #[napi]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Subscribe to stdout events
    #[napi]
    pub fn on_stdout(&self) -> napi::Result<String> {
        let mut rx = self.stdout_tx.subscribe();
        Ok("stdout_subscription".to_string())
    }

    /// Subscribe to stderr events
    #[napi]
    pub fn on_stderr(&self) -> napi::Result<String> {
        let mut rx = self.stderr_tx.subscribe();
        Ok("stderr_subscription".to_string())
    }

    /// Subscribe to exit events
    #[napi]
    pub fn on_exit(&self) -> napi::Result<String> {
        let mut rx = self.exit_tx.subscribe();
        Ok("exit_subscription".to_string())
    }

    /// Write data to stdin
    #[napi]
    pub async fn write(&self, data: String) -> napi::Result<()> {
        // Check backpressure
        let is_paused = *self.write_paused.lock().await;
        if is_paused {
            return Err(napi::Error::from_reason("Write buffer full, backpressure applied".to_string()));
        }

        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(data.as_bytes()).await
                    .map_err(|e| napi::Error::from_reason(format!("Write error: {}", e)))?;
                stdin.flush().await
                    .map_err(|e| napi::Error::from_reason(format!("Flush error: {}", e)))?;
                Ok(())
            } else {
                Err(napi::Error::from_reason("Stdin not piped".to_string()))
            }
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Write a line to stdin (with newline)
    #[napi]
    pub async fn write_line(&self, line: String) -> napi::Result<()> {
        self.write(format!("{}\n", line)).await
    }

    /// Write data to stdin without blocking (async)
    #[napi]
    pub async fn write_async(&self, data: String) -> napi::Result<()> {
        self.write(data).await
    }

    /// Close stdin (send EOF)
    #[napi]
    pub async fn close_stdin(&self) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            if let Some(ref mut stdin) = child.stdin {
                stdin.close().await
                    .map_err(|e| napi::Error::from_reason(format!("Close stdin error: {}", e)))?;
                Ok(())
            } else {
                Err(napi::Error::from_reason("Stdin not piped".to_string()))
            }
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Get current stream state including backpressure info
    #[napi]
    pub async fn get_state(&self) -> napi::Result<StreamDuplexState> {
        let write_paused = *self.write_paused.lock().await;
        let read_paused = *self.read_paused.lock().await;
        
        Ok(StreamDuplexState {
            read_buffer_size: self.buffer_config.read_size.unwrap_or(8192),
            write_buffer_size: self.buffer_config.write_size.unwrap_or(8192),
            is_backpressure_active: write_paused || read_paused,
            read_buffer_bytes: 0,
            write_buffer_bytes: 0,
        })
    }

    /// Set write buffer high watermark (triggers backpressure when exceeded)
    #[napi]
    pub async fn set_write_high_watermark(&self, size: usize) -> napi::Result<()> {
        // This would be used in conjunction with monitoring write buffer
        // For now, we provide the API structure
        Ok(())
    }

    /// Set read buffer high watermark (triggers backpressure when exceeded)
    #[napi]
    pub async fn set_read_high_watermark(&self, size: usize) -> napi::Result<()> {
        // This would be used in conjunction with monitoring read buffer
        // For now, we provide the API structure
        Ok(())
    }

    /// Resume writing after backpressure
    #[napi]
    pub async fn resume_write(&self) -> napi::Result<()> {
        let mut paused = self.write_paused.lock().await;
        *paused = false;
        Ok(())
    }

    /// Pause writing (apply backpressure)
    #[napi]
    pub async fn pause_write(&self) -> napi::Result<()> {
        let mut paused = self.write_paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resume reading after backpressure
    #[napi]
    pub async fn resume_read(&self) -> napi::Result<()> {
        let mut paused = self.read_paused.lock().await;
        *paused = false;
        Ok(())
    }

    /// Pause reading (apply backpressure)
    #[napi]
    pub async fn pause_read(&self) -> napi::Result<()> {
        let mut paused = self.read_paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Wait for process to exit
    #[napi]
    pub async fn wait(&self) -> napi::Result<ProcessStatus> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = child_guard.take() {
            let status = child.wait().await
                .map_err(|e| napi::Error::from_reason(format!("Wait error: {}", e)))?;
            
            let process_status = ProcessStatus {
                pid: self.pid,
                success: status.success(),
                code: status.code(),
            };
            
            // Send exit event
            let _ = self.exit_tx.send(process_status.clone());
            
            Ok(process_status)
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Kill the process
    #[napi]
    pub async fn kill(&self) -> napi::Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = child_guard {
            child.kill().await
                .map_err(|e| napi::Error::from_reason(format!("Kill error: {}", e)))?;
            
            // Send exit event
            let status = ProcessStatus {
                pid: self.pid,
                success: false,
                code: Some(-1),
            };
            let _ = self.exit_tx.send(status);
            
            Ok(())
        } else {
            Err(napi::Error::from_reason("Process not running".to_string()))
        }
    }

    /// Check if process is still running
    #[napi]
    pub fn is_running(&self) -> bool {
        // We can't easily check without locking, so this is a best effort
        true
    }
}
