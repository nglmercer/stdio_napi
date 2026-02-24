import { expect, test, describe, beforeAll } from "bun:test";
import {
  getVersion,
  getTerminalSize,
  clearScreen,
  moveCursor,
  printStdout,
  printStderr,
  printSuccess,
  printError,
  printWarning,
  printInfo,
  readLine,
  prompt
} from "../index.js";

describe("stdio-napi Initialize Test", () => {
  test("module should be initialized and exported functions should exist", () => {
    // Test that all expected functions are exported and are functions
    expect(typeof getVersion).toBe("function");
    expect(typeof getTerminalSize).toBe("function");
    expect(typeof clearScreen).toBe("function");
    expect(typeof moveCursor).toBe("function");
    expect(typeof printStdout).toBe("function");
    expect(typeof printStderr).toBe("function");
    expect(typeof printSuccess).toBe("function");
    expect(typeof printError).toBe("function");
    expect(typeof printWarning).toBe("function");
    expect(typeof printInfo).toBe("function");
    expect(typeof readLine).toBe("function");
    expect(typeof prompt).toBe("function");
  });
});

describe("stdio-napi Version Tests", () => {
  test("getVersion should return a string", () => {
    const version = getVersion();
    expect(typeof version).toBe("string");
    expect(version).toMatch(/^\d+\.\d+\.\d+$/);
  });

  test("getVersion should return non-empty string", () => {
    const version = getVersion();
    expect(version.length).toBeGreaterThan(0);
  });
});

describe("stdio-napi Terminal Tests", () => {
  test("getTerminalSize should return columns and rows", () => {
    const size = getTerminalSize();
    expect(size).toBeDefined();
    expect(typeof size.columns).toBe("number");
    expect(typeof size.rows).toBe("number");
    expect(size.columns).toBeGreaterThan(0);
    expect(size.rows).toBeGreaterThan(0);
  });

  test("clearScreen should not throw", () => {
    expect(() => clearScreen()).not.toThrow();
  });

  test("moveCursor should not throw with valid coordinates", () => {
    expect(() => moveCursor(0, 0)).not.toThrow();
  });
});

describe("stdio-napi Print Functions Tests", () => {
  test("printStdout should be callable", () => {
    expect(() => printStdout("test")).not.toThrow();
  });

  test("printStderr should be callable", () => {
    expect(() => printStderr("test")).not.toThrow();
  });

  test("printSuccess should be callable", () => {
    expect(() => printSuccess("test")).not.toThrow();
  });

  test("printError should be callable", () => {
    expect(() => printError("test")).not.toThrow();
  });

  test("printWarning should be callable", () => {
    expect(() => printWarning("test")).not.toThrow();
  });

  test("printInfo should be callable", () => {
    expect(() => printInfo("test")).not.toThrow();
  });
});
