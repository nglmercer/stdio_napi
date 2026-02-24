import { expect, test, describe } from "bun:test";
import {
  printStdout,
  printStderr,
  printSuccess,
  printError,
  printWarning,
  printInfo,
  printProgress,
  getSpinnerFrame,
} from "../index.js";

// ============================================
// Print Functions Tests
// ============================================
describe("Print Functions Tests", () => {
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

// ============================================
// Progress Bar Tests
// ============================================
describe("Progress Bar Tests", () => {
  test("printProgress should work with default width", () => {
    expect(() => printProgress(50, 100)).not.toThrow();
  });

  test("printProgress should work with custom width", () => {
    expect(() => printProgress(25, 100, 30)).not.toThrow();
  });

  test("printProgress should handle zero values", () => {
    expect(() => printProgress(0, 100)).not.toThrow();
  });

  test("printProgress should handle complete progress", () => {
    expect(() => printProgress(100, 100)).not.toThrow();
  });

  test("printProgress should handle current > total", () => {
    expect(() => printProgress(150, 100)).not.toThrow();
  });

  test("printProgress should handle very small width", () => {
    expect(() => printProgress(50, 100, 1)).not.toThrow();
  });

  test("printProgress should handle very large width", () => {
    expect(() => printProgress(50, 100, 200)).not.toThrow();
  });

  test("printProgress should handle width of zero", () => {
    expect(() => printProgress(50, 100, 0)).not.toThrow();
  });

  test("printProgress should handle large total values", () => {
    expect(() => printProgress(1000000, 10000000, 50)).not.toThrow();
  });

  test("printProgress should handle equal current and total", () => {
    expect(() => printProgress(50, 50)).not.toThrow();
  });
});

// ============================================
// Spinner Tests
// ============================================
describe("Spinner Tests", () => {
  test("getSpinnerFrame should return valid spinner characters", () => {
    const frames = ["", ""]; // We'll check the return type
    for (let i = 0; i < 10; i++) {
      const frame = getSpinnerFrame(i);
      expect(typeof frame).toBe("string");
      expect(frame.length).toBeGreaterThan(0);
    }
  });

  test("getSpinnerFrame should wrap around for large frame numbers", () => {
    const frame1 = getSpinnerFrame(0);
    const frame11 = getSpinnerFrame(10);
    expect(frame1).toBe(frame11);
  });

  test("getSpinnerFrame should handle large frame numbers", () => {
    const frame1 = getSpinnerFrame(0);
    const frame1000 = getSpinnerFrame(1000);
    expect(frame1).toBe(frame1000);
  });

  test("getSpinnerFrame should return consistent results", () => {
    const frame1 = getSpinnerFrame(5);
    const frame2 = getSpinnerFrame(5);
    expect(frame1).toBe(frame2);
  });

  test("getSpinnerFrame should handle zero", () => {
    const frame = getSpinnerFrame(0);
    expect(typeof frame).toBe("string");
    expect(frame.length).toBeGreaterThan(0);
  });
});

// ============================================
// Unicode Handling Tests
// ============================================
describe("Unicode Handling Tests", () => {
  test("printStdout should handle unicode characters", () => {
    expect(() => printStdout("Hello ")).not.toThrow();
  });

  test("printSuccess should handle unicode characters", () => {
    expect(() => printSuccess("Success")).not.toThrow();
  });

  test("printError should handle unicode characters", () => {
    expect(() => printError("Error")).not.toThrow();
  });

  test("printWarning should handle unicode characters", () => {
    expect(() => printWarning("Warning")).not.toThrow();
  });

  test("printInfo should handle unicode characters", () => {
    expect(() => printInfo("Info")).not.toThrow();
  });

  test("printStderr should handle unicode characters", () => {
    expect(() => printStderr("Error")).not.toThrow();
  });
});

// ============================================
// Empty Input Handling Tests
// ============================================
describe("Empty Input Handling Tests", () => {
  test("printStdout should handle empty string", () => {
    expect(() => printStdout("")).not.toThrow();
  });

  test("printSuccess should handle empty string", () => {
    expect(() => printSuccess("")).not.toThrow();
  });

  test("printError should handle empty string", () => {
    expect(() => printError("")).not.toThrow();
  });

  test("printWarning should handle empty string", () => {
    expect(() => printWarning("")).not.toThrow();
  });

  test("printInfo should handle empty string", () => {
    expect(() => printInfo("")).not.toThrow();
  });

  test("printStderr should handle empty string", () => {
    expect(() => printStderr("")).not.toThrow();
  });
});

// ============================================
// Large Input Handling Tests
// ============================================
describe("Large Input Handling Tests", () => {
  test("printStdout should handle large strings", () => {
    const largeString = "a".repeat(100000);
    expect(() => printStdout(largeString)).not.toThrow();
  });

  test("printStderr should handle large strings", () => {
    const largeString = "a".repeat(100000);
    expect(() => printStderr(largeString)).not.toThrow();
  });

  test("printSuccess should handle large strings", () => {
    const largeString = "a".repeat(10000);
    expect(() => printSuccess(largeString)).not.toThrow();
  });

  test("printError should handle large strings", () => {
    const largeString = "a".repeat(10000);
    expect(() => printError(largeString)).not.toThrow();
  });
});

// ============================================
// Special Characters Tests
// ============================================
describe("Special Characters Tests", () => {
  test("printStdout should handle newlines", () => {
    expect(() => printStdout("hello\nworld")).not.toThrow();
  });

  test("printStdout should handle tabs", () => {
    expect(() => printStdout("hello\tworld")).not.toThrow();
  });

  test("printStdout should handle carriage returns", () => {
    expect(() => printStdout("hello\rworld")).not.toThrow();
  });

  test("printStdout should handle ANSI escape codes", () => {
    expect(() => printStdout("\x1b[31mred\x1b[0m")).not.toThrow();
  });
});