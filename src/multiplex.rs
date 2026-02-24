//! Terminal multiplexing support for managing multiple terminal sessions.
//!
//! This module provides tmux-like functionality for creating and managing
//! multiple terminal sessions, windows, and panes.

use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Unique identifier for a session.
pub type SessionId = String;

/// Unique identifier for a window.
pub type WindowId = String;

/// Unique identifier for a pane.
pub type PaneId = String;

/// Terminal pane representing a single terminal view.
#[napi(object)]
#[derive(Clone)]
pub struct Pane {
    /// Unique pane identifier
    pub id: String,
    /// Parent window ID
    pub window_id: String,
    /// Pane index in the window
    pub index: u32,
    /// Current working directory
    pub cwd: Option<String>,
    /// Pane title
    pub title: Option<String>,
    /// Whether the pane is active
    pub active: bool,
    /// Pane width in columns
    pub width: u16,
    /// Pane height in rows
    pub height: u16,
    /// Pane X position in window
    pub x: u16,
    /// Pane Y position in window
    pub y: u16,
    /// Process ID running in pane
    pub pid: Option<u32>,
}

/// Terminal window containing one or more panes.
#[napi(object)]
#[derive(Clone)]
pub struct Window {
    /// Unique window identifier
    pub id: String,
    /// Parent session ID
    pub session_id: String,
    /// Window index in session
    pub index: u32,
    /// Window name
    pub name: Option<String>,
    /// Window width in columns
    pub width: u16,
    /// Window height in rows
    pub height: u16,
    /// List of pane IDs in this window
    pub panes: Vec<String>,
    /// Currently active pane ID
    pub active_pane: Option<String>,
}

/// Terminal session containing one or more windows.
#[napi(object)]
#[derive(Clone)]
pub struct Session {
    /// Unique session identifier
    pub id: String,
    /// Session name
    pub name: String,
    /// Creation timestamp (Unix epoch)
    pub created_at: u32,
    /// Last activity timestamp (Unix epoch)
    pub last_activity: u32,
    /// List of window IDs in this session
    pub windows: Vec<String>,
    /// Currently active window ID
    pub active_window: Option<String>,
    /// Session width in columns
    pub width: u16,
    /// Session height in rows
    pub height: u16,
    /// Whether the session is attached
    pub attached: bool,
}

/// Layout direction for pane splitting.
#[napi]
pub enum LayoutDirection {
    /// Split horizontally (side by side)
    Horizontal,
    /// Split vertically (top and bottom)
    Vertical,
}

/// Session creation options.
#[napi(object)]
pub struct SessionOptions {
    /// Session name (required)
    pub name: String,
    /// Initial working directory
    pub cwd: Option<String>,
    /// Initial window name
    pub window_name: Option<String>,
    /// Session width in columns (default: terminal width)
    pub width: Option<u16>,
    /// Session height in rows (default: terminal height)
    pub height: Option<u16>,
    /// Shell to use (default: $SHELL or /bin/sh)
    pub shell: Option<String>,
}

/// Window creation options.
#[napi(object)]
pub struct WindowOptions {
    /// Window name
    pub name: Option<String>,
    /// Working directory
    pub cwd: Option<String>,
    /// Command to run
    pub command: Option<String>,
    /// Window width
    pub width: Option<u16>,
    /// Window height
    pub height: Option<u16>,
}

/// Pane creation options.
#[napi(object)]
pub struct PaneOptions {
    /// Working directory
    pub cwd: Option<String>,
    /// Command to run in pane
    pub command: Option<String>,
    /// Split direction
    pub split: Option<String>,
    /// Percentage of window to use (default: 50)
    pub percentage: Option<u8>,
}

/// Internal state for a multiplexer session.
struct SessionState {
    info: Session,
    windows: HashMap<WindowId, WindowState>,
}

/// Internal state for a window.
struct WindowState {
    info: Window,
    panes: HashMap<PaneId, Pane>,
}

/// Terminal multiplexer for managing sessions, windows, and panes.
///
/// This class provides tmux-like functionality for terminal management.
#[napi]
pub struct TerminalMultiplexer {
    sessions: Arc<RwLock<HashMap<SessionId, SessionState>>>,
    active_session: Arc<Mutex<Option<SessionId>>>,
}

#[napi]
impl TerminalMultiplexer {
    /// Create a new terminal multiplexer.
    ///
    /// # Example
    /// ```javascript
    /// const { TerminalMultiplexer } = require('stdio-napi');
    /// const mux = new TerminalMultiplexer();
    /// ```
    #[napi(constructor)]
    pub fn new() -> TerminalMultiplexer {
        TerminalMultiplexer {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_session: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new session.
    ///
    /// # Arguments
    /// * `options` - Session creation options
    ///
    /// # Returns
    /// * `Session` - The created session
    #[napi]
    pub async fn create_session(&self, options: SessionOptions) -> napi::Result<Session> {
        let session_id = generate_id("s");
        let window_id = generate_id("w");
        let pane_id = generate_id("p");

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        let width = options.width.unwrap_or(80);
        let height = options.height.unwrap_or(24);

        // Create initial pane
        let pane = Pane {
            id: pane_id.clone(),
            window_id: window_id.clone(),
            index: 0,
            cwd: options.cwd.clone(),
            title: None,
            active: true,
            width,
            height,
            x: 0,
            y: 0,
            pid: None,
        };

        // Create initial window
        let window = Window {
            id: window_id.clone(),
            session_id: session_id.clone(),
            index: 0,
            name: options.window_name.clone(),
            width,
            height,
            panes: vec![pane_id.clone()],
            active_pane: Some(pane_id.clone()),
        };

        // Create session
        let session = Session {
            id: session_id.clone(),
            name: options.name,
            created_at: now,
            last_activity: now,
            windows: vec![window_id.clone()],
            active_window: Some(window_id.clone()),
            width,
            height,
            attached: false,
        };

        // Store in state
        let mut panes = HashMap::new();
        panes.insert(pane_id, pane);

        let mut windows = HashMap::new();
        windows.insert(
            window_id,
            WindowState {
                info: window,
                panes,
            },
        );

        let mut sessions = self.sessions.write().await;
        sessions.insert(
            session_id.clone(),
            SessionState {
                info: session.clone(),
                windows,
            },
        );

        Ok(session)
    }

    /// List all sessions.
    ///
    /// # Returns
    /// * `Vec<Session>` - List of all sessions
    #[napi]
    pub async fn list_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.info.clone()).collect()
    }

    /// Get a session by ID.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    ///
    /// # Returns
    /// * `Option<Session>` - The session if found
    #[napi]
    pub async fn get_session(&self, session_id: String) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).map(|s| s.info.clone())
    }

    /// Attach to a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID to attach to
    #[napi]
    pub async fn attach_session(&self, session_id: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.info.attached = true;
            session.info.last_activity = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as u32)
                .unwrap_or(0);
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Detach from a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID to detach from
    #[napi]
    pub async fn detach_session(&self, session_id: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.info.attached = false;
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Kill a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID to kill
    #[napi]
    pub async fn kill_session(&self, session_id: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;
        if sessions.remove(&session_id).is_some() {
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Create a new window in a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `options` - Window creation options
    ///
    /// # Returns
    /// * `Window` - The created window
    #[napi]
    pub async fn create_window(
        &self,
        session_id: String,
        options: WindowOptions,
    ) -> napi::Result<Window> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window_id = generate_id("w");
        let pane_id = generate_id("p");

        let width = options.width.unwrap_or(session.info.width);
        let height = options.height.unwrap_or(session.info.height);

        let pane = Pane {
            id: pane_id.clone(),
            window_id: window_id.clone(),
            index: 0,
            cwd: options.cwd.clone(),
            title: None,
            active: true,
            width,
            height,
            x: 0,
            y: 0,
            pid: None,
        };

        let window = Window {
            id: window_id.clone(),
            session_id: session_id.clone(),
            index: session.windows.len() as u32,
            name: options.name,
            width,
            height,
            panes: vec![pane_id.clone()],
            active_pane: Some(pane_id.clone()),
        };

        session.info.windows.push(window_id.clone());
        session.info.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        let mut panes = HashMap::new();
        panes.insert(pane_id, pane);

        session.windows.insert(
            window_id.clone(),
            WindowState {
                info: window.clone(),
                panes,
            },
        );

        Ok(window)
    }

    /// List windows in a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    ///
    /// # Returns
    /// * `Vec<Window>` - List of windows
    #[napi]
    pub async fn list_windows(&self, session_id: String) -> napi::Result<Vec<Window>> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        Ok(session.windows.values().map(|w| w.info.clone()).collect())
    }

    /// Get a window by ID.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    ///
    /// # Returns
    /// * `Option<Window>` - The window if found
    #[napi]
    pub async fn get_window(&self, session_id: String, window_id: String) -> Option<Window> {
        let sessions = self.sessions.read().await;
        sessions
            .get(&session_id)
            .and_then(|s| s.windows.get(&window_id))
            .map(|w| w.info.clone())
    }

    /// Kill a window.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID to kill
    #[napi]
    pub async fn kill_window(&self, session_id: String, window_id: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        if session.windows.remove(&window_id).is_some() {
            session.info.windows.retain(|id| id != &window_id);
            if session.info.active_window.as_ref() == Some(&window_id) {
                session.info.active_window = session.info.windows.first().cloned();
            }
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Window not found: {}",
                window_id
            )))
        }
    }

    /// Select (activate) a window.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID to select
    #[napi]
    pub async fn select_window(&self, session_id: String, window_id: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        if session.windows.contains_key(&window_id) {
            session.info.active_window = Some(window_id);
            session.info.last_activity = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u32)
                .unwrap_or(0);
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Window not found: {}",
                window_id
            )))
        }
    }

    /// Split a pane (create a new pane by splitting existing one).
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane to split
    /// * `options` - Pane creation options
    ///
    /// # Returns
    /// * `Pane` - The newly created pane
    #[napi]
    pub async fn split_pane(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
        options: PaneOptions,
    ) -> napi::Result<Pane> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        let existing_pane = window
            .panes
            .get(&pane_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Pane not found: {}", pane_id)))?
            .clone();

        let new_pane_id = generate_id("p");
        let percentage = options.percentage.unwrap_or(50).min(100).max(1);
        let split_horizontal = options.split.as_deref() == Some("horizontal");

        let (existing_width, existing_height, new_width, new_height, new_x, new_y) =
            if split_horizontal {
                let new_width = existing_pane.width * percentage as u16 / 100;
                let existing_width = existing_pane.width - new_width;
                (
                    existing_width,
                    existing_pane.height,
                    new_width,
                    existing_pane.height,
                    existing_pane.x + existing_width,
                    existing_pane.y,
                )
            } else {
                let new_height = existing_pane.height * percentage as u16 / 100;
                let existing_height = existing_pane.height - new_height;
                (
                    existing_pane.width,
                    existing_height,
                    existing_pane.width,
                    new_height,
                    existing_pane.x,
                    existing_pane.y + existing_height,
                )
            };

        // Update existing pane
        if let Some(p) = window.panes.get_mut(&pane_id) {
            p.width = existing_width;
            p.height = existing_height;
            p.active = false;
        }

        // Create new pane
        let new_pane = Pane {
            id: new_pane_id.clone(),
            window_id: window_id.clone(),
            index: window.panes.len() as u32,
            cwd: options.cwd,
            title: None,
            active: true,
            width: new_width,
            height: new_height,
            x: new_x,
            y: new_y,
            pid: None,
        };

        window.info.panes.push(new_pane_id.clone());
        window.info.active_pane = Some(new_pane_id.clone());

        window.panes.insert(new_pane_id.clone(), new_pane.clone());

        Ok(new_pane)
    }

    /// List panes in a window.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    ///
    /// # Returns
    /// * `Vec<Pane>` - List of panes
    #[napi]
    pub async fn list_panes(
        &self,
        session_id: String,
        window_id: String,
    ) -> napi::Result<Vec<Pane>> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        Ok(window.panes.values().cloned().collect())
    }

    /// Get a pane by ID.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane ID
    ///
    /// # Returns
    /// * `Option<Pane>` - The pane if found
    #[napi]
    pub async fn get_pane(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
    ) -> Option<Pane> {
        let sessions = self.sessions.read().await;
        sessions
            .get(&session_id)
            .and_then(|s| s.windows.get(&window_id))
            .and_then(|w| w.panes.get(&pane_id))
            .cloned()
    }

    /// Kill a pane.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane ID to kill
    #[napi]
    pub async fn kill_pane(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
    ) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        if window.panes.remove(&pane_id).is_some() {
            window.info.panes.retain(|id| id != &pane_id);
            if window.info.active_pane.as_ref() == Some(&pane_id) {
                window.info.active_pane = window.info.panes.first().cloned();
            }
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Pane not found: {}",
                pane_id
            )))
        }
    }

    /// Select (activate) a pane.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane ID to select
    #[napi]
    pub async fn select_pane(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
    ) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        if window.panes.contains_key(&pane_id) {
            // Deactivate all panes
            for pane in window.panes.values_mut() {
                pane.active = false;
            }
            // Activate selected pane
            if let Some(p) = window.panes.get_mut(&pane_id) {
                p.active = true;
            }
            window.info.active_pane = Some(pane_id);
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Pane not found: {}",
                pane_id
            )))
        }
    }

    /// Resize a pane.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane ID to resize
    /// * `width` - New width (optional)
    /// * `height` - New height (optional)
    #[napi]
    pub async fn resize_pane(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
        width: Option<u16>,
        height: Option<u16>,
    ) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;

        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        if let Some(p) = window.panes.get_mut(&pane_id) {
            if let Some(w) = width {
                p.width = w;
            }
            if let Some(h) = height {
                p.height = h;
            }
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Pane not found: {}",
                pane_id
            )))
        }
    }

    /// Send keys/text to a pane.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `pane_id` - The pane ID
    /// * `keys` - The keys/text to send
    #[napi]
    pub async fn send_keys(
        &self,
        session_id: String,
        window_id: String,
        pane_id: String,
        keys: String,
    ) -> napi::Result<()> {
        let sessions = self.sessions.read().await;

        let session = sessions.get(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        if window.panes.contains_key(&pane_id) {
            // In a real implementation, this would send keys to the process
            // For now, we just validate the pane exists
            let _ = keys;
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Pane not found: {}",
                pane_id
            )))
        }
    }

    /// Get the active session.
    ///
    /// # Returns
    /// * `Option<Session>` - The active session
    #[napi]
    pub async fn get_active_session(&self) -> Option<Session> {
        let active = self.active_session.lock().await;
        if let Some(ref session_id) = *active {
            let sessions = self.sessions.read().await;
            sessions.get(session_id).map(|s| s.info.clone())
        } else {
            None
        }
    }

    /// Set the active session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID to make active
    #[napi]
    pub async fn set_active_session(&self, session_id: String) -> napi::Result<()> {
        let sessions = self.sessions.read().await;
        if sessions.contains_key(&session_id) {
            let mut active = self.active_session.lock().await;
            *active = Some(session_id);
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Rename a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `new_name` - The new name
    #[napi]
    pub async fn rename_session(&self, session_id: String, new_name: String) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.info.name = new_name;
            Ok(())
        } else {
            Err(napi::Error::from_reason(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Rename a window.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    /// * `new_name` - The new name
    #[napi]
    pub async fn rename_window(
        &self,
        session_id: String,
        window_id: String,
        new_name: String,
    ) -> napi::Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;

        let window = session
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;

        window.info.name = Some(new_name);
        Ok(())
    }

    /// Get session count.
    ///
    /// # Returns
    /// * `u32` - Number of sessions
    #[napi]
    pub async fn session_count(&self) -> u32 {
        let sessions = self.sessions.read().await;
        sessions.len() as u32
    }

    /// Get window count for a session.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    ///
    /// # Returns
    /// * `u32` - Number of windows
    #[napi]
    pub async fn window_count(&self, session_id: String) -> napi::Result<u32> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;
        Ok(session.windows.len() as u32)
    }

    /// Get pane count for a window.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `window_id` - The window ID
    ///
    /// # Returns
    /// * `u32` - Number of panes
    #[napi]
    pub async fn pane_count(&self, session_id: String, window_id: String) -> napi::Result<u32> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(&session_id).ok_or_else(|| {
            napi::Error::from_reason(format!("Session not found: {}", session_id))
        })?;
        let window = session
            .windows
            .get(&window_id)
            .ok_or_else(|| napi::Error::from_reason(format!("Window not found: {}", window_id)))?;
        Ok(window.panes.len() as u32)
    }
}

impl Default for TerminalMultiplexer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a unique identifier with a prefix.
fn generate_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let num = COUNTER.fetch_add(1, Ordering::SeqCst);
    let random_part = rand_u64();
    format!("{}_{:x}_{:x}", prefix, num, random_part)
}

/// Simple random number generator for IDs.
fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0) as u64;
    // Simple xorshift
    let mut x = now;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// Create a new terminal multiplexer instance.
///
/// Convenience function for creating a TerminalMultiplexer.
#[napi]
pub fn create_multiplexer() -> TerminalMultiplexer {
    TerminalMultiplexer::new()
}

/// Check if terminal multiplexing is supported.
///
/// # Returns
/// * `bool` - True if multiplexing is supported
#[napi]
pub fn is_multiplexing_supported() -> bool {
    #[cfg(unix)]
    {
        true
    }
    #[cfg(windows)]
    {
        // Limited support on Windows
        true
    }
}
