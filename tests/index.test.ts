import { expect, test, describe } from "bun:test";
import {
  getVersion,
  getTerminalSize,
  printSuccess,
  printInfo
} from "../index.js";

describe("stdio-napi Tests", () => {
  test("getVersion should return a string", () => {
    const version = getVersion();
    expect(typeof version).toBe("string");
    expect(version).toMatch(/^\d+\.\d+\.\d+$/);
  });

  test("getTerminalSize should return columns and rows", () => {
    const size = getTerminalSize();
    expect(size).toBeDefined();
    expect(typeof size.columns).toBe("number");
    expect(typeof size.rows).toBe("number");
  });

  test("print functions should exist", () => {
    expect(typeof printSuccess).toBe("function");
    expect(typeof printInfo).toBe("function");
  });
});
