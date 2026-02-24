/**
 * IPC Server - stdin/stdout communication
 * 
 * Demonstrates inter-process communication via stdin/stdout.
 * 
 * Run modes:
 *   bun run examples/ipc/server.ts                    # Interactive mode
 *   echo '{"cmd":"ping"}' | bun run examples/ipc/server.ts  # Pipe mode
 * 
 * Interactive mode accepts:
 *   - JSON:     {"cmd":"ping"}
 *   - Plain:    ping
 *   - With args: ping arg1 arg2
 */

import { 
  BufferedReader, 
  execSync,
  printInfo,
  printError,
  printSuccess,
  printWarning,
  getVersion,
} from "../../index.js";

// Check if running in pipe mode (stdin has data)
const isPipeMode = !process.stdin.isTTY;

interface IPCCommand {
  cmd: string;
  args?: string[];
  env?: Record<string, string>;
  timeout?: number;
}

interface IPCResponse {
  status: "ok" | "error";
  data?: string;
  error?: string;
  pid?: number;
}

// Parse input into IPCCommand
function parseInput(input: string): IPCCommand {
  const trimmed = input.trim();
  
  // Try JSON first
  if (trimmed.startsWith("{")) {
    try {
      return JSON.parse(trimmed);
    } catch {
      throw new Error("Invalid JSON");
    }
  }
  
  // Parse plain text: "cmd arg1 arg2"
  const parts = trimmed.split(/\s+/);
  const cmd = parts[0] || "";
  const args = parts.slice(1);
  
  return { cmd, args };
}

// Command handlers
const commands: Record<string, (cmd: IPCCommand) => IPCResponse | Promise<IPCResponse>> = {
  ping: () => ({ status: "ok", data: "pong" }),

  echo: (cmd) => ({
    status: "ok",
    data: cmd.args?.join(" ") || "",
  }),

  version: () => ({
    status: "ok",
    data: getVersion(),
  }),

  exec: async (cmd) => {
    if (!cmd.args || cmd.args.length === 0) {
      return { status: "error", error: "exec requires command argument" };
    }
    try {
      const result = execSync(cmd.args[0], cmd.args.slice(1));
      console.log(result)
      return {
        status: "ok",
        data: result.stdout,
        pid: result.pid,
      };
    } catch (e) {
      return { status: "error", error: String(e) };
    }
  },

  env: (cmd) => {
    const envVars = cmd.args || Object.keys(process.env);
    const env: Record<string, string> = {};
    for (const v of envVars) {
      env[v] = process.env[v] || "";
    }
    return { status: "ok", data: JSON.stringify(env, null, 2) };
  },

  sleep: async (cmd) => {
    const ms = parseInt(cmd.args?.[0] || "1000", 10);
    await new Promise(r => setTimeout(r, Math.min(ms, 10000)));
    return { status: "ok", data: `Slept for ${ms}ms` };
  },

  // Send output to stdout
  send: (cmd) => {
    const message = cmd.args?.join(" ") || "";
    console.log(`[SEND] ${message}`);
    return { status: "ok", data: "Message sent" };
  },

  // Receive input from stdin
  receive: async (_cmd: IPCCommand): Promise<IPCResponse> => {
    const reader = new BufferedReader();
    try {
      const line = await reader.readLine();
      return { status: "ok", data: line || "" };
    } catch (e) {
      return { status: "error", error: String(e) };
    }
  },

  // Exit the server
  exit: () => {
    printWarning("Goodbye!");
    process.exit(0);
  },

  // Show help
  help: () => ({
    status: "ok",
    data: `Available commands:
  ping              - Returns "pong"
  echo <args>      - Echo back arguments  
  exec <cmd> [args] - Execute a command
  version          - Get stdio-napi version
  env [vars]       - Get environment variables
  sleep <ms>       - Sleep for milliseconds
  send <msg>       - Send message to stdout
  receive          - Receive input from stdin
  help             - Show this help
  exit             - Exit the server`,
  }),
};

// Process single command
function processCommand(line: string): IPCResponse | Promise<IPCResponse> {
  try {
    const cmd = parseInput(line);
    
    if (!cmd.cmd) {
      return { status: "error", error: "Missing command" };
    }

    const handler = commands[cmd.cmd];
    if (!handler) {
      return { status: "error", error: `Unknown command: ${cmd.cmd}` };
    }

    return handler(cmd);
  } catch (e) {
    return { status: "error", error: `Parse error: ${e}` };
  }
}

// Pipe mode: process single command and exit
async function pipeMode(): Promise<void> {
  const reader = new BufferedReader();
  
  try {
    const line = await reader.readLine();
    
    if (!line || line.trim() === "") {
      console.log(JSON.stringify({ status: "error", error: "No input" }));
      process.exit(1);
    }

    const response = processCommand(line);
    
    if (response instanceof Promise) {
      const resolved = await response;
      console.log(JSON.stringify(resolved));
    } else {
      console.log(JSON.stringify(response));
    }
  } catch (e) {
    console.log(JSON.stringify({ status: "error", error: String(e) }));
  }
}

// Interactive mode: REPL server
async function interactiveMode(): Promise<void> {
  printSuccess("IPC Server running in interactive mode");
  printInfo("Commands: ping, echo, exec, version, env, sleep, send, receive, help, exit");
  printInfo("Usage: ping, echo hello world, exec ls -la, help\n");

  const reader = new BufferedReader();
  let lineNum = 0;

  while (true) {
    try {
      process.stdout.write(`[${++lineNum}]> `);
      const line = await reader.readLine();
      
      if (!line || line.trim() === "") {
        continue;
      }

      const response = processCommand(line);
      
      if (response instanceof Promise) {
        const resolved = await response;
        console.log(JSON.stringify(resolved, null, 2));
      } else {
        console.log(JSON.stringify(response, null, 2));
      }
    } catch (e) {
      printError(`Error: ${e}`);
    }
  }
}

// Main entry
async function main(): Promise<void> {
  printInfo(`IPC Server v${getVersion()}`);
  
  if (isPipeMode) {
    await pipeMode();
  } else {
    await interactiveMode();
  }
}

process.on("SIGINT", () => {
  printWarning("\nShutting down...");
  process.exit(0);
});

main().catch((e) => {
  printError(`Fatal: ${e}`);
  process.exit(1);
});
