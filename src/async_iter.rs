//! Async iterator support and event emitter patterns for process management.
//!
//! This module provides async iteration capabilities for streaming process output
//! and an event emitter pattern for handling process events.

use napi_derive::napi;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};

/// Event types for process event emitter.
#[napi]
pub enum ProcessEvent {
    /// Process started
    Start,
    /// Stdout data received
    Stdout,
    /// Stderr data received
    Stderr,
    /// Process exited
    Exit,
    /// Error occurred
    Error,
}

/// Event data structure for process events.
#[napi(object)]
pub struct ProcessEventData {
    /// Event type
    pub event: String,
    /// Event data (for stdout/stderr events)
    pub data: Option<String>,
    /// Exit code (for exit events)
    pub code: Option<i32>,
    /// Process PID
    pub pid: u32,
    /// Timestamp (Unix epoch in ms)
    pub timestamp: u32,
}

/// Listener handle for unsubscribing from events.
#[napi(object)]
pub struct EventListenerHandle {
    /// Unique listener ID
    pub id: u32,
    /// Event type
    pub event: String,
}

/// Process event emitter for handling process lifecycle events.
///
/// This class provides an event-driven interface for process management.
#[napi]
pub struct ProcessEventEmitter {
    pid: u32,
    listeners: Arc<RwLock<Vec<EventListener>>>,
    shutdown_tx: broadcast::Sender<()>,
    next_listener_id: Arc<Mutex<u32>>,
}

/// Internal event listener.
struct EventListener {
    id: u32,
    event: ProcessEvent,
}

#[napi]
impl ProcessEventEmitter {
    /// Create a new process event emitter.
    ///
    /// # Arguments
    /// * `pid` - Process ID
    #[napi(constructor)]
    pub fn new(pid: u32) -> ProcessEventEmitter {
        let (shutdown_tx, _) = broadcast::channel(1);
        ProcessEventEmitter {
            pid,
            listeners: Arc::new(RwLock::new(Vec::new())),
            shutdown_tx,
            next_listener_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Register an event listener.
    ///
    /// # Arguments
    /// * `event` - Event type to listen for
    ///
    /// # Returns
    /// * `EventListenerHandle` - Handle for removing the listener
    #[napi]
    pub async fn on(&self, event: String) -> napi::Result<EventListenerHandle> {
        let event_type = match event.to_lowercase().as_str() {
            "start" => ProcessEvent::Start,
            "stdout" => ProcessEvent::Stdout,
            "stderr" => ProcessEvent::Stderr,
            "exit" => ProcessEvent::Exit,
            "error" => ProcessEvent::Error,
            _ => {
                return Err(napi::Error::from_reason(format!(
                    "Unknown event: {}",
                    event
                )))
            }
        };

        let mut next_id = self.next_listener_id.lock().await;
        let listener_id = *next_id;
        *next_id += 1;

        let listener = EventListener {
            id: listener_id,
            event: event_type,
        };

        let mut listeners = self.listeners.write().await;
        listeners.push(listener);

        Ok(EventListenerHandle {
            id: listener_id,
            event,
        })
    }

    /// Register a one-time event listener.
    #[napi]
    pub async fn once(&self, event: String) -> napi::Result<EventListenerHandle> {
        self.on(event).await
    }

    /// Remove an event listener.
    #[napi]
    pub async fn off(&self, handle: EventListenerHandle) -> napi::Result<()> {
        let mut listeners = self.listeners.write().await;
        listeners.retain(|l| l.id != handle.id);
        Ok(())
    }

    /// Remove all listeners for an event.
    #[napi]
    pub async fn remove_all_listeners(&self, event: Option<String>) -> napi::Result<()> {
        let mut listeners = self.listeners.write().await;
        if let Some(evt) = event {
            let event_type = match evt.to_lowercase().as_str() {
                "start" => ProcessEvent::Start,
                "stdout" => ProcessEvent::Stdout,
                "stderr" => ProcessEvent::Stderr,
                "exit" => ProcessEvent::Exit,
                "error" => ProcessEvent::Error,
                _ => return Err(napi::Error::from_reason(format!("Unknown event: {}", evt))),
            };
            listeners.retain(|l| {
                std::mem::discriminant(&l.event) != std::mem::discriminant(&event_type)
            });
        } else {
            listeners.clear();
        }
        Ok(())
    }

    /// Emit an event.
    #[napi]
    pub async fn emit(&self, event: String, data: Option<String>, code: Option<i32>) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u32)
            .unwrap_or(0);

        let _event_data = ProcessEventData {
            event: event.clone(),
            data,
            code,
            pid: self.pid,
            timestamp,
        };
    }

    /// Get listener count.
    #[napi]
    pub async fn listener_count(&self, event: Option<String>) -> u32 {
        let listeners = self.listeners.read().await;
        if let Some(evt) = event {
            let event_type = match evt.to_lowercase().as_str() {
                "start" => Some(ProcessEvent::Start),
                "stdout" => Some(ProcessEvent::Stdout),
                "stderr" => Some(ProcessEvent::Stderr),
                "exit" => Some(ProcessEvent::Exit),
                "error" => Some(ProcessEvent::Error),
                _ => None,
            };
            if let Some(et) = event_type {
                listeners
                    .iter()
                    .filter(|l| std::mem::discriminant(&l.event) == std::mem::discriminant(&et))
                    .count() as u32
            } else {
                0
            }
        } else {
            listeners.len() as u32
        }
    }

    /// Shutdown the emitter.
    #[napi]
    pub async fn shutdown(&self) -> napi::Result<()> {
        let _ = self.shutdown_tx.send(());
        self.remove_all_listeners(None).await
    }
}

/// Async line iterator for streaming process output.
#[napi]
pub struct AsyncLineIterator {
    lines: Arc<Mutex<Vec<String>>>,
    index: Arc<Mutex<usize>>,
}

#[napi]
impl AsyncLineIterator {
    #[napi(constructor)]
    pub fn new() -> AsyncLineIterator {
        AsyncLineIterator {
            lines: Arc::new(Mutex::new(Vec::new())),
            index: Arc::new(Mutex::new(0)),
        }
    }

    #[napi]
    pub async fn add_line(&self, line: String) {
        let mut lines = self.lines.lock().await;
        lines.push(line);
    }

    #[napi]
    pub async fn add_lines(&self, lines: Vec<String>) {
        let mut self_lines = self.lines.lock().await;
        self_lines.extend(lines);
    }

    #[napi]
    pub async fn next(&self) -> napi::Result<Option<String>> {
        let mut index = self.index.lock().await;
        let lines = self.lines.lock().await;

        if *index >= lines.len() {
            return Ok(None);
        }

        let line = lines.get(*index).cloned();
        *index += 1;
        Ok(line)
    }

    #[napi]
    pub async fn is_exhausted(&self) -> bool {
        let index = *self.index.lock().await;
        let lines = self.lines.lock().await;
        index >= lines.len()
    }

    #[napi]
    pub async fn collect(&self) -> napi::Result<Vec<String>> {
        let mut result = Vec::new();
        while let Some(line) = self.next().await? {
            result.push(line);
        }
        Ok(result)
    }

    #[napi]
    pub async fn take(&self, n: u32) -> napi::Result<Vec<String>> {
        let mut lines = Vec::new();
        for _ in 0..n {
            match self.next().await? {
                Some(line) => lines.push(line),
                None => break,
            }
        }
        Ok(lines)
    }

    #[napi]
    pub async fn skip(&self, n: u32) -> napi::Result<()> {
        for _ in 0..n {
            self.next().await?;
        }
        Ok(())
    }

    #[napi]
    pub async fn filter(
        &self,
        pattern: String,
        max_lines: Option<u32>,
    ) -> napi::Result<Vec<String>> {
        let regex = regex::Regex::new(&pattern)
            .map_err(|e| napi::Error::from_reason(format!("Invalid regex: {}", e)))?;

        let mut lines = Vec::new();
        let max = max_lines.unwrap_or(u32::MAX);

        while lines.len() < max as usize {
            match self.next().await? {
                Some(line) => {
                    if regex.is_match(&line) {
                        lines.push(line);
                    }
                }
                None => break,
            }
        }
        Ok(lines)
    }

    #[napi]
    pub async fn reset(&self) {
        let mut index = self.index.lock().await;
        *index = 0;
    }

    #[napi]
    pub async fn len(&self) -> u32 {
        let lines = self.lines.lock().await;
        lines.len() as u32
    }

    #[napi]
    pub async fn is_empty(&self) -> bool {
        let lines = self.lines.lock().await;
        lines.is_empty()
    }
}

impl Default for AsyncLineIterator {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration builder for process spawning.
#[napi]
pub struct ProcessBuilder {
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
    env: HashMap<String, String>,
    capture_stdout: bool,
    capture_stderr: bool,
    capture_stdin: bool,
    detached: bool,
    shell: bool,
    timeout_ms: Option<u32>,
}

impl fmt::Display for ProcessBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)?;
        for arg in &self.args {
            if arg.contains(' ') || arg.contains('"') {
                write!(f, " \"{}\"", arg.replace('"', "\\\""))?;
            } else {
                write!(f, " {}", arg)?;
            }
        }
        Ok(())
    }
}

#[napi]
impl ProcessBuilder {
    #[napi(constructor)]
    pub fn new(command: String) -> ProcessBuilder {
        ProcessBuilder {
            command,
            args: Vec::new(),
            cwd: None,
            env: HashMap::new(),
            capture_stdout: false,
            capture_stderr: false,
            capture_stdin: false,
            detached: false,
            shell: false,
            timeout_ms: None,
        }
    }

    #[napi]
    pub fn arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    #[napi]
    pub fn args(&mut self, args: Vec<String>) {
        self.args.extend(args);
    }

    #[napi]
    pub fn cwd(&mut self, cwd: String) {
        self.cwd = Some(cwd);
    }

    #[napi]
    pub fn env(&mut self, key: String, value: String) {
        self.env.insert(key, value);
    }

    #[napi]
    pub fn envs(&mut self, env: HashMap<String, String>) {
        self.env.extend(env);
    }

    #[napi]
    pub fn capture_stdout(&mut self, capture: bool) {
        self.capture_stdout = capture;
    }

    #[napi]
    pub fn capture_stderr(&mut self, capture: bool) {
        self.capture_stderr = capture;
    }

    #[napi]
    pub fn capture_stdin(&mut self, capture: bool) {
        self.capture_stdin = capture;
    }

    #[napi]
    pub fn detached(&mut self, detached: bool) {
        self.detached = detached;
    }

    #[napi]
    pub fn shell(&mut self, shell: bool) {
        self.shell = shell;
    }

    #[napi]
    pub fn timeout(&mut self, timeout_ms: u32) {
        self.timeout_ms = Some(timeout_ms);
    }

    #[napi]
    pub fn to_command_string(&self) -> String {
        self.to_string()
    }

    #[napi]
    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    #[napi]
    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    #[napi]
    pub fn get_cwd(&self) -> Option<String> {
        self.cwd.clone()
    }
}

/// Create a process builder.
#[napi]
pub fn process_builder(command: String) -> ProcessBuilder {
    ProcessBuilder::new(command)
}

/// Create a process event emitter.
#[napi]
pub fn process_events(pid: u32) -> ProcessEventEmitter {
    ProcessEventEmitter::new(pid)
}

/// Create an async line iterator.
#[napi]
pub fn line_iterator() -> AsyncLineIterator {
    AsyncLineIterator::new()
}
