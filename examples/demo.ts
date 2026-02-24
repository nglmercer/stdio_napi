import {
  printStdout,
  printSuccess,
  printError,
  printInfo,
  printWarning,
  prompt,
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

  const shouldClear = await prompt("Should I clear the screen? (y/n)");
  if (shouldClear.toLowerCase() === "y") {
    clearScreen();
    printSuccess("Screen cleared!");
  } else {
    printInfo("Keeping screen content.");
  }
}

main().catch(console.error);
