# stdio-napi

A powerful NAPI (Node-API) native addon built with Rust for advanced standard input/output and terminal operations in Node.js/Bun.

## Features

- **Standard I/O Wrappers**: Easy-to-use functions for writing to stdout/stderr.
- **Async Stdin**: Non-blocking input reading from Rust.
- **Terminal Operations**: Get terminal size, clear screen, cursor movement, alternate screen buffer.
- **Colored Output**: Built-in support for colored terminal messages (Success, Info, Warning, Error).
- **Interactive Prompts**: Built-in prompts for user input, confirmations, passwords, and selections.
- **Progress Indicators**: Progress bars and spinner animations.
- **Child Process Management**: Spawn processes, manage them with streaming I/O.
- **TypeScript Support**: Automatic type definitions generation.
- **Bun Optimized**: Seamless integration with Bun runtime.

## Installation

```bash
# Using npm
npm install

# Using bun
bun install

# Using yarn
yarn
```

## Building

### Release build

```bash
# Using npm
npm run build

# Using bun
bun run build
```

### Debug build

```bash
npm run build:debug
```

## Quick Start

```typescript
import { printSuccess, prompt, getTerminalSize } from "stdio-napi";

async function main() {
  const size = getTerminalSize();
  console.log(`Working in ${size.columns}x${size.rows} terminal`);

  printSuccess("stdio-napi is ready!");

  const name = await prompt("Enter your name");
  console.log(`Hello, ${name}!`);
}

main();
```

## API Reference

### Stdio Operations

#### printStdout

Write text to stdout without a newline.

```typescript
printStdout(text: string): void
```

#### printStderr

Write text to stderr without a newline.

```typescript
printStderr(text: string): void
```

#### printSuccess

Print bold green success message to stdout.

```typescript
printSuccess(text: string): void
```

**Example:**
```typescript
printSuccess("Operation completed successfully!");
```

#### printError

Print bold red error message to stderr.

```typescript
printError(text: string): void
```

**Example:**
```typescript
printError("An error occurred!");
```

#### printWarning

Print yellow warning message to stdout.

```typescript
printWarning(text: string): void
```

**Example:**
```typescript
printWarning("This is a warning!");
```

#### printInfo

Print blue info message to stdout.

```typescript
printInfo(text: string): void
```

**Example:**
```typescript
printInfo("Here is some information.");
```

#### readLine

Asynchronously reads a single line from stdin.

```typescript
readLine(): Promise<string>
```

**Example:**
```typescript
const input = await readLine();
console.log("You entered:", input);
```

#### prompt

Displays a prompt message and reads user input.

```typescript
prompt(message: string): Promise<string>
```

**Example:**
```typescript
const name = await prompt("Enter your name");
console.log(`Hello, ${name}!`);
```

#### confirm

Displays a yes/no confirmation prompt.

```typescript
confirm(message: string, default?: boolean): Promise<boolean>
```

**Example:**
```typescript
const result = await confirm("Continue?", true); // true = yes, false = no
if (result) {
  console.log("User confirmed!");
}
```

#### readPassword

Reads a password from stdin with optional character masking. Requires a TTY.

```typescript
readPassword(mask?: string): Promise<string>
```

**Example:**
```typescript
const password = await readPassword("*"); // Shows * for each character
```

#### selectMenu

Displays an interactive selection menu in the terminal. Requires a TTY.

```typescript
selectMenu(message: string, options: string[]): Promise<number>
```

**Example:**
```typescript
const index = await selectMenu("Choose a color", ["Red", "Green", "Blue"]);
console.log(`You selected: ${["Red", "Green", "Blue"][index]}`);
```

#### readMultiline

Reads multiple lines of input until the delimiter is entered.

```typescript
readMultiline(delimiter?: string): Promise<string>
```

**Example:**
```typescript
const text = await readMultiline("DONE");
// Enter "DONE" on a new line to finish
```

#### printProgress

Prints a progress bar to stdout.

```typescript
printProgress(current: number, total: number, width?: number): void
```

**Example:**
```typescript
for (let i = 0; i <= 100; i += 10) {
  printProgress(i, 100, 30);
  await new Promise(r => setTimeout(r, 100));
}
```

#### getSpinnerFrame

Returns a spinner animation frame.

```typescript
getSpinnerFrame(frame: number): string
```

**Example:**
```typescript
const frame = getSpinnerFrame(0); // Returns "⠋"
```

### Terminal Operations

#### getTerminalSize

Gets the current terminal size.

```typescript
getTerminalSize(): TerminalSize
```

**Returns:**
```typescript
interface TerminalSize {
  columns: number;  // Number of columns (width)
  rows: number;     // Number of rows (height)
}
```

#### getTerminalInfo

Gets information about the terminal.

```typescript
getTerminalInfo(): TerminalInfo
```

**Returns:**
```typescript
interface TerminalInfo {
  terminalType: string;   // e.g., "xterm-256color"
  colorSupport: string;   // e.g., "256color", "basic"
}
```

#### clearScreen

Clears the entire screen and moves cursor to home position (0, 0).

```typescript
clearScreen(): void
```

#### moveCursor

Moves the cursor to the specified position.

```typescript
moveCursor(column: number, row: number): void
```

#### showCursor

Shows the cursor (after it was hidden).

```typescript
showCursor(): void
```

#### hideCursor

Hides the cursor.

```typescript
hideCursor(): void
```

#### setCursorShape

Sets the cursor shape using ANSI escape codes.

```typescript
setCursorShape(shape: CursorShape): void

enum CursorShape {
  Block,           // Block cursor (default)
  BlinkingBlock,  // Blinking block cursor
  Underline,      // Underline cursor
  BlinkingUnderline, // Blinking underline cursor
  Bar,            // Vertical bar cursor (I-beam)
  BlinkingBar     // Blinking bar cursor
}
```

#### setTerminalTitle

Sets the terminal window title.

```typescript
setTerminalTitle(title: string): void
```

#### enterAlternateScreen

Enters the alternate screen buffer (useful for full-screen applications).

```typescript
enterAlternateScreen(): void
```

#### leaveAlternateScreen

Leaves the alternate screen buffer and returns to the main screen.

```typescript
leaveAlternateScreen(): void
```

#### enableRawMode

Enables raw mode for the terminal. In raw mode, input is available character-by-character without line buffering.

```typescript
enableRawMode(): void
```

#### disableRawMode

Disables raw mode and restores normal terminal behavior.

```typescript
disableRawMode(): void
```

#### setScrollRegion

Sets the scroll region to a specific area of the terminal.

```typescript
setScrollRegion(top: number, bottom: number): void
```

#### resetScrollRegion

Resets the scroll region to the full terminal.

```typescript
resetScrollRegion(): void
```

#### saveCursorPosition / restoreCursorPosition

Saves and restores the cursor position.

```typescript
saveCursorPosition(): void
restoreCursorPosition(): void
```

#### Cursor Movement Functions

```typescript
moveCursorUp(n: number): void      // Move cursor up n rows
moveCursorDown(n: number): void    // Move cursor down n rows
moveCursorLeft(n: number): void   // Move cursor left n columns
moveCursorRight(n: number): void  // Move cursor right n columns
moveCursorNextLine(n: number): void    // Move to beginning of next line
moveCursorPreviousLine(n: number): void // Move to beginning of previous line
moveCursorToColumn(column: number): void // Move to specific column
moveCursorToRow(row: number): void // Move to specific row
```

#### Line Clearing Functions

```typescript
clearCurrentLine(): void     // Clear current line
clearUntilNewline(): void    // Clear from cursor to end of line
clearFromCursor(): void     // Clear from cursor to beginning of line
```

#### Scroll Functions

```typescript
scrollUp(n: number): void   // Scroll up n lines
scrollDown(n: number): void // Scroll down n lines
```

#### TTY Check Functions

```typescript
isTty(): boolean       // Check if stdout is a TTY
isStderrTty(): boolean // Check if stderr is a TTY
isStdinTty(): boolean  // Check if stdin is a TTY
```

### Process Management

#### execCommand

Execute a command and wait for completion (async).

```typescript
execCommand(command: string, args: string[]): Promise<ProcessStatus>

interface ProcessStatus {
  pid: number;      // Process ID
  success: boolean; // Whether process exited successfully
  code?: number;   // Exit code
}
```

**Example:**
```typescript
const status = await execCommand("ls", ["-la", "/tmp"]);
console.log(`Process exited with code: ${status.code}`);
```

#### spawnWithOptions

Spawn a process with options.

```typescript
spawnWithOptions(options: SpawnOptions): Promise<ProcessStatus>

interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;                           // Working directory
  env?: Record<string, string>;           // Environment variables
  captureStdout?: boolean;                // Capture stdout
  captureStderr?: boolean;                // Capture stderr
}
```

#### spawnWithPipes

Spawn a process with piped stdio for streaming.

```typescript
spawnWithPipes(options: SpawnOptions): Promise<ProcessStatus>
```

#### execSync

Execute a command synchronously and return output.

```typescript
execSync(command: string): ProcessOutput

interface ProcessOutput {
  stdout: string;
  stderr: string;
  code?: number;
  success: boolean;
}
```

#### execSyncWithArgs

Execute a command with arguments synchronously.

```typescript
execSyncWithArgs(command: string, args: string[]): ProcessOutput
```

#### shellEscape

Shell escape a string for safe command execution.

```typescript
shellEscape(input: string): string
```

#### shellEscapeArgs

Shell escape multiple arguments.

```typescript
shellEscapeArgs(args: string[]): string[]
```

#### killProcess

Kill a process by PID.

```typescript
killProcess(pid: number, signal?: string): Promise<boolean>
// signal: "SIGKILL", "SIGTERM", "SIGINT", "SIGHUP"
```

#### ManagedProcess

A class for managing processes with streaming capabilities.

```typescript
const proc = new ManagedProcess("ls", ["-la"]);
const pid = proc.pid();

// Read from stdout
const line = await proc.readStdoutLine();

// Write to stdin
await proc.stdin.write("data\n");

// Wait for completion
const status = await proc.wait();

// Kill the process
await proc.kill();
```

### BufferedReader

A buffered reader class for efficient input reading.

```typescript
const reader = new BufferedReader();

// Read a line
const line = await reader.readLine();

// Read until delimiter
const data = await reader.readUntil(",");

// Read with custom buffer size
const chunk = await reader.read(4096);

// Use as iterator
for await (const line of reader) {
  console.log(line);
}
```

### Error Handling

The library provides custom error types:

```typescript
type StdioError =
  | { type: 'Io', message: string }
  | { type: 'Terminal', message: string }
  | { type: 'Process', message: string }
  | { type: 'Stream', message: string }
  | { type: 'Validation', message: string }
  | { type: 'Buffer', message: string };
```

### Misc

#### getVersion

Returns the current library version.

```typescript
getVersion(): string
```

**Example:**
```typescript
console.log(`Using stdio-napi v${getVersion()}`);
```

## Migration Guide from Node.js `child_process`

### Basic Spawn

**Node.js:**
```javascript
const { spawn } = require('child_process');
const child = spawn('ls', ['-la']);
```

**stdio-napi:**
```typescript
import { execCommand } from 'stdio-napi';
const status = await execCommand('ls', ['-la']);
```

### Spawn with Options

**Node.js:**
```javascript
const { spawn } = require('child_process');
const child = spawn('node', ['script.js'], {
  cwd: '/path/to/dir',
  env: { NODE_ENV: 'production' }
});
```

**stdio-napi:**
```typescript
import { spawnWithOptions } from 'stdio-napi';
const status = await spawnWithOptions({
  command: 'node',
  args: ['script.js'],
  cwd: '/path/to/dir',
  env: { NODE_ENV: 'production' }
});
```

### Exec (Sync)

**Node.js:**
```javascript
const { execSync } = require('child_process');
const output = execSync('ls -la');
```

**stdio-napi:**
```typescript
import { execSync } from 'stdio-napi';
const output = execSync('ls -la');
console.log(output.stdout);
```

### Piped Streams

**Node.js:**
```javascript
const { spawn } = require('child_process');
const child = spawn('ls', ['-la']);
child.stdout.on('data', (data) => console.log(data));
child.stderr.on('data', (data) => console.error(data));
```

**stdio-napi:**
```typescript
import { ManagedProcess } from 'stdio-napi';
const proc = new ManagedProcess('ls', ['-la']);
const line = await proc.readStdoutLine();
await proc.wait();
```

## Platform-Specific Behavior

- **TTY Functions**: Functions like `readPassword`, `selectMenu`, and `confirm` require a terminal (TTY). They will throw an error if not run in interactive mode.
- **Windows**: Some cursor shapes and scroll regions may have limited support on Windows legacy console.
- **Signals**: Signal names are case-insensitive on Unix. On Windows, only `SIGTERM` and `SIGKILL` are supported via `taskkill`.

## License

MIT
