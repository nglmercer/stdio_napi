# TODO: stdio-napi

A NAPI-RS native addon for Node.js/Bun providing stdio operations, terminal utilities, and child process management.

## Project Status

### Initial Structure

- [x] Initialize project structure (NAPI-RS)
- [x] Rename project to `stdio-napi`
- [x] Define core modules for stdio handling
- [x] Set up build configuration for multiple platforms

### Implemented Features

#### Stdio Operations (`src/stdio.rs`)

- [x] **Stdout Wrapper**: `print_stdout(text: String)` - Write to stdout from Rust
- [x] **Stderr Wrapper**: `print_stderr(text: String)` - Write to stderr from Rust
- [x] **Stdin Reader**: `read_line()` - Asynchronous reader for stdin
- [x] **Prompt Interface**: `prompt(message: String)` - Interactive input prompts
- [x] **Confirm Dialog**: `confirm(message: String, default: Option<bool>)` - Yes/No confirmation
- [x] **Read Password**: `read_password(mask: Option<String>)` - Masked password input
- [x] **Select Menu**: `select_menu(message: String, options: Vec<String>)` - Interactive selection
- [x] **Read Multi-line**: `read_multiline(delimiter: Option<String>)` - Multi-line input
- [x] **Progress Bar**: `print_progress(current: u32, total: u32, width: Option<u32>)` - Progress display
- [x] **Spinner Frame**: `get_spinner_frame(frame: u32)` - Get spinner animation frame

#### Color Support (`src/stdio.rs`)

- [x] **Success Messages**: `print_success(text: String)` - Green bold output
- [x] **Error Messages**: `print_error(text: String)` - Red bold output to stderr
- [x] **Warning Messages**: `print_warning(text: String)` - Yellow output
- [x] **Info Messages**: `print_info(text: String)` - Blue output

#### Terminal Utilities (`src/terminal.rs`)

- [x] **Terminal Size**: `get_terminal_size()` - Get terminal columns and rows
- [x] **Terminal Info**: `get_terminal_info()` - Get terminal type and color support
- [x] **Clear Screen**: `clear_screen()` - Clear terminal and reset cursor
- [x] **Cursor Movement**: `move_cursor(column: u16, row: u16)` - Move cursor to position
- [x] **Show Cursor**: `show_cursor()` - Show cursor
- [x] **Hide Cursor**: `hide_cursor()` - Hide cursor
- [x] **Set Cursor Shape**: `set_cursor_shape(shape: CursorShape)` - Change cursor shape
- [x] **Set Terminal Title**: `set_terminal_title(title: String)` - Set terminal title
- [x] **Enter Alternate Screen**: `enter_alternate_screen()` - Enter alternate screen buffer
- [x] **Leave Alternate Screen**: `leave_alternate_screen()` - Leave alternate screen buffer
- [x] **Enable Raw Mode**: `enable_raw_mode()` - Enable raw mode
- [x] **Disable Raw Mode**: `disable_raw_mode()` - Disable raw mode
- [x] **Set Scroll Region**: `set_scroll_region(top: u16, bottom: u16)` - Set scroll region
- [x] **Reset Scroll Region**: `reset_scroll_region()` - Reset scroll region to full terminal

#### Utility Functions (`src/lib.rs`)

- [x] **Version Info**: `get_version()` - Get package version

#### Child Process Spawning (`src/process.rs`)

- [x] **Basic Spawn**: `exec_command(command: String, args: Vec<String>)` - Spawn and wait for a child process
- [x] **Spawn with Options**: `spawn_with_options(options: SpawnOptions)` - Spawn with configuration
- [x] **Process Status**: `ProcessStatus` struct with PID, exit code, success flag
- [x] **Process Output**: `ProcessOutput` struct with stdout, stderr, code, success
- [x] **Spawn with Pipes**: `spawn_with_pipes(options: SpawnOptions)` - Spawn with piped stdio
- [x] **Exec Sync**: `exec_sync(command: String)` - Synchronous command execution
- [x] **Exec Sync with Args**: `exec_sync_with_args(command: String, args: Vec<String>)` - Sync with arguments
- [x] **Shell Escape**: `shell_escape(input: String)` - Escape shell arguments
- [x] **Shell Escape Args**: `shell_escape_args(args: Vec<String>)` - Escape multiple arguments
- [x] **Process Kill**: `kill_process(pid: u32, signal: Option<String>)` - Kill process by PID

---

## Features to Implement

### Advanced Stdio Features (>>> Complete)

- [x] **Buffered Reader**: `BufferedReader` class for efficient reading
- [x] **Line Iterator**: Async iterator for reading lines

### Stream Handling (>>> Complete)

- [x] **Managed Process**: `ManagedProcess` class with streaming capabilities
- [x] **Buffer Management**: Configurable buffer sizes (`BufferConfig`)
- [x] **Process Events**: Stream events for stdout/stderr/exit

### Error Handling & Type Definitions (>>> Complete)

- [x] **Custom Error Types**: Add custom error types for better error messages (`StdioError` enum)
- [x] **Input Validation**: Validate parameters before processing (validation module)
- [x] **TypeScript Types**: Generate comprehensive `.d.ts` type definitions

### Documentation & Testing (>>> Complete)

- [x] **Documentation Comments**: Add rustdoc comments to all public functions
- [x] **Integration Tests**: Add tests for all stdio functions
- [x] **Performance Benchmarks**: Add performance benchmarks

---

## Bug Fixes & Improvements

### Critical

- [x] **Fix Test Suite**: Tests in `tests/index.test.ts` reference non-existent functions
- [x] **Fix Demo Imports**: `examples/demo.ts` imports corrected

### Improvements

- [x] **TypeScript Types**: Generate comprehensive `.d.ts` type definitions
- [x] **Documentation Comments**: Add rustdoc comments to all public functions
- [x] **Graceful Degradation**: Handle missing TTY gracefully

---

## Dependencies

| Crate              | Purpose               | Status      |
| ------------------ | --------------------- | ----------- |
| `tokio`            | Async runtime         | ✅ Added    |
| `crossterm`        | Terminal control      | ✅ Added    |
| `colored`          | Color output          | ✅ Added    |
| `napi`             | Node bindings         | ✅ Added    |
| `libc`             | Unix syscalls         | ✅ Added    |
| `errno`            | Error numbers         | ✅ Added    |
| `regex`            | Pattern matching      | ✅ Added    |
| `signal-hook`      | Unix signal handling  | ✅ Added    |
| `signal-hook-tokio | Async signal handling | ✅ Added    |
| `nix`              | Unix APIs             | ✅ Added    |
| `winapi`           | Windows APIs          | ✅ Added    |
| `windows-sys`      | Windows APIs          | 🔲 Future   |
| `which`            | Find executable       | 🔲 Optional |

---

## Release Milestones

### v1.1.0 - Bug Fixes (✅ Complete)

- [x] Fix test suite
- [x] Fix demo imports
- [x] Add comprehensive tests

### v2.0.0 - Child Process Support (✅ Complete)

- [x] Implement `spawn` functionality
- [x] Add process management
- [x] Stream handling for stdio

### v2.1.0 - Enhanced Terminal (✅ Complete)

- [x] Raw mode support
- [x] Alternate screen
- [x] Advanced cursor control

### v3.0.0 - Interactive UI (✅ Complete)

- [x] Progress bars
- [x] Spinners
- [x] Interactive menus
- [x] Form inputs (✅ Complete)

### v4.0.0 - Advanced Features (✅ Complete)

- [x] Form inputs for interactive data collection
- [x] Table rendering for terminal output
- [x] Key event handling
- [x] Mouse event support
- [x] Performance benchmarks

### v5.0.0 - Terminal Multiplexing & Signal Handling (✅ Complete)

- [x] Terminal multiplexing support
  - [x] TerminalMultiplexer class
  - [x] Session, Window, Pane management
  - [x] Full CRUD operations for sessions/windows/panes
- [x] Cross-platform signal handling
  - [x] SignalHandler class
  - [x] send_signal, get_signal_info functions
  - [x] Process group management
- [x] API improvements
  - [x] AsyncLineIterator for streaming
  - [x] ProcessEventEmitter for event-driven process management
  - [x] ProcessBuilder for fluent configuration

---

## Next Phase: v6.0.0 - Platform Enhancements

### Windows-Specific APIs

- [ ] Windows console API integration
- [ ] Windows job objects for process management
- [ ] Windows named pipes support

### macOS-Specific APIs

- [ ] macOS terminal integration
- [ ] macOS keychain integration for secure storage

### Advanced Features

- [ ] Pseudo-terminal (PTY) support

---

## Completed: v4.0.0 - Advanced Features (✅ Complete)

### Advanced Features

- [x] Form inputs for interactive data collection (✅ Complete)
  - [x] Text input with validation
  - [x] Number input with min/max validation
  - [x] Password input with raw mode
  - [x] Checkbox (yes/no)
  - [x] Select with arrow key navigation
  - [x] Multi-select with checkbox selection
  - [x] Batch form (collect_form, display_form)
- [x] Table rendering for terminal output (✅ Complete)
  - [x] render_table - Render table to string
  - [x] print_table - Print table directly to stdout
  - [x] create_table - Simple table from headers and rows
  - [x] render_key_value_table - Key-value pairs table
  - [x] render_csv_table - CSV data to table
  - [x] Multiple border styles (single, double, rounded, markdown, ascii)
  - [x] Column alignment (left, center, right)
  - [x] Color support for headers and cells
- [x] Key event handling (keyboard shortcuts) (✅ Complete)
  - [x] read_key - Read single key press
  - [x] read_key_with_timeout - Read key with timeout
  - [x] read_line_with_events - Read line with key events
  - [x] wait_for_shortcut - Wait for keyboard shortcut
  - [x] is_keyboard_available - Check if terminal
  - [x] KeyboardEvent struct with modifiers
- [x] Mouse event support (✅ Complete)
  - [x] enable_mouse - Enable mouse reporting
  - [x] disable_mouse - Disable mouse reporting
  - [x] read_mouse_event - Read mouse event
  - [x] read_mouse_event_timeout - Read with timeout
  - [x] wait_for_click_at - Wait for click at position
  - [x] wait_for_click_in_region - Wait for click in region
  - [x] MouseEventListener - Continuous mouse monitoring
  - [x] is_mouse_supported - Check mouse support
- [x] Terminal multiplexing support (✅ Complete)
  - [x] TerminalMultiplexer class for session management
  - [x] Session, Window, Pane management
  - [x] create_session, list_sessions, attach_session, detach_session, kill_session
  - [x] create_window, list_windows, kill_window, select_window, rename_window
  - [x] split_pane, list_panes, kill_pane, select_pane, resize_pane
  - [x] send_keys for pane input
  - [x] is_multiplexing_supported check

### Performance (✅ Complete)

- [x] Add benchmarks for stdio operations
  - [x] benchmark_print - Print operations
  - [x] benchmark_string_format - String formatting
  - [x] benchmark_progress_bar - Progress bar rendering
  - [x] benchmark_spinner - Spinner frame generation
- [x] Add benchmarks for terminal operations
  - [x] benchmark_terminal_size - Terminal size queries
  - [x] benchmark_table_render - Table rendering
- [x] Add benchmarks for process spawning
  - [x] benchmark_shell_escape - Shell escape operations
  - [x] benchmark_key_parsing - Key combination parsing
- [x] Add general benchmarks
  - [x] benchmark_string_allocation - String allocation
  - [x] benchmark_vector_operations - Vector operations
  - [x] benchmark_hashmap_operations - HashMap operations
  - [x] run_benchmark_suite - Run all benchmarks

### Platform Enhancements (✅ Complete)

- [ ] Add Windows-specific APIs
- [ ] Add macOS-specific APIs
- [x] Cross-platform signal handling (✅ Complete)
  - [x] SignalHandler class for receiving signals
  - [x] send_signal - Send signals to processes
  - [x] get_signal_info - Get signal information
  - [x] on_interrupt - Register Ctrl+C handler
  - [x] is_tty - Check if running in TTY
  - [x] is_background - Check if process is in background
  - [x] get_process_group - Get process group ID
  - [x] set_process_group - Set process group ID
  - [x] get_supported_signals - List supported signals

### API Improvements (✅ Complete)

- [x] Async iterator support for ManagedProcess (✅ Complete)
  - [x] AsyncLineIterator class
  - [x] next, is_exhausted, collect, take, skip, filter methods
- [x] Event emitter pattern for process events (✅ Complete)
  - [x] ProcessEventEmitter class
  - [x] on, once, off, remove_all_listeners, listener_count methods
  - [x] ProcessEvent enum (Start, Stdout, Stderr, Exit, Error)
- [x] Configuration builder pattern (✅ Complete)
  - [x] ProcessBuilder class
  - [x] arg, args, cwd, env, envs methods
  - [x] capture_stdout, capture_stderr, capture_stdin methods
  - [x] detached, shell, timeout methods
  - [x] spawn, exec, to_string methods

### Documentation

- [x] Create detailed README for stdio operations
- [x] Document all NAPI exports with examples
- [x] Add API reference documentation
- [x] Create migration guide from Node.js `child_process`
- [ ] Add architecture diagram
- [x] Document platform-specific behavior

### Unit Tests (Rust)

- [x] Add unit tests for `stdio.rs` functions
- [x] Add unit tests for `terminal.rs` functions
- [x] Add unit tests for process spawning
- [x] Add edge case tests (empty input, large input, unicode)

### Integration Tests (TypeScript)

- [x] Add tests for all stdio functions
- [x] Add tests for terminal functions
- [x] Add tests for process spawning
- [x] Add performance benchmarks (via edge case tests)
