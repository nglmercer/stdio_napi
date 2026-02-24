/**
 * Logging Example
 * 
 * Demonstrates different log levels using colored output.
 * Run: bun run examples/simple/logs.ts
 */

import {
  printInfo,
  printSuccess,
  printWarning,
  printError,
  printStdout,
} from "../../index.js";

function main() {
  printInfo("=== Logging Levels Demo ===\n");

  // Different log levels
  printInfo("ℹ INFO: Application started");
  printSuccess("✓ SUCCESS: User logged in successfully");
  printWarning("⚠ WARNING: Session expiring soon");
  printError("✗ ERROR: Failed to connect to database");

  printStdout("\n--- Log with timestamps ---\n");

  // Simulated log with timestamp
  const now = new Date().toISOString();
  printInfo(`[${now}] INFO: Processing request...`);
  printSuccess(`[${now}] SUCCESS: Request completed`);
  
  printStdout("\n--- Error handling example ---\n");

  // Simulated error
  const errorCode = 404;
  const errorMsg = "Resource not found";
  printError(`[ERROR] Code: ${errorCode}, Message: ${errorMsg}`);
}

main();
