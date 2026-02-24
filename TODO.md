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

- [x] **Stream Duplex**: Full-duplex stream for process communication (`StreamDuplex`)
- [x] **Stream Events**: EventEmitter-like interface for stdout/stderr (broadcast channels)
- [x] **Buffer Management**: Configurable buffer sizes (`BufferConfig`)
- [x] **Backpressure**: Handle backpressure in streams (pause/resume write/read)

### Error Handling & Type Definitions (>>> Complete)

- [x] **Custom Error Types**: Add custom error types for better error messages (`StdioError` enum)
- [x] **Input Validation**: Validate parameters before processing (validation module)
- [ ] **TypeScript Types**: Generate comprehensive `.d.ts` type definitions

---

## Bug Fixes & Improvements

### Critical

- [x] **Fix Test Suite**: Tests in `tests/index.test.ts` reference non-existent functions
- [x] **Fix Demo Imports**: `examples/demo.ts` imports corrected

### Improvements

- [ ] **TypeScript Types**: Generate comprehensive `.d.ts` type definitions
- [ ] **Documentation Comments**: Add rustdoc comments to all public functions
- [ ] **Graceful Degradation**: Handle missing TTY gracefully

---

## Documentation

- [ ] Create detailed README for stdio operations
- [ ] Document all NAPI exports with examples
- [ ] Add API reference documentation
- [ ] Create migration guide from Node.js `child_process`
- [ ] Add architecture diagram
- [ ] Document platform-specific behavior

---

## Testing

### Unit Tests (Rust)

- [ ] Add unit tests for `stdio.rs` functions
- [ ] Add unit tests for `terminal.rs` functions
- [ ] Add unit tests for process spawning
- [ ] Add edge case tests (empty input, large input, unicode)

### Integration Tests (TypeScript)

- [x] Fix existing test suite
- [ ] Add tests for all stdio functions
- [ ] Add tests for terminal functions
- [ ] Add tests for process spawning
- [ ] Add performance benchmarks

---

## Dependencies

| Crate         | Purpose          | Status      |
| ------------- | ---------------- | ----------- |
| `tokio`       | Async runtime    | ✅ Added    |
| `crossterm`   | Terminal control | ✅ Added    |
| `colored`     | Color output     | ✅ Added    |
| `napi`        | Node bindings    | ✅ Added    |
| `libc`        | Unix syscalls    | ✅ Added    |
| `errno`       | Error numbers    | ✅ Added    |
| `windows-sys` | Windows APIs     | 🔲 Future   |
| `which`       | Find executable  | 🔲 Optional |

---

## Release Milestones

### v1.1.0 - Bug Fixes

- Fix test suite
- Fix demo imports
- Add comprehensive tests

### v2.0.0 - Child Process Support

- Implement `spawn` functionality
- Add process management
- Stream handling for stdio

### v2.1.0 - Enhanced Terminal

- Raw mode support
- Alternate screen
- Advanced cursor control

### v3.0.0 - Interactive UI

- [x] Progress bars
- [x] Spinners
- [x] Interactive menus
- [ ] Form inputs
