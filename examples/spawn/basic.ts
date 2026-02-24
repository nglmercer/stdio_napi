/**
 * Basic Spawn Example
 * 
 * Demonstrates how to spawn child processes.
 * Run: bun run examples/spawn/basic.ts
 */

import { execCommand, spawnWithOptions, printInfo, printSuccess, printError } from "../../index.js";

async function main() {
  printInfo("=== Basic Spawn Demo ===\n");

  // Example 1: Simple command with execCommand
  printInfo("Running 'echo hello world'...");
  const result = await execCommand("echo", ["hello", "world"]);
  
  if (result.success) {
    printSuccess(`✓ Command succeeded (exit code: ${result.code})`);
  } else {
    printError(`✗ Command failed (exit code: ${result.code})`);
  }

  printInfo("\n--- Spawn with options ---\n");

  // Example 2: Spawn with environment variables
  printInfo("Running command with custom environment...");
  const spawnResult = await spawnWithOptions({
    command: "sh",
    args: ["-c", "echo Hello $NAME"],
    env: { NAME: "stdio-napi user" }
  });

  if (spawnResult.success) {
    printSuccess("✓ Spawn with env succeeded!");
  } else {
    printError("✗ Spawn with env failed!");
  }
}

main().catch(console.error);
