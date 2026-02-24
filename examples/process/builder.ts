/**
 * Process Builder Configuration Example
 * 
 * Demonstrates the ProcessBuilder API for building commands.
 * Run: bun run examples/process/builder.ts
 */

import { ProcessBuilder, printInfo, printSuccess } from "../../index.js";

function main() {
  printInfo("=== Process Builder Demo ===\n");

  // Example 1: Basic command
  printInfo("Example 1: Basic command");
  const basic = new ProcessBuilder("echo");
  basic.arg("Hello from ProcessBuilder!");
  printSuccess(`Command: ${basic.getCommand()}`);
  printSuccess(`Args: ${JSON.stringify(basic.getArgs())}`);

  // Example 2: Multiple arguments
  printInfo("\nExample 2: Multiple arguments");
  const withArgs = new ProcessBuilder("echo");
  withArgs.args(["Hello", "World", "from", "stdio-napi"]);
  printSuccess(`Command: ${withArgs.getCommand()}`);
  printSuccess(`Args: ${JSON.stringify(withArgs.getArgs())}`);

  // Example 3: Environment variables
  printInfo("\nExample 3: Environment variables");
  const withEnv = new ProcessBuilder("sh");
  withEnv.args(["-c", "echo $GREETING"]);
  withEnv.env("GREETING", "Hello");
  withEnv.env("NAME", "Developer");
  printSuccess(`Command: ${withEnv.getCommand()}`);
  printSuccess(`CWD: ${withEnv.getCwd()}`);

  // Example 4: Working directory
  printInfo("\nExample 4: Working directory");
  const withCwd = new ProcessBuilder("ls");
  withCwd.cwd("/tmp");
  printSuccess(`CWD set to: ${withCwd.getCwd()}`);

  // Example 5: Capture options
  printInfo("\nExample 5: Output capture options");
  const capture = new ProcessBuilder("echo");
  capture.arg("test");
  capture.captureStdout(true);
  capture.captureStderr(true);
  printSuccess(`Builder configured for output capture`);

  printSuccess("\n✓ All ProcessBuilder configurations demonstrated!");
  printInfo("\nTip: Use spawnWithOptions() or execCommand() to execute built commands.");
}

main();
