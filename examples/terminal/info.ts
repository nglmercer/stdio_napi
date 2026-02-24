/**
 * Terminal Info Example
 * 
 * Demonstrates terminal information and manipulation functions.
 * Run: bun run examples/terminal/info.ts
 */

import {
  getTerminalSize,
  getTerminalInfo,
  clearScreen,
  setTerminalTitle,
  isStdoutTty,
  isStdinTty,
  printInfo,
  printSuccess,
  printWarning,
} from "../../index.js";

function main() {
  printInfo("=== Terminal Info Demo ===\n");

  // Get terminal size
  const size = getTerminalSize();
  printInfo(`Terminal size: ${size.columns} columns x ${size.rows} rows`);

  // Get detailed terminal info
  const info = getTerminalInfo();
  printInfo(`Terminal type: ${info.terminal_type}`);
  printInfo(`Is interactive: ${info.is_interactive}`);

  // Check if running in a TTY
  printInfo(`\nStdout is TTY: ${isStdoutTty()}`);
  printInfo(`Stdin is TTY: ${isStdinTty()}`);

  // Set terminal title
  setTerminalTitle("stdio-napi demo");
  printSuccess("\n✓ Terminal title set!");
}

main();
