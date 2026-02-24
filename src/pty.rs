//! Pseudo-terminal (PTY) support.
//!
//! This module provides PTY support on Unix systems.

use napi_derive::napi;
use std::collections::HashMap;

/// PTY configuration options.
#[napi(object)]
pub struct PtyConfig {
    /// Rows (height) of the PTY
    pub rows: Option<u16>,
    /// Columns (width) of the PTY
    pub columns: Option<u16>,
    /// Working directory for the process
    pub cwd: Option<String>,
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
    /// Shell to execute
    pub shell: Option<String>,
    /// Whether to use UTF-8 mode
    pub utf8: Option<bool>,
}

impl Default for PtyConfig {
    fn default() -> Self {
        PtyConfig {
            rows: Some(24),
            columns: Some(80),
            cwd: None,
            env: None,
            shell: None,
            utf8: Some(true),
        }
    }
}

/// PTY process information.
#[napi(object)]
pub struct PtyProcessInfo {
    /// Process ID
    pub pid: u32,
    /// PTY file descriptor
    pub fd: Option<u32>,
    /// PTY slave name
    pub pts_name: Option<String>,
}

/// PTY size information.
#[napi(object)]
pub struct PtySize {
    /// Number of rows (height)
    pub rows: u16,
    /// Number of columns (width)
    pub columns: u16,
    /// Pixel width (if available)
    pub pixel_width: Option<u16>,
    /// Pixel height (if available)
    pub pixel_height: Option<u16>,
}

/// Open a new PTY pair (Unix only).
#[napi]
pub fn open_pty(config: Option<PtyConfig>) -> napi::Result<PtyProcessInfo> {
    #[cfg(unix)]
    {
        let cfg = config.unwrap_or_default();
        let mut size: libc::winsize = unsafe { std::mem::zeroed() };
        size.ws_row = cfg.rows.unwrap_or(24);
        size.ws_col = cfg.columns.unwrap_or(80);

        let mut master_fd: libc::c_int = 0;
        let mut slave_fd: libc::c_int = 0;

        unsafe {
            let result = libc::openpty(
                &mut master_fd,
                &mut slave_fd,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &size,
            );

            if result != 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to open PTY: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        let pts_name = unsafe {
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            if libc::ptsname_r(slave_fd, buf.as_mut_ptr(), 1024) == 0 {
                Some(
                    std::ffi::CStr::from_ptr(buf.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                None
            }
        };

        unsafe { libc::close(slave_fd) };

        Ok(PtyProcessInfo {
            pid: 0,
            fd: Some(master_fd as u32),
            pts_name,
        })
    }

    #[cfg(not(unix))]
    {
        let _ = config;
        Err(napi::Error::from_reason(
            "PTY not supported on this platform".to_string(),
        ))
    }
}

/// Get the current PTY size (Unix only).
#[napi]
pub fn get_pty_size(fd: u32) -> napi::Result<PtySize> {
    #[cfg(unix)]
    {
        let mut size: libc::winsize = unsafe { std::mem::zeroed() };

        unsafe {
            if libc::ioctl(fd as libc::c_int, libc::TIOCGWINSZ, &mut size) != 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to get PTY size: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        Ok(PtySize {
            rows: size.ws_row,
            columns: size.ws_col,
            pixel_width: if size.ws_xpixel > 0 {
                Some(size.ws_xpixel)
            } else {
                None
            },
            pixel_height: if size.ws_ypixel > 0 {
                Some(size.ws_ypixel)
            } else {
                None
            },
        })
    }

    #[cfg(not(unix))]
    {
        let _ = fd;
        Err(napi::Error::from_reason(
            "get_pty_size not supported on this platform".to_string(),
        ))
    }
}

/// Set the PTY size (Unix only).
#[napi]
pub fn set_pty_size(fd: u32, rows: u16, columns: u16) -> napi::Result<()> {
    #[cfg(unix)]
    {
        let mut size: libc::winsize = unsafe { std::mem::zeroed() };
        size.ws_row = rows;
        size.ws_col = columns;

        unsafe {
            if libc::ioctl(fd as libc::c_int, libc::TIOCSWINSZ, &size) != 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to set PTY size: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        Ok(())
    }

    #[cfg(not(unix))]
    {
        let _ = fd;
        let _ = rows;
        let _ = columns;
        Err(napi::Error::from_reason(
            "set_pty_size not supported on this platform".to_string(),
        ))
    }
}

/// Get the current window size.
#[napi]
pub fn get_window_size() -> napi::Result<PtySize> {
    #[cfg(unix)]
    {
        let fd = unsafe { libc::dup(libc::STDOUT_FILENO) };
        if fd < 0 {
            return Ok(PtySize {
                rows: 24,
                columns: 80,
                pixel_width: None,
                pixel_height: None,
            });
        }

        let mut size: libc::winsize = unsafe { std::mem::zeroed() };
        let result = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut size) };
        unsafe { libc::close(fd) };

        if result != 0 || size.ws_col == 0 {
            return Ok(PtySize {
                rows: 24,
                columns: 80,
                pixel_width: None,
                pixel_height: None,
            });
        }

        Ok(PtySize {
            rows: size.ws_row,
            columns: size.ws_col,
            pixel_width: if size.ws_xpixel > 0 {
                Some(size.ws_xpixel)
            } else {
                None
            },
            pixel_height: if size.ws_ypixel > 0 {
                Some(size.ws_ypixel)
            } else {
                None
            },
        })
    }

    #[cfg(not(unix))]
    {
        Err(napi::Error::from_reason(
            "get_window_size not supported on this platform".to_string(),
        ))
    }
}

/// Get the slave PTY name for a given master fd.
#[napi]
pub fn get_pty_name(fd: u32) -> napi::Result<String> {
    #[cfg(unix)]
    {
        unsafe {
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            if libc::ptsname_r(fd as libc::c_int, buf.as_mut_ptr(), 1024) == 0 {
                Ok(std::ffi::CStr::from_ptr(buf.as_ptr())
                    .to_string_lossy()
                    .into_owned())
            } else {
                Err(napi::Error::from_reason(format!("Failed to get PTY name",)))
            }
        }
    }

    #[cfg(not(unix))]
    {
        let _ = fd;
        Err(napi::Error::from_reason(
            "get_pty_name not supported on this platform".to_string(),
        ))
    }
}

/// Check if PTY is supported on this platform.
#[napi]
pub fn is_pty_supported() -> bool {
    #[cfg(unix)]
    {
        true
    }
    #[cfg(not(unix))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_config_default() {
        let config = PtyConfig::default();
        assert_eq!(config.rows, Some(24));
        assert_eq!(config.columns, Some(80));
    }

    #[test]
    fn test_pty_size_struct() {
        let size = PtySize {
            rows: 24,
            columns: 80,
            pixel_width: Some(1920),
            pixel_height: Some(1080),
        };
        assert_eq!(size.rows, 24);
    }

    #[test]
    fn test_is_pty_supported() {
        let supported = is_pty_supported();
        #[cfg(unix)]
        assert!(supported);
    }
}
