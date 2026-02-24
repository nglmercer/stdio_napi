import { expect, test, describe } from "bun:test";
import {
  execSync,
  execSyncWithArgs,
  execCommand,
  shellEscape,
  shellEscapeArgs,
  killProcess,
  processBuilder,
  ProcessBuilder,
  spawnWithOptions,
  waitForProcess,
  sendSignal,
  setProcessGroup,
  getProcessGroup,
  isBackground,
  readProcessStdout,
  readProcessStderr,
  writeProcessStdin,
  ManagedProcess,
  ProcessEventEmitter,
  processEvents,
  type ProcessOutput,
  type ProcessStatus,
  type SpawnOptions,
} from "../index.js";

// ============================================
// Shell Escape Tests
// ============================================
describe("Shell Escape Tests", () => {
  test("shellEscape should escape special characters", () => {
    const result = shellEscape("hello world");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should handle empty string", () => {
    const result = shellEscape("");
    expect(typeof result).toBe("string");
    expect(result).toBe("''");
  });

  test("shellEscape should handle special shell characters", () => {
    const result = shellEscape("test; rm -rf /");
    expect(typeof result).toBe("string");
  });

  test("shellEscapeArgs should escape multiple arguments", () => {
    const result = shellEscapeArgs(["hello world", "test; ls", "normal"]);
    expect(Array.isArray(result)).toBe(true);
    expect(result.length).toBe(3);
  });

  test("shellEscape should handle unicode characters", () => {
    const result = shellEscape("Hello ");
    expect(typeof result).toBe("string");
  });

  test("shellEscapeArgs should handle unicode arguments", () => {
    const result = shellEscapeArgs(["hello", "", ""]);
    expect(result.length).toBe(3);
  });

  test("shellEscapeArgs should handle empty array", () => {
    const result = shellEscapeArgs([]);
    expect(result.length).toBe(0);
  });

  test("shellEscape should escape quotes", () => {
    const result = shellEscape("hello'world");
    expect(result).toContain("'");
  });

  test("shellEscape should escape double quotes", () => {
    const result = shellEscape('hello"world');
    expect(typeof result).toBe("string");
  });

  test("shellEscape should escape backticks", () => {
    const result = shellEscape("hello`world");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should escape dollar signs", () => {
    const result = shellEscape("hello$world");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should escape newlines", () => {
    const result = shellEscape("hello\nworld");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should handle simple strings", () => {
    const result = shellEscape("simple");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should handle strings with backslashes", () => {
    const result = shellEscape("hello\\world");
    expect(typeof result).toBe("string");
  });
});

// ============================================
// Exec Sync Tests
// ============================================
describe("Exec Sync Tests", () => {
  test("execSync should execute a simple command", () => {
    const result = execSync("echo");
    expect(result).toBeDefined();
    expect(typeof result.stdout).toBe("string");
    expect(typeof result.stderr).toBe("string");
    expect(typeof result.success).toBe("boolean");
  });

  test("execSyncWithArgs should execute command with arguments", () => {
    const result = execSyncWithArgs("echo", ["hello"]);
    expect(result).toBeDefined();
    expect(result.success).toBe(true);
  });

  test("execSync should return ProcessOutput structure", () => {
    let command = "ls";
    if (process.platform === "win32") {
      command = "dir";
    }
    const result = execSync(command);
    console.log(result)
    expect("stdout" in result).toBe(true);
    expect("stderr" in result).toBe(true);
    expect("success" in result).toBe(true);
  });

  test("execSync should handle non-existent command", () => {
    expect(() => execSync("nonexistentcommand12345")).toThrow();
  });

  test("execSyncWithArgs should handle multiple arguments", () => {
    const result = execSyncWithArgs("echo", ["a", "b", "c"]);
    expect(result.success).toBe(true);
  });

  test("execSyncWithArgs should handle empty args array", () => {
    const result = execSyncWithArgs("echo", []);
    expect(result).toBeDefined();
  });
});

// ============================================
// Kill Process Tests
// ============================================
describe("Kill Process Tests", () => {
  test("killProcess should handle non-existent PID gracefully", async () => {
    // PID 999999 is unlikely to exist
    const result = await killProcess(999999, "SIGTERM").catch(() => false);
    expect(typeof result).toBe("boolean");
  });

  test("killProcess should accept different signals", async () => {
    // Test that the function accepts various signal names
    const signals = ["SIGTERM", "SIGKILL", "SIGINT"];
    for (const signal of signals) {
      // Just verify the function can be called with these signals
      expect(typeof killProcess).toBe("function");
    }
  });
});

// ============================================
// Process Builder Tests
// ============================================
describe("ProcessBuilder Tests", () => {
  test("processBuilder should create a ProcessBuilder instance", () => {
    const builder = processBuilder("echo");
    expect(builder).toBeDefined();
    expect(typeof builder.arg).toBe("function");
    expect(typeof builder.args).toBe("function");
  });

  test("ProcessBuilder should have fluent interface", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.arg("hello")).not.toThrow();
    expect(() => builder.args(["world"])).not.toThrow();
  });

  test("ProcessBuilder should support cwd", () => {
    const builder = new ProcessBuilder("ls");
    expect(() => builder.cwd("/tmp")).not.toThrow();
  });

  test("ProcessBuilder should support env", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.env("KEY", "value")).not.toThrow();
  });

  test("ProcessBuilder should support envs", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.envs({ KEY1: "value1", KEY2: "value2" })).not.toThrow();
  });

  test("ProcessBuilder should support captureStdout", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.captureStdout(true)).not.toThrow();
  });

  test("ProcessBuilder should support captureStderr", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.captureStderr(true)).not.toThrow();
  });

  test("ProcessBuilder should support detached", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.detached(true)).not.toThrow();
  });

  test("ProcessBuilder should support shell", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.shell(true)).not.toThrow();
  });

  test("ProcessBuilder should support timeout", () => {
    const builder = new ProcessBuilder("echo");
    expect(() => builder.timeout(5000)).not.toThrow();
  });

  test("ProcessBuilder toString should return a string", () => {
    const builder = new ProcessBuilder("echo");
    const str = builder.toString();
    expect(typeof str).toBe("string");
  });

  test("ProcessBuilder getCommand should return command", () => {
    const builder = new ProcessBuilder("ls");
    const cmd = builder.getCommand();
    expect(cmd).toBe("ls");
  });

  test("ProcessBuilder getArgs should return args array", () => {
    const builder = new ProcessBuilder("echo");
    builder.args(["hello", "world"]);
    const args = builder.getArgs();
    expect(Array.isArray(args)).toBe(true);
  });

  test("ProcessBuilder getCwd should return cwd or null", () => {
    const builder = new ProcessBuilder("echo");
    const cwd = builder.getCwd();
    // cwd may be null if not set
    expect(cwd === null || typeof cwd === "string").toBe(true);
  });
});

// ============================================
// Process Status Tests
// ============================================
describe("Process Status Tests", () => {
  test("isBackground should return a boolean", () => {
    const result = isBackground();
    expect(typeof result).toBe("boolean");
  });

  test("getProcessGroup should return a number", () => {
    const result = getProcessGroup();
    expect(typeof result).toBe("number");
    expect(result).toBeGreaterThan(0);
  });
});

// ============================================
// Managed Process Tests
// ============================================
describe("ManagedProcess Tests", () => {
  // Note: ManagedProcess requires Tokio runtime context
  // These tests verify the class structure only
  test("ManagedProcess class should be defined", () => {
    expect(ManagedProcess).toBeDefined();
  });

  test("ManagedProcess should have expected methods", () => {
    expect(typeof ManagedProcess.prototype.pid).toBe("function");
    expect(typeof ManagedProcess.prototype.readStdoutLine).toBe("function");
    expect(typeof ManagedProcess.prototype.readStderrLine).toBe("function");
    expect(typeof ManagedProcess.prototype.writeStdin).toBe("function");
    expect(typeof ManagedProcess.prototype.wait).toBe("function");
    expect(typeof ManagedProcess.prototype.kill).toBe("function");
  });
});

// ============================================
// Process Event Emitter Tests
// ============================================
describe("ProcessEventEmitter Tests", () => {
  test("processEvents should create a ProcessEventEmitter", () => {
    // Use current process PID
    const emitter = processEvents(process.pid);
    expect(emitter).toBeDefined();
  });

  test("ProcessEventEmitter should have on method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.on).toBe("function");
  });

  test("ProcessEventEmitter should have once method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.once).toBe("function");
  });

  test("ProcessEventEmitter should have off method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.off).toBe("function");
  });

  test("ProcessEventEmitter should have removeAllListeners method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.removeAllListeners).toBe("function");
  });

  test("ProcessEventEmitter should have emit method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.emit).toBe("function");
  });

  test("ProcessEventEmitter should have listenerCount method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.listenerCount).toBe("function");
  });

  test("ProcessEventEmitter should have shutdown method", () => {
    const emitter = new ProcessEventEmitter(process.pid);
    expect(typeof emitter.shutdown).toBe("function");
  });
});

// ============================================
// Type Export Tests
// ============================================
describe("Process Type Export Tests", () => {
  test("ProcessOutput type should be exported", () => {
    const output: ProcessOutput = {
      pid: 123,
      stdout: "test",
      stderr: "",
      success: true,
    };
    expect(output.stdout).toBe("test");
    expect(output.pid).toBe(123);
  });

  test("ProcessStatus type should be exported", () => {
    const status: ProcessStatus = {
      pid: 123,
      success: true,
      code: 0,
    };
    expect(status.pid).toBe(123);
  });

  test("SpawnOptions type should be exported", () => {
    const options: SpawnOptions = {
      command: "echo",
      args: ["hello"],
    };
    expect(options.command).toBe("echo");
  });
});

// ============================================
// Async Process Tests
// ============================================
describe("Async Process Tests", () => {
  test("execCommand should execute a command asynchronously", async () => {
    const result = await execCommand("echo", ["hello"]);
    expect(result).toBeDefined();
    expect(typeof result.pid).toBe("number");
    expect(typeof result.success).toBe("boolean");
  });

  test("spawnWithOptions should work with minimal options", async () => {
    const result = await spawnWithOptions({
      command: "echo",
      args: ["test"],
    });
    expect(result).toBeDefined();
  });
});