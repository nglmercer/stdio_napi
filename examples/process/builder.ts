/**
 * Process Builder Example
 * 
 * Demonstrates the ProcessBuilder fluent API for building commands.
 * Run: bun run examples/process/builder.ts
 */

import { ProcessBuilder, printInfo, printSuccess, printError } from "../../index.js";

async function main() {
  printInfo("=== Process Builder Demo ===\n");

  // Example 1: Basic builder
  printInfo("Example 1: Basic command");
  const basic = new ProcessBuilder("echo");
  basic.arg("Hello from ProcessBuilder!");
  const result1 = await basic.spawn();
  if (result1.success) {
    printSuccess("✓ Basic builder succeeded");
  }

  // Example 2: With arguments
  printInfo("\nExample 2: Multiple arguments");
  const withArgs = new ProcessBuilder("echo")
    .args(["Hello", "World", "from", "stdio-napi"]);
  const result2 = await withArgs.spawn();
  if (result2.success) {
    printSuccess("✓ Multiple args succeeded");
  }

  // Example 3: With environment variables
  printInfo("\nExample 3: Environment variables");
  const withEnv = new ProcessBuilder("sh")
    .args(["-c", "echo $GREETING $NAME"])
    .env("GREETING", "Hello")
    .env("NAME", "Developer");
  const result3 = await withEnv.spawn();
  if (result3.success) {
    printSuccess("✓ Environment variables worked");
  }

  // Example 4: With working directory
  printInfo("\nExample 4: Working directory");
  const withCwd = new ProcessBuilder("pwd").cwd("/tmp");
  const result4 = await withCwd.spawn();
  if (result4.success) {
    printSuccess("✓ Working directory changed");
  }

  // Example 5: Capture output
  printInfo("\nExample 5: Capture stdout");
  const capture = new ProcessBuilder("echo")
    .arg("This is captured output")
    .captureStdout(true);
  const result5 = await capture.spawn();
  if (result5.stdout) {
    printSuccess(`✓ Captured: ${result5.stdout.trim()}`);
  }
}

main().catch(console.error);
