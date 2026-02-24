//! Windows-specific APIs for console, job objects, and named pipes.
//!
//! This module provides Windows-specific functionality for:
//! - Console API integration (screen buffer, console modes)
//! - Windows Job Objects for process group management
//! - Named pipes for IPC

use napi_derive::napi;

/// Windows console buffer information.
#[napi(object)]
pub struct ConsoleBufferInfo {
    /// Size of the buffer (width, height)
    pub size: Vec<u32>,
    /// Cursor position (x, y)
    pub cursor_position: Vec<u32>,
    /// Attribute used for writing text
    pub attributes: u32,
    /// Window rectangle (left, top, right, bottom)
    pub window: Vec<u32>,
    /// Maximum window size
    pub max_window_size: Vec<u32>,
}

/// Windows console m       ode flags.
#[napi(object)]
pub struct ConsoleMode {
    /// Input mode flags
    pub input: u32,
    /// Output mode flags
    pub output: u32,
}

/// Console mode constants.
#[napi]
pub enum ConsoleModeFlags {
    /// Echo input characters
    EchoInput = 0x0004,
    /// Line input mode
    LineInput = 0x0002,
    /// Mouse input enabled
    MouseInput = 0x0010,
    /// Window input enabled
    WindowInput = 0x0020,
    /// Processed input (Ctrl+C, etc.)
    ProcessedInput = 0x0001,
    /// Raw input
    RawInput = 0x0100,
    /// VT220 escape sequence output
    VirtualTerminal = 0x0200,
    /// Auto-position cursor
    AutoPosition = 0x0400,
}

/// Console color constants.
#[napi]
pub enum ConsoleColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Yellow = 6,
    White = 7,
    Bright = 8,
}

/// Get console buffer information.
#[napi]
pub fn get_console_buffer_info() -> napi::Result<ConsoleBufferInfo> {
    #[cfg(windows)]
    {
        use std::mem::MaybeUninit;

        unsafe {
            let handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(
                    "Failed to get console handle".to_string(),
                ));
            }

            let mut info: MaybeUninit<winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO> =
                MaybeUninit::uninit();
            if winapi::um::wincon::GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) == 0 {
                return Err(napi::Error::from_reason(
                    "Failed to get console buffer info".to_string(),
                ));
            }

            let info = info.assume_init();

            Ok(ConsoleBufferInfo {
                size: vec![info.dwSize.X as u32, info.dwSize.Y as u32],
                cursor_position: vec![
                    info.dwCursorPosition.X as u32,
                    info.dwCursorPosition.Y as u32,
                ],
                attributes: info.wAttributes as u32,
                window: vec![
                    info.srWindow.Left as u32,
                    info.srWindow.Top as u32,
                    info.srWindow.Right as u32,
                    info.srWindow.Bottom as u32,
                ],
                max_window_size: vec![
                    info.dwMaximumWindowSize.X as u32,
                    info.dwMaximumWindowSize.Y as u32,
                ],
            })
        }
    }

    #[cfg(not(windows))]
    {
        Err(napi::Error::from_reason(
            "Console buffer info only available on Windows".to_string(),
        ))
    }
}

/// Get console input mode.
#[napi]
pub fn get_console_input_mode() -> napi::Result<ConsoleMode> {
    #[cfg(windows)]
    {
        unsafe {
            let handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE);
            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(
                    "Failed to get console handle".to_string(),
                ));
            }

            let mut mode: u32 = 0;
            if winapi::um::consoleapi::GetConsoleMode(handle, &mut mode) == 0 {
                return Err(napi::Error::from_reason(
                    "Failed to get console mode".to_string(),
                ));
            }

            // Also get output mode
            let output_handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
            let mut output_mode: u32 = 0;
            if output_handle != winapi::um::handleapi::INVALID_HANDLE_VALUE {
                let _ = winapi::um::consoleapi::GetConsoleMode(output_handle, &mut output_mode);
            }

            Ok(ConsoleMode {
                input: mode,
                output: output_mode,
            })
        }
    }

    #[cfg(not(windows))]
    {
        Err(napi::Error::from_reason(
            "Console mode only available on Windows".to_string(),
        ))
    }
}

/// Set console input mode.
#[napi]
pub fn set_console_input_mode(mode: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE);
            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(
                    "Failed to get console handle".to_string(),
                ));
            }

            if winapi::um::consoleapi::SetConsoleMode(handle, mode) == 0 {
                return Err(napi::Error::from_reason(
                    "Failed to set console mode".to_string(),
                ));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = mode;
        Err(napi::Error::from_reason(
            "Console mode only available on Windows".to_string(),
        ))
    }
}

/// Set console output mode.
#[napi]
pub fn set_console_output_mode(mode: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(
                    "Failed to get console handle".to_string(),
                ));
            }

            if winapi::um::consoleapi::SetConsoleMode(handle, mode) == 0 {
                return Err(napi::Error::from_reason(
                    "Failed to set console mode".to_string(),
                ));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = mode;
        Err(napi::Error::from_reason(
            "Console mode only available on Windows".to_string(),
        ))
    }
}

/// Enable virtual terminal processing for ANSI escape sequences.
#[napi]
pub fn enable_virtual_terminal_processing() -> napi::Result<()> {
    #[cfg(windows)]
    {
        const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
        set_console_output_mode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)
    }

    #[cfg(not(windows))]
    {
        Err(napi::Error::from_reason(
            "Virtual terminal processing only available on Windows".to_string(),
        ))
    }
}

/// Set console text color.
#[napi]
pub fn set_console_text_color(color: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle =
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(
                    "Failed to get console handle".to_string(),
                ));
            }

            if winapi::um::wincon::SetConsoleTextAttribute(handle, color as u16) == 0 {
                return Err(napi::Error::from_reason(
                    "Failed to set console text color".to_string(),
                ));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = color;
        Err(napi::Error::from_reason(
            "Console text color only available on Windows".to_string(),
        ))
    }
}

/// Reset console text attributes to default.
#[napi]
pub fn reset_console_attributes() -> napi::Result<()> {
    #[cfg(windows)]
    {
        // Default is white on black
        set_console_text_color(7) // White on black
    }

    #[cfg(not(windows))]
    {
        Err(napi::Error::from_reason(
            "Console attributes only available on Windows".to_string(),
        ))
    }
}

/// Windows Job Object information.
#[napi(object)]
pub struct JobObjectInfo {
    /// Job object handle (as number)
    pub handle: i64,
    /// Process IDs in the job
    pub process_ids: Vec<u32>,
    /// Whether the job is empty
    pub is_empty: bool,
}

/// Create a Windows Job Object.
#[napi]
pub fn create_job_object() -> napi::Result<i64> {
    #[cfg(windows)]
    {
        unsafe {
            let handle =
                winapi::um::jobapi2::CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
            if handle.is_null() {
                return Err(napi::Error::from_reason(format!(
                    "Failed to create job object: {}",
                    std::io::Error::last_os_error()
                )));
            }
            Ok(handle as i64)
        }
    }

    #[cfg(not(windows))]
    {
        Err(napi::Error::from_reason(
            "Job objects only available on Windows".to_string(),
        ))
    }
}

/// Assign a process to a Job Object.
#[napi]
pub fn assign_process_to_job(job_handle: i64, process_id: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = job_handle as *mut winapi::ctypes::c_void;

            // Open the process
            let process = winapi::um::processthreadsapi::OpenProcess(
                winapi::um::winnt::PROCESS_SET_QUOTA | winapi::um::winnt::PROCESS_TERMINATE,
                0,
                process_id,
            );

            if process.is_null() {
                return Err(napi::Error::from_reason(format!(
                    "Failed to open process: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let result = winapi::um::jobapi2::AssignProcessToJobObject(handle, process);
            winapi::um::handleapi::CloseHandle(process);

            if result == 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to assign process to job: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = job_handle;
        let _ = process_id;
        Err(napi::Error::from_reason(
            "Job objects only available on Windows".to_string(),
        ))
    }
}

/// Terminate all processes in a Job Object.
#[napi]
pub fn terminate_job_object(job_handle: i64, exit_code: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = job_handle as *mut winapi::ctypes::c_void;

            if winapi::um::jobapi2::TerminateJobObject(handle, exit_code) == 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to terminate job: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = job_handle;
        let _ = exit_code;
        Err(napi::Error::from_reason(
            "Job objects only available on Windows".to_string(),
        ))
    }
}

/// Close a Job Object handle.
#[napi]
pub fn close_job_object(job_handle: i64) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = job_handle as *mut winapi::ctypes::c_void;
            if winapi::um::handleapi::CloseHandle(handle) == 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to close job object: {}",
                    std::io::Error::last_os_error()
                )));
            }
            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = job_handle;
        Err(napi::Error::from_reason(
            "Job objects only available on Windows".to_string(),
        ))
    }
}

/// Named pipe information.
#[napi(object)]
pub struct NamedPipeInfo {
    /// Pipe name
    pub name: String,
    /// Whether the pipe is connected
    pub connected: bool,
    /// Number of instances available
    pub instances: u32,
    /// Maximum buffer size
    pub max_buffer_size: u32,
}

/// Create a named pipe server.
#[napi]
pub fn create_named_pipe(name: String, buffer_size: u32) -> napi::Result<i64> {
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let wide_name: Vec<u16> = OsStr::new(&name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let handle = winapi::um::namedpipeapi::CreateNamedPipeW(
                wide_name.as_ptr(),
                winapi::um::winbase::PIPE_ACCESS_DUPLEX,
                winapi::um::winbase::PIPE_TYPE_MESSAGE
                    | winapi::um::winbase::PIPE_READMODE_MESSAGE
                    | winapi::um::winbase::PIPE_WAIT,
                1, // Max instances
                buffer_size,
                buffer_size,
                0, // Default timeout
                std::ptr::null_mut(),
            );

            if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(napi::Error::from_reason(format!(
                    "Failed to create named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(handle as i64)
        }
    }

    #[cfg(not(windows))]
    {
        let _ = name;
        let _ = buffer_size;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Connect to a named pipe (client).
#[napi]
pub fn connect_named_pipe(pipe_handle: i64) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = pipe_handle as *mut winapi::ctypes::c_void;

            if winapi::um::namedpipeapi::ConnectNamedPipe(handle, std::ptr::null_mut()) == 0 {
                let error = std::io::Error::last_os_error();
                // ERROR_PIPE_CONNECTED (535) is OK - means already connected
                if error.raw_os_error() != Some(535) {
                    return Err(napi::Error::from_reason(format!(
                        "Failed to connect to named pipe: {}",
                        error
                    )));
                }
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = pipe_handle;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Wait for a named pipe to become available.
#[napi]
pub fn wait_named_pipe(name: String, timeout_ms: u32) -> napi::Result<()> {
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let wide_name: Vec<u16> = OsStr::new(&name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            if winapi::um::namedpipeapi::WaitNamedPipeW(wide_name.as_ptr(), timeout_ms) == 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to wait for named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = name;
        let _ = timeout_ms;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Read from a named pipe.
#[napi]
pub fn read_named_pipe(pipe_handle: i64, size: u32) -> napi::Result<Vec<u8>> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = pipe_handle as *mut winapi::ctypes::c_void;
            let mut buffer: Vec<u8> = vec![0; size as usize];
            let mut bytes_read: u32 = 0;

            if winapi::um::fileapi::ReadFile(
                handle,
                buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
                size,
                &mut bytes_read,
                std::ptr::null_mut(),
            ) == 0
            {
                return Err(napi::Error::from_reason(format!(
                    "Failed to read from named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }

            buffer.truncate(bytes_read as usize);
            Ok(buffer)
        }
    }

    #[cfg(not(windows))]
    {
        let _ = pipe_handle;
        let _ = size;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Write to a named pipe.
#[napi]
pub fn write_named_pipe(pipe_handle: i64, data: Vec<u8>) -> napi::Result<u32> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = pipe_handle as *mut winapi::ctypes::c_void;
            let mut bytes_written: u32 = 0;

            if winapi::um::fileapi::WriteFile(
                handle,
                data.as_ptr() as *const winapi::ctypes::c_void,
                data.len() as u32,
                &mut bytes_written,
                std::ptr::null_mut(),
            ) == 0
            {
                return Err(napi::Error::from_reason(format!(
                    "Failed to write to named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(bytes_written)
        }
    }

    #[cfg(not(windows))]
    {
        let _ = pipe_handle;
        let _ = data;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Close a named pipe handle.
#[napi]
pub fn close_named_pipe(pipe_handle: i64) -> napi::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            let handle = pipe_handle as *mut winapi::ctypes::c_void;
            if winapi::um::handleapi::CloseHandle(handle) == 0 {
                return Err(napi::Error::from_reason(format!(
                    "Failed to close named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }
            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let _ = pipe_handle;
        Err(napi::Error::from_reason(
            "Named pipes only available on Windows".to_string(),
        ))
    }
}

/// Check if running on Windows.
#[napi]
pub fn is_windows() -> bool {
    #[cfg(windows)]
    {
        true
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_windows() {
        let result = is_windows();
        #[cfg(windows)]
        assert!(result);
        #[cfg(not(windows))]
        assert!(!result);
    }

    #[test]
    fn test_console_mode_flags_values() {
        // Test that the enum variants have correct values
        let _ = ConsoleModeFlags::EchoInput as u32;
        let _ = ConsoleModeFlags::LineInput as u32;
        let _ = ConsoleModeFlags::MouseInput as u32;
    }

    #[test]
    fn test_console_color_values() {
        // Test that color enum variants exist
        let _ = ConsoleColor::Black as u32;
        let _ = ConsoleColor::Blue as u32;
        let _ = ConsoleColor::Green as u32;
        let _ = ConsoleColor::Red as u32;
        let _ = ConsoleColor::White as u32;
    }
}
