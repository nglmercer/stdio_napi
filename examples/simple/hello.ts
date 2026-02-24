/**
 * Simple Hello World Example
 * 
 * Demonstrates basic print functions from stdio-napi.
 * Run: bun run examples/simple/hello.ts
 */

import {
  printStdout,
  printSuccess,
  printError,
  printInfo,
  printWarning,
  getVersion,
} from "../../index.js";

async function main() {
  console.log(`Starting stdio-napi (v${getVersion()})...\n`);

  // Basic stdout print (no newline)
  printStdout("Hello, ");
  printStdout("World!\n");

  // Colored output
  printSuccess("✓ Operation completed successfully!");
  printError("✗ An error occurred!");
  printWarning("⚠ Warning: Please check your input");
  printInfo("ℹ Information: This is a helpful message");
}

main().catch(console.error);
