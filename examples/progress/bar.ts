/**
 * Progress Bar Example
 * 
 * Demonstrates the progress bar functionality.
 * Run: bun run examples/progress/bar.ts
 */

import { printProgress, printSuccess, printInfo } from "../../index.js";

async function main() {
  printInfo("=== Progress Bar Demo ===\n");

  // Example 1: Default progress bar (20 chars wide)
  printInfo("Downloading file (default width)...");
  for (let i = 0; i <= 100; i += 10) {
    printProgress(i, 100);
    await new Promise(r => setTimeout(r, 100));
  }
  printSuccess("✓ Download complete!\n");

  // Example 2: Custom width progress bar
  printInfo("Installing packages (custom width)...");
  const packages = ["react", "vue", "angular", "svelte", "solid"];
  const total = packages.length;
  
  for (let i = 0; i < packages.length; i++) {
    const progress = ((i + 1) / total) * 100;
    printProgress(progress, 100, 30);
    await new Promise(r => setTimeout(r, 300));
  }
  printSuccess("✓ All packages installed!\n");

  // Example 3: Quick progress simulation
  printInfo("Processing data...");
  for (let i = 0; i <= 100; i += 5) {
    printProgress(i, 100, 15);
    await new Promise(r => setTimeout(r, 50));
  }
  printSuccess("✓ Processing complete!");
}

main().catch(console.error);
