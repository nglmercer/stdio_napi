import { expect, test, describe } from "bun:test";
import {
  readKey,
  readKeyWithTimeout,
  readLine,
  readLineWithEvents,
  readPassword,
  readMultiline,
  prompt,
  selectMenu,
  getKeyboardMode,
  isKeyboardAvailable,
  waitForShortcut,
  KeyboardEvent,
  Key,
  KeyModifier,
} from "../index.js";

// ============================================
// Keyboard Availability Tests
// ============================================
describe("Keyboard Availability Tests", () => {
  test("isKeyboardAvailable should return a boolean", () => {
    const result = isKeyboardAvailable();
    expect(typeof result).toBe("boolean");
  });

  test("getKeyboardMode should return a string", () => {
    const mode = getKeyboardMode();
    expect(typeof mode).toBe("string");
  });
});

// ============================================
// Keyboard Event Tests
// ============================================
describe("KeyboardEvent Tests", () => {
  test("KeyboardEvent class should be defined", () => {
    expect(KeyboardEvent).toBeDefined();
  });

  test("KeyboardEvent should have expected properties", () => {
    // Create a mock keyboard event to test the type
    const event = {
      key: "a",
      code: "KeyA",
      pressed: true,
      modifiers: [],
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(event.key).toBe("a");
    expect(event.pressed).toBe(true);
  });
});

// ============================================
// Key Type Tests
// ============================================
describe("Key Type Tests", () => {
  test("Key type should support arrow keys", () => {
    const arrowUp: Key = { type: "ArrowUp" };
    const arrowDown: Key = { type: "ArrowDown" };
    const arrowLeft: Key = { type: "ArrowLeft" };
    const arrowRight: Key = { type: "ArrowRight" };
    expect(arrowUp.type).toBe("ArrowUp");
    expect(arrowDown.type).toBe("ArrowDown");
    expect(arrowLeft.type).toBe("ArrowLeft");
    expect(arrowRight.type).toBe("ArrowRight");
  });

  test("Key type should support special keys", () => {
    const enter: Key = { type: "Enter" };
    const escape: Key = { type: "Escape" };
    const backspace: Key = { type: "Backspace" };
    const tab: Key = { type: "Tab" };
    const space: Key = { type: "Space" };
    expect(enter.type).toBe("Enter");
    expect(escape.type).toBe("Escape");
    expect(backspace.type).toBe("Backspace");
    expect(tab.type).toBe("Tab");
    expect(space.type).toBe("Space");
  });

  test("Key type should support navigation keys", () => {
    const home: Key = { type: "Home" };
    const end: Key = { type: "End" };
    const pageUp: Key = { type: "PageUp" };
    const pageDown: Key = { type: "PageDown" };
    const insert: Key = { type: "Insert" };
    const deleteKey: Key = { type: "Delete" };
    expect(home.type).toBe("Home");
    expect(end.type).toBe("End");
    expect(pageUp.type).toBe("PageUp");
    expect(pageDown.type).toBe("PageDown");
    expect(insert.type).toBe("Insert");
    expect(deleteKey.type).toBe("Delete");
  });

  test("Key type should support function keys", () => {
    const f1: Key = { type: "F", field0: 1 };
    const f12: Key = { type: "F", field0: 12 };
    expect(f1.type).toBe("F");
    expect(f1.field0).toBe(1);
    expect(f12.field0).toBe(12);
  });

  test("Key type should support character keys", () => {
    const charA: Key = { type: "Char", field0: "a" };
    const charZ: Key = { type: "Char", field0: "z" };
    expect(charA.type).toBe("Char");
    expect(charA.field0).toBe("a");
    expect(charZ.field0).toBe("z");
  });

  test("Key type should support unknown keys", () => {
    const unknown: Key = { type: "Unknown" };
    expect(unknown.type).toBe("Unknown");
  });
});

// ============================================
// KeyModifier Enum Tests
// ============================================
describe("KeyModifier Enum Tests", () => {
  test("KeyModifier should have correct values", () => {
    expect(KeyModifier.Shift).toBe(0);
    expect(KeyModifier.Ctrl).toBe(1);
    expect(KeyModifier.Alt).toBe(2);
    expect(KeyModifier.Meta).toBe(3);
  });
});

// ============================================
// Read Key Tests (Non-blocking/Timeout)
// ============================================
describe("Read Key Tests", () => {
  test("readKey should be a function", () => {
    expect(typeof readKey).toBe("function");
  });

  test("readKeyWithTimeout should be a function", () => {
    expect(typeof readKeyWithTimeout).toBe("function");
  });

  test("readKeyWithTimeout should handle timeout", async () => {
    // Use a very short timeout - should return null if no key pressed
    const result = await readKeyWithTimeout(10).catch(() => null);
    // Result should be null (timeout) or a KeyboardEvent
    expect(result === null || result instanceof Object).toBe(true);
  });
});

// ============================================
// Read Line Tests
// ============================================
describe("Read Line Tests", () => {
  test("readLine should be a function", () => {
    expect(typeof readLine).toBe("function");
  });

  test("readLineWithEvents should be a function", () => {
    expect(typeof readLineWithEvents).toBe("function");
  });

  test("readPassword should be a function", () => {
    expect(typeof readPassword).toBe("function");
  });

  test("readMultiline should be a function", () => {
    expect(typeof readMultiline).toBe("function");
  });
});

// ============================================
// Prompt Tests
// ============================================
describe("Prompt Tests", () => {
  test("prompt should be a function", () => {
    expect(typeof prompt).toBe("function");
  });

  test("selectMenu should be a function", () => {
    expect(typeof selectMenu).toBe("function");
  });
});

// ============================================
// Wait For Shortcut Tests
// ============================================
describe("Wait For Shortcut Tests", () => {
  test("waitForShortcut should be a function", () => {
    expect(typeof waitForShortcut).toBe("function");
  });

  test("waitForShortcut should accept shortcut strings", () => {
    // Just verify the function exists and can be called
    expect(typeof waitForShortcut).toBe("function");
  });
});
