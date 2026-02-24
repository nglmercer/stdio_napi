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

#### Color Support (`src/stdio.rs`)

- [x] **Success Messages**: `print_success(text: String)` - Green bold output
- [x] **Error Messages**: `print_error(text: String)` - Red bold output to stderr
- [x] **Warning Messages**: `print_warning(text: String)` - Yellow output
- [x] **Info Messages**: `print_info(text: String)` - Blue output

#### Terminal Utilities (`src/terminal.rs`)

- [x] **Terminal Size**: `get_terminal_size()` - Get terminal columns and rows
- [x] **Clear Screen**: `clear_screen()` - Clear terminal and reset cursor
- [x] **Cursor Movement**: `move_cursor(column: u16, row: u16)` - Move cursor to position

#### Utility Functions (`src/lib.rs`)

- [x] **Version Info**: `get_version()` - Get package version

---

## Features to Implement

### Child Process Spawning (`src/process.rs`)

High priority for v2.0.0:

- [ ] **Basic Spawn**: `spawn(command: String, args: Vec<String>)` - Spawn a child process
- [ ] **Spawn with Options**: `spawn_with_options(options: SpawnOptions)` - Spawn with configuration
  ```rust
  struct SpawnOptions {
      command: String,
      args: Vec<String>,
      cwd: Option<String>,
      env: Option<HashMap<String, String>>,
      timeout: Option<u32>,
  }
  ```
- [ ] **Stdout Pipe**: Read stdout from child process asynchronously
- [ ] **Stderr Pipe**: Read stderr from child process asynchronously
- [ ] **Stdin Pipe**: Write to stdin of child process
- [ ] **Process Status**: `ProcessStatus` struct with PID, exit code, signal
- [ ] **Process Kill**: `kill(pid: u32, signal: Option<String>)` - Terminate process
- [ ] **Process Wait**: `wait(pid: u32)` - Wait for process completion
- [ ] **Exec Sync**: `exec_sync(command: String)` - Synchronous command execution
- [ ] **Shell Escape**: Utility for safe shell command escaping

### Enhanced Terminal Features

- [ ] **Terminal Type Detection**: Detect terminal emulator type
- [ ] **Color Support Detection**: Check truecolor/256 color support
- [ ] **Raw Mode**: Enter/exit raw terminal mode
- [ ] **Alternate Screen**: Switch to alternate screen buffer
- [ ] **Cursor Visibility**: Show/hide cursor
- [ ] **Cursor Shape**: Change cursor shape (block, underline, bar)
- [ ] **Scroll Region**: Set scroll region
- [ ] **Title Support**: Get/set terminal title

### Advanced Stdio Features

- [ ] **Buffered Reader**: `BufferedReader` class for efficient reading
- [ ] **Line Iterator**: Async iterator for reading lines
- [ ] **Progress Bar**: Built-in progress bar utilities
- [ ] **Spinner**: Loading spinner animation
- [ ] **Multi-line Input**: Read multiple lines until delimiter
- [ ] **Password Input**: Masked password input
- [ ] **Select Menu**: Interactive selection menu
- [ ] **Confirm Dialog**: Yes/No confirmation with default

### Stream Handling

- [ ] **Stream Duplex**: Full-duplex stream for process communication
- [ ] **Stream Events**: EventEmitter-like interface for stdout/stderr
- [ ] **Buffer Management**: Configurable buffer sizes
- [ ] **Backpressure**: Handle backpressure in streams

---

## Bug Fixes & Improvements

### Critical

- [ ] **Fix Test Suite**: Tests in `tests/index.test.ts` reference non-existent functions
  - `Message`, `MessageType`, `createGreeting`, `add`, `processNumbers`
  - `divideNumbers`, `generateSequence`, `delayedMessage`
- [ ] **Fix Demo Imports**: `examples/demo.ts` imports `printStdout` but function is `print_stdout`

### Improvements

- [ ] **Error Handling**: Add custom error types for better error messages
- [ ] **TypeScript Types**: Generate comprehensive `.d.ts` type definitions
- [ ] **Documentation Comments**: Add rustdoc comments to all public functions
- [ ] **Input Validation**: Validate parameters before processing
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

- [ ] Fix existing test suite
- [ ] Add tests for all stdio functions
- [ ] Add tests for terminal functions
- [ ] Add tests for process spawning
- [ ] Add performance benchmarks

---

## Dependencies to Consider

| Crate         | Purpose          | Status                |
| ------------- | ---------------- | --------------------- |
| `tokio`       | Async runtime    | ✅ Added              |
| `crossterm`   | Terminal control | ✅ Added              |
| `colored`     | Color output     | ✅ Added              |
| `napi`        | Node bindings    | ✅ Added              |
| `libc`        | Unix syscalls    | 🔲 Needed for signals |
| `windows-sys` | Windows APIs     | 🔲 Needed for Windows |
| `which`       | Find executable  | 🔲 Optional           |

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

- Progress bars
- Spinners
- Interactive menus
- Form inputs
