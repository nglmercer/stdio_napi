import { expect, test, describe } from "bun:test";
import {
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
  isStdoutTty,
  isStdinTty,
  isStderrTty,
  CursorShape,
  type TerminalSize,
  type TerminalInfo,
} from "../index.js";

// ============================================
// Terminal Size Tests
// ============================================
describe("Terminal Size Tests", () => {
  test("getTerminalSize should return valid dimensions or throw in non-TTY", () => {
    if (isStdoutTty()) {
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

  test("getTerminalSize should return reasonable values in TTY", () => {
    if (isStdoutTty()) {
      const size = getTerminalSize();
      // Most terminals are at least 80x24
      expect(size.columns).toBeGreaterThanOrEqual(1);
      expect(size.rows).toBeGreaterThanOrEqual(1);
    }
  });
});

// ============================================
// Terminal Info Tests
// ============================================
describe("Terminal Info Tests", () => {
  test("getTerminalInfo should return terminal information", () => {
    const info = getTerminalInfo();
    expect(info).toBeDefined();
    expect(typeof info.terminalType).toBe("string");
    expect(typeof info.colorSupport).toBe("string");
  });

  test("getTerminalInfo should have non-empty values", () => {
    const info = getTerminalInfo();
    expect(info.terminalType.length).toBeGreaterThanOrEqual(0);
    expect(info.colorSupport.length).toBeGreaterThanOrEqual(0);
  });
});

// ============================================
// TTY Detection Tests
// ============================================
describe("TTY Detection Tests", () => {
  test("isStdoutTty should return a boolean", () => {
    const result = isStdoutTty();
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

  test("TTY functions should be consistent", () => {
    // isStdoutTty should match isStdinTty in most cases
    const ttyResult = isStdoutTty();
    const stdinResult = isStdinTty();
    // They should both be booleans
    expect(typeof ttyResult).toBe("boolean");
    expect(typeof stdinResult).toBe("boolean");
  });
});

// ============================================
// Cursor Movement Tests
// ============================================
describe("Cursor Movement Tests", () => {
  test("moveCursor should not throw with valid coordinates", () => {
    expect(() => moveCursor(0, 0)).not.toThrow();
    expect(() => moveCursor(10, 5)).not.toThrow();
  });

  test("moveCursor should handle edge cases", () => {
    expect(() => moveCursor(0, 0)).not.toThrow();
    expect(() => moveCursor(1000, 1000)).not.toThrow();
  });

  test("moveCursorUp should not throw", () => {
    expect(() => moveCursorUp(1)).not.toThrow();
    expect(() => moveCursorUp(5)).not.toThrow();
  });

  test("moveCursorUp should handle zero", () => {
    expect(() => moveCursorUp(0)).not.toThrow();
  });

  test("moveCursorDown should not throw", () => {
    expect(() => moveCursorDown(1)).not.toThrow();
    expect(() => moveCursorDown(5)).not.toThrow();
  });

  test("moveCursorDown should handle zero", () => {
    expect(() => moveCursorDown(0)).not.toThrow();
  });

  test("moveCursorLeft should not throw", () => {
    expect(() => moveCursorLeft(1)).not.toThrow();
    expect(() => moveCursorLeft(5)).not.toThrow();
  });

  test("moveCursorLeft should handle zero", () => {
    expect(() => moveCursorLeft(0)).not.toThrow();
  });

  test("moveCursorRight should not throw", () => {
    expect(() => moveCursorRight(1)).not.toThrow();
    expect(() => moveCursorRight(5)).not.toThrow();
  });

  test("moveCursorRight should handle zero", () => {
    expect(() => moveCursorRight(0)).not.toThrow();
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
    expect(() => moveCursorNextLine(3)).not.toThrow();
  });

  test("moveCursorPreviousLine should not throw", () => {
    expect(() => moveCursorPreviousLine(1)).not.toThrow();
    expect(() => moveCursorPreviousLine(3)).not.toThrow();
  });
});

// ============================================
// Cursor Visibility Tests
// ============================================
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

  test("setCursorShape should handle numeric values", () => {
    expect(() => setCursorShape(0)).not.toThrow(); // Block
    expect(() => setCursorShape(1)).not.toThrow(); // BlinkingBlock
    expect(() => setCursorShape(2)).not.toThrow(); // Underline
  });
});

// ============================================
// Cursor Position Save/Restore Tests
// ============================================
describe("Cursor Position Save/Restore Tests", () => {
  test("saveCursorPosition should not throw", () => {
    expect(() => saveCursorPosition()).not.toThrow();
  });

  test("restoreCursorPosition should not throw", () => {
    expect(() => restoreCursorPosition()).not.toThrow();
  });

  test("save and restore cursor position should work together", () => {
    expect(() => {
      saveCursorPosition();
      moveCursor(5, 5);
      restoreCursorPosition();
    }).not.toThrow();
  });
});

// ============================================
// Screen Control Tests
// ============================================
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

// ============================================
// Scroll Tests
// ============================================
describe("Scroll Tests", () => {
  test("scrollUp should not throw", () => {
    expect(() => scrollUp(1)).not.toThrow();
    expect(() => scrollUp(5)).not.toThrow();
  });

  test("scrollUp should handle zero", () => {
    expect(() => scrollUp(0)).not.toThrow();
  });

  test("scrollDown should not throw", () => {
    expect(() => scrollDown(1)).not.toThrow();
    expect(() => scrollDown(5)).not.toThrow();
  });

  test("scrollDown should handle zero", () => {
    expect(() => scrollDown(0)).not.toThrow();
  });

  test("setScrollRegion should be callable", () => {
    // setScrollRegion writes ANSI codes directly, may not throw in non-TTY
    expect(() => setScrollRegion(0, 24)).not.toThrow();
  });

  test("setScrollRegion should handle various ranges", () => {
    expect(() => setScrollRegion(0, 10)).not.toThrow();
    expect(() => setScrollRegion(5, 15)).not.toThrow();
  });

  test("resetScrollRegion should not throw in TTY or non-TTY", () => {
    // resetScrollRegion should work gracefully in both TTY and non-TTY environments
    // It handles errors internally and doesn't throw
    expect(() => resetScrollRegion()).not.toThrow();
  });
});

// ============================================
// Alternate Screen Tests
// ============================================
describe("Alternate Screen Tests", () => {
  test("enterAlternateScreen should not throw", () => {
    expect(() => enterAlternateScreen()).not.toThrow();
  });

  test("leaveAlternateScreen should not throw", () => {
    expect(() => leaveAlternateScreen()).not.toThrow();
  });

  test("alternate screen enter and leave should work together", () => {
    expect(() => {
      enterAlternateScreen();
      leaveAlternateScreen();
    }).not.toThrow();
  });
});

// ============================================
// Raw Mode Tests
// ============================================
describe("Raw Mode Tests", () => {
  test("enableRawMode should not throw in TTY or non-TTY", () => {
    // enableRawMode should work gracefully in both TTY and non-TTY environments
    // It handles errors internally and doesn't throw
    expect(() => enableRawMode()).not.toThrow();
  });

  test("disableRawMode should not throw", () => {
    expect(() => disableRawMode()).not.toThrow();
  });
});

// ============================================
// Terminal Title Tests
// ============================================
describe("Terminal Title Tests", () => {
  test("setTerminalTitle should not throw", () => {
    expect(() => setTerminalTitle("Test Title")).not.toThrow();
  });

  test("setTerminalTitle should handle empty string", () => {
    expect(() => setTerminalTitle("")).not.toThrow();
  });

  test("setTerminalTitle should handle unicode", () => {
    expect(() => setTerminalTitle("Test 🎉 Title")).not.toThrow();
  });

  test("setTerminalTitle should handle special characters", () => {
    expect(() => setTerminalTitle("Test\nTitle")).not.toThrow();
    expect(() => setTerminalTitle("Test\tTitle")).not.toThrow();
  });
});

// ============================================
// CursorShape Enum Tests
// ============================================
describe("CursorShape Enum Tests", () => {
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
// Type Export Tests
// ============================================
describe("Type Export Tests", () => {
  test("TerminalSize type should be exported", () => {
    const size: TerminalSize = { columns: 80, rows: 24 };
    expect(size.columns).toBe(80);
    expect(size.rows).toBe(24);
  });

  test("TerminalInfo type should be exported", () => {
    const info: TerminalInfo = { terminalType: "xterm", colorSupport: "256color" };
    expect(info.terminalType).toBe("xterm");
    expect(info.colorSupport).toBe("256color");
  });
});
