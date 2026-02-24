# stdio-napi

A powerful NAPI (Node-API) native addon built with Rust for advanced standard input/output and terminal operations in Node.js/Bun.

## Features

- **Standard I/O Wrappers**: Easy-to-use functions for writing to stdout/stderr.
- **Async Stdin**: Non-blocking input reading from Rust.
- **Terminal Operations**: Get terminal size, clear screen, and move cursor.
- **Colored Output**: Built-in support for colored terminal messages (Success, Info, Warning, Error).
- **TypeScript Support**: Automatic type definitions generation.
- **Bun Optimized**: Seamless integration with Bun runtime.

## Installation

```bash
bun install
```

## Building

### Release build

```bash
bun run build
```

### Debug build

```bash
bun run build:debug
```

## Usage Example

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

### Stdio

- `printStdout(text: string)`: Write text to stdout.
- `printStderr(text: string)`: Write text to stderr.
- `printSuccess(text: string)`: Print bold green text.
- `printError(text: string)`: Print bold red text to stderr.
- `printWarning(text: string)`: Print yellow text.
- `printInfo(text: string)`: Print blue text.
- `readLine(): Promise<string>`: Read a line from stdin asynchronously.
- `prompt(message: string): Promise<string>`: Show a prompt and read input.

### Terminal

- `getTerminalSize(): TerminalSize`: Returns `{ columns: number, rows: number }`.
- `clearScreen()`: Clears the terminal screen.
- `moveCursor(column: number, row: number)`: Moves cursor to specified position.

### Misc

- `getVersion()`: returns the current library version.

## License

MIT
