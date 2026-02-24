import { execCommand, spawnWithOptions, printSuccess, printError, printInfo } from "../index.js";

async function main() {
  printInfo("Testing child process execution...");

  // Simple ls command
  printInfo("Running 'ls -la'...");
  const status = await execCommand("ls", ["-la"]);
  if (status.success) {
    printSuccess(`Command finished successfully with code ${status.code}`);
  } else {
    printError(`Command failed with code ${status.code}`);
  }

  // Testing spawn with options
  printInfo("Running 'echo $GREETING' with environment variable...");
  const spawnStatus = await spawnWithOptions({
    command: "sh",
    args: ["-c", "echo $GREETING"],
    env: { GREETING: "Hello from stdio-napi!" }
  });

  if (spawnStatus.success) {
    printSuccess("Environment variable test passed!");
  } else {
    printError("Environment variable test failed!");
  }
}

main().catch(console.error);
