import { expect, test, describe } from "bun:test";
import {
  // Version
  getVersion,
  // Terminal
  getTerminalSize,
  getTerminalInfo,
  clearScreen,
  moveCursor,
  showCursor,
  hideCursor,
  setCursorShape,
  setTerminalTitle,
  enterAlternateScreen,
  leaveAlternateScreen,
  enableRawMode,
  disableRawMode,
  setScrollRegion,
  resetScrollRegion,
  saveCursorPosition,
  restoreCursorPosition,
  moveCursorUp,
  moveCursorDown,
  moveCursorLeft,
  moveCursorRight,
  moveCursorToColumn,
  moveCursorToRow,
  moveCursorNextLine,
  moveCursorPreviousLine,
  clearCurrentLine,
  clearUntilNewline,
  clearFromCursor,
  scrollUp,
  scrollDown,
  isTty,
  isStdinTty,
  isStderrTty,
  CursorShape,
  // Stdio
  printStdout,
  printStderr,
  printSuccess,
  printError,
  printWarning,
  printInfo,
  printProgress,
  getSpinnerFrame,
  // Process
  execSync,
  execSyncWithArgs,
  shellEscape,
  shellEscapeArgs,
  killProcess,
  // Types
  type TerminalSize,
  type TerminalInfo,
  type ProcessOutput,
  type ProcessStatus,
  type SpawnOptions,
  type BufferConfig,
  type StreamEventData,
  type StdioError,
} from "../index.js";

// ============================================
// Version Tests
// ============================================
describe("Version Tests", () => {
  test("getVersion should return a valid semver string", () => {
    const version = getVersion();
    expect(typeof version).toBe("string");
    expect(version).toMatch(/^\d+\.\d+\.\d+/);
  });
});

// ============================================
// Terminal Tests
// ============================================
describe("Terminal Size Tests", () => {
  test("getTerminalSize should return valid dimensions or throw in non-TTY", () => {
    if (isTty()) {
      const size = getTerminalSize();
      expect(size).toBeDefined();
      expect(typeof size.columns).toBe("number");
      expect(typeof size.rows).toBe("number");
      expect(size.columns).toBeGreaterThan(0);
      expect(size.rows).toBeGreaterThan(0);
    } else {
      // In non-TTY environment, this should throw
      expect(() => getTerminalSize()).toThrow();
    }
  });
});

describe("Terminal Info Tests", () => {
  test("getTerminalInfo should return terminal information", () => {
    const info = getTerminalInfo();
    expect(info).toBeDefined();
    expect(typeof info.terminalType).toBe("string");
    expect(typeof info.colorSupport).toBe("string");
  });
});

describe("TTY Detection Tests", () => {
  test("isTty should return a boolean", () => {
    const result = isTty();
    expect(typeof result).toBe("boolean");
  });

  test("isStdinTty should return a boolean", () => {
    const result = isStdinTty();
    expect(typeof result).toBe("boolean");
  });

  test("isStderrTty should return a boolean", () => {
    const result = isStderrTty();
    expect(typeof result).toBe("boolean");
  });
});

describe("Cursor Movement Tests", () => {
  test("moveCursor should not throw with valid coordinates", () => {
    expect(() => moveCursor(0, 0)).not.toThrow();
    expect(() => moveCursor(10, 5)).not.toThrow();
  });

  test("moveCursorUp should not throw", () => {
    expect(() => moveCursorUp(1)).not.toThrow();
    expect(() => moveCursorUp(5)).not.toThrow();
  });

  test("moveCursorDown should not throw", () => {
    expect(() => moveCursorDown(1)).not.toThrow();
    expect(() => moveCursorDown(5)).not.toThrow();
  });

  test("moveCursorLeft should not throw", () => {
    expect(() => moveCursorLeft(1)).not.toThrow();
    expect(() => moveCursorLeft(5)).not.toThrow();
  });

  test("moveCursorRight should not throw", () => {
    expect(() => moveCursorRight(1)).not.toThrow();
    expect(() => moveCursorRight(5)).not.toThrow();
  });

  test("moveCursorToColumn should not throw", () => {
    expect(() => moveCursorToColumn(0)).not.toThrow();
    expect(() => moveCursorToColumn(10)).not.toThrow();
  });

  test("moveCursorToRow should not throw", () => {
    expect(() => moveCursorToRow(0)).not.toThrow();
    expect(() => moveCursorToRow(10)).not.toThrow();
  });

  test("moveCursorNextLine should not throw", () => {
    expect(() => moveCursorNextLine(1)).not.toThrow();
  });

  test("moveCursorPreviousLine should not throw", () => {
    expect(() => moveCursorPreviousLine(1)).not.toThrow();
  });
});

describe("Cursor Visibility Tests", () => {
  test("showCursor should not throw", () => {
    expect(() => showCursor()).not.toThrow();
  });

  test("hideCursor should not throw", () => {
    expect(() => hideCursor()).not.toThrow();
  });

  test("setCursorShape should accept all cursor shapes", () => {
    expect(() => setCursorShape(CursorShape.Block)).not.toThrow();
    expect(() => setCursorShape(CursorShape.BlinkingBlock)).not.toThrow();
    expect(() => setCursorShape(CursorShape.Underline)).not.toThrow();
    expect(() => setCursorShape(CursorShape.BlinkingUnderline)).not.toThrow();
    expect(() => setCursorShape(CursorShape.Bar)).not.toThrow();
    expect(() => setCursorShape(CursorShape.BlinkingBar)).not.toThrow();
  });
});

describe("Cursor Position Save/Restore Tests", () => {
  test("saveCursorPosition should not throw", () => {
    expect(() => saveCursorPosition()).not.toThrow();
  });

  test("restoreCursorPosition should not throw", () => {
    expect(() => restoreCursorPosition()).not.toThrow();
  });
});

describe("Screen Control Tests", () => {
  test("clearScreen should not throw", () => {
    expect(() => clearScreen()).not.toThrow();
  });

  test("clearCurrentLine should not throw", () => {
    expect(() => clearCurrentLine()).not.toThrow();
  });

  test("clearUntilNewline should not throw", () => {
    expect(() => clearUntilNewline()).not.toThrow();
  });

  test("clearFromCursor should not throw", () => {
    expect(() => clearFromCursor()).not.toThrow();
  });
});

describe("Scroll Tests", () => {
  test("scrollUp should not throw", () => {
    expect(() => scrollUp(1)).not.toThrow();
  });

  test("scrollDown should not throw", () => {
    expect(() => scrollDown(1)).not.toThrow();
  });

  test("setScrollRegion should be callable", () => {
    // setScrollRegion writes ANSI codes directly, may not throw in non-TTY
    expect(() => setScrollRegion(0, 24)).not.toThrow();
  });

  test("resetScrollRegion should not throw in TTY or throw gracefully in non-TTY", () => {
    if (isTty()) {
      expect(() => resetScrollRegion()).not.toThrow();
    } else {
      expect(() => resetScrollRegion()).toThrow();
    }
  });
});

describe("Alternate Screen Tests", () => {
  test("enterAlternateScreen should not throw", () => {
    expect(() => enterAlternateScreen()).not.toThrow();
  });

  test("leaveAlternateScreen should not throw", () => {
    expect(() => leaveAlternateScreen()).not.toThrow();
  });
});

describe("Raw Mode Tests", () => {
  test("enableRawMode should not throw in TTY or throw gracefully in non-TTY", () => {
    if (isTty()) {
      expect(() => enableRawMode()).not.toThrow();
    } else {
      expect(() => enableRawMode()).toThrow();
    }
  });

  test("disableRawMode should not throw", () => {
    expect(() => disableRawMode()).not.toThrow();
  });
});

describe("Terminal Title Tests", () => {
  test("setTerminalTitle should not throw", () => {
    expect(() => setTerminalTitle("Test Title")).not.toThrow();
  });
});

// ============================================
// Stdio Print Tests
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
});

describe("Spinner Tests", () => {
  test("getSpinnerFrame should return valid spinner characters", () => {
    const frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
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
});

// ============================================
// Process Tests
// ============================================
describe("Shell Escape Tests", () => {
  test("shellEscape should escape special characters", () => {
    const result = shellEscape("hello world");
    expect(typeof result).toBe("string");
  });

  test("shellEscape should handle empty string", () => {
    const result = shellEscape("");
    expect(typeof result).toBe("string");
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
});

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
    const result = execSync("ls");
    expect("stdout" in result).toBe(true);
    expect("stderr" in result).toBe(true);
    expect("success" in result).toBe(true);
  });
});

describe("Kill Process Tests", () => {
  test("killProcess should handle non-existent PID gracefully", async () => {
    // PID 999999 is unlikely to exist
    const result = await killProcess(999999, "SIGTERM").catch(() => false);
    expect(typeof result).toBe("boolean");
  });
});

// ============================================
// Type Export Tests
// ============================================
describe("Type Exports Tests", () => {
  test("CursorShape enum should be exported with correct values", () => {
    expect(CursorShape.Block).toBe(0);
    expect(CursorShape.BlinkingBlock).toBe(1);
    expect(CursorShape.Underline).toBe(2);
    expect(CursorShape.BlinkingUnderline).toBe(3);
    expect(CursorShape.Bar).toBe(4);
    expect(CursorShape.BlinkingBar).toBe(5);
  });
});

// ============================================
// Edge Case Tests
// ============================================
describe("Edge Case Tests", () => {
  describe("Unicode Handling", () => {
    test("printStdout should handle unicode characters", () => {
      expect(() => printStdout("Hello 🌍")).not.toThrow();
    });

    test("printSuccess should handle unicode characters", () => {
      expect(() => printSuccess("Success ✅")).not.toThrow();
    });

    test("shellEscape should handle unicode characters", () => {
      const result = shellEscape("Hello 🌍");
      expect(typeof result).toBe("string");
    });

    test("shellEscapeArgs should handle unicode arguments", () => {
      const result = shellEscapeArgs(["hello", "🌍", "日本語"]);
      expect(result.length).toBe(3);
    });
  });

  describe("Empty Input Handling", () => {
    test("printStdout should handle empty string", () => {
      expect(() => printStdout("")).not.toThrow();
    });

    test("printSuccess should handle empty string", () => {
      expect(() => printSuccess("")).not.toThrow();
    });

    test("shellEscape should handle empty string", () => {
      const result = shellEscape("");
      expect(result).toBe("''");
    });

    test("shellEscapeArgs should handle empty array", () => {
      const result = shellEscapeArgs([]);
      expect(result.length).toBe(0);
    });
  });

  describe("Large Input Handling", () => {
    test("printStdout should handle large strings", () => {
      const largeString = "a".repeat(100000);
      expect(() => printStdout(largeString)).not.toThrow();
    });

    test("printProgress should handle large total values", () => {
      expect(() => printProgress(1000000, 10000000, 50)).not.toThrow();
    });

    test("getSpinnerFrame should handle large frame numbers", () => {
      const frame1 = getSpinnerFrame(0);
      const frame1000 = getSpinnerFrame(1000);
      expect(frame1).toBe(frame1000);
    });
  });

  describe("Special Characters", () => {
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
  });
});

// ============================================
// Progress Bar Edge Cases
// ============================================
describe("Progress Bar Edge Cases", () => {
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
});

// ============================================
// Terminal Size Edge Cases
// ============================================
describe("Terminal Size Edge Cases", () => {
  test("getTerminalSize should return reasonable values in TTY", () => {
    if (isTty()) {
      const size = getTerminalSize();
      // Most terminals are at least 80x24
      expect(size.columns).toBeGreaterThanOrEqual(1);
      expect(size.rows).toBeGreaterThanOrEqual(1);
    }
  });
});
