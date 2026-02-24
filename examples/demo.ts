import {
  printStdout,
  printSuccess,
  printError,
  printInfo,
  printWarning,
  prompt,
  confirm,
  readPassword,
  printProgress,
  getTerminalSize,
  clearScreen,
  getVersion,
} from "../index.js";

async function main() {
  console.log(`Starting stdio-napi demo (v${getVersion()})...`);

  const size = getTerminalSize();
  console.log(`Terminal size: ${size.columns}x${size.rows}`);

  printInfo("This is an info message from Rust");
  printSuccess("This is a success message from Rust");
  printWarning("This is a warning message from Rust");
  printError("This is an error message from Rust");

  printStdout("Writing directly to stdout...\n");

  const name = await prompt("What is your name?");
  printSuccess(`Hello, ${name}!`);

  const hasPassword = await confirm("Would you like to enter a secret password?", true);
  if (hasPassword) {
    printInfo("Enter your secret (it will be masked with *):");
    const secret = await readPassword("*");
    printSuccess("Secret received!");
  }

  printInfo("Simulating progress...");
  for (let i = 0; i <= 100; i += 10) {
    printProgress(i, 100);
    await new Promise(r => setTimeout(r, 100));
  }
  printSuccess("Task complete!");
  
  const shouldClear = await confirm("Should I clear the screen?", false);
  if (shouldClear) {
    clearScreen();
    printSuccess("Screen cleared!");
  } else {
    printInfo("Keeping screen content.");
  }
}

main().catch(console.error);
