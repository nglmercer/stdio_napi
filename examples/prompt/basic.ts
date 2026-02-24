/**
 * Interactive Prompt Example
 * 
 * Demonstrates user input functions: prompt, confirm, and readPassword.
 * Run: bun run examples/prompt/basic.ts
 */

import {
  prompt,
  confirm,
  readPassword,
  printInfo,
  printSuccess,
  printWarning,
} from "../../index.js";

async function main() {
  printInfo("=== Interactive Prompts Demo ===\n");

  // Simple text prompt
  const name = await prompt("What is your name?");
  printSuccess(`Hello, ${name}!\n`);

  // Confirmation prompt (with default)
  const isConfirmed = await confirm("Do you want to continue?", true);
  
  if (isConfirmed) {
    printSuccess("You chose to continue!\n");
    
    // Password input (masked)
    const hasPassword = await confirm("Do you want to enter a password?", false);
    
    if (hasPassword) {
      printInfo("Enter your secret (will be masked):");
      const secret = await readPassword("*");
      printSuccess("Password received! (length: " + secret.length + " characters)");
    } else {
      printWarning("Skipped password input.");
    }
  } else {
    printWarning("You chose to cancel.");
  }
}

main().catch(console.error);
