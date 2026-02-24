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

/// Buffer configuration for stream handling
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
        if let Some(ref mut child) = *child_guard {
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
