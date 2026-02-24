import { expect, test, describe } from "bun:test";
import {
  enableMouse,
  disableMouse,
  readMouseEvent,
  readMouseEventTimeout,
  getMousePosition,
  isMouseSupported,
  waitForClickAt,
  waitForClickInRegion,
  MouseEventListener,
  MouseButtonEnum,
  MouseEventType,
  type MouseEventInfo,
} from "../index.js";

// ============================================
// Mouse Support Tests
// ============================================
describe("Mouse Support Tests", () => {
  test("isMouseSupported should return a boolean", () => {
    const result = isMouseSupported();
    expect(typeof result).toBe("boolean");
  });

  test("enableMouse should not throw", () => {
    expect(() => enableMouse()).not.toThrow();
  });

  test("disableMouse should not throw", () => {
    expect(() => disableMouse()).not.toThrow();
  });
});

// ============================================
// Mouse Event Listener Tests
// ============================================
describe("MouseEventListener Tests", () => {
  test("MouseEventListener should be constructable", () => {
    const listener = new MouseEventListener();
    expect(listener).toBeDefined();
  });

  test("MouseEventListener should have listen method", () => {
    const listener = new MouseEventListener();
    expect(typeof listener.listen).toBe("function");
  });

  test("MouseEventListener should have stop method", () => {
    const listener = new MouseEventListener();
    expect(typeof listener.stop).toBe("function");
  });

  test("MouseEventListener should have isRunning method", () => {
    const listener = new MouseEventListener();
    expect(typeof listener.isRunning).toBe("function");
  });

  test("MouseEventListener isRunning should return false initially", async () => {
    const listener = new MouseEventListener();
    const running = await listener.isRunning();
    expect(running).toBe(false);
  });
});

// ============================================
// Mouse Event Reading Tests
// ============================================
describe("Mouse Event Reading Tests", () => {
  test("readMouseEvent should be a function", () => {
    expect(typeof readMouseEvent).toBe("function");
  });

  test("readMouseEventTimeout should be a function", () => {
    expect(typeof readMouseEventTimeout).toBe("function");
  });

  test("getMousePosition should be a function", () => {
    expect(typeof getMousePosition).toBe("function");
  });

  test("getMousePosition should return a tuple", () => {
    // This may throw in non-TTY environments
    try {
      const position = getMousePosition();
      expect(Array.isArray(position)).toBe(true);
      expect(position.length).toBe(2);
    } catch {
      // Expected in non-TTY
    }
  });
});

// ============================================
// Mouse Click Wait Tests
// ============================================
describe("Mouse Click Wait Tests", () => {
  test("waitForClickAt should be a function", () => {
    expect(typeof waitForClickAt).toBe("function");
  });

  test("waitForClickInRegion should be a function", () => {
    expect(typeof waitForClickInRegion).toBe("function");
  });
});

// ============================================
// MouseButtonEnum Tests
// ============================================
describe("MouseButtonEnum Tests", () => {
  test("MouseButtonEnum should have correct values", () => {
    expect(MouseButtonEnum.Left).toBe(0);
    expect(MouseButtonEnum.Right).toBe(1);
    expect(MouseButtonEnum.Middle).toBe(2);
    expect(MouseButtonEnum.None).toBe(3);
  });
});

// ============================================
// MouseEventType Tests
// ============================================
describe("MouseEventType Tests", () => {
  test("MouseEventType should have correct values", () => {
    expect(MouseEventType.Down).toBe(0);
    expect(MouseEventType.Up).toBe(1);
    expect(MouseEventType.Click).toBe(2);
    expect(MouseEventType.Drag).toBe(3);
    expect(MouseEventType.Move).toBe(4);
    expect(MouseEventType.ScrollUp).toBe(5);
    expect(MouseEventType.ScrollDown).toBe(6);
  });
});

// ============================================
// MouseEventInfo Type Tests
// ============================================
describe("MouseEventInfo Type Tests", () => {
  test("MouseEventInfo type should have expected properties", () => {
    const event: MouseEventInfo = {
      eventType: "click",
      button: "left",
      column: 10,
      row: 5,
      ctrl: false,
      alt: false,
      shift: false,
    };
    expect(event.eventType).toBe("click");
    expect(event.button).toBe("left");
    expect(event.column).toBe(10);
    expect(event.row).toBe(5);
    expect(event.ctrl).toBe(false);
    expect(event.alt).toBe(false);
    expect(event.shift).toBe(false);
  });

  test("MouseEventInfo should support all event types", () => {
    const eventTypes = ["down", "up", "click", "drag", "move", "scrollUp", "scrollDown"];
    for (const type of eventTypes) {
      const event: MouseEventInfo = {
        eventType: type,
        button: "left",
        column: 0,
        row: 0,
        ctrl: false,
        alt: false,
        shift: false,
      };
      expect(event.eventType).toBe(type);
    }
  });

  test("MouseEventInfo should support all button types", () => {
    const buttons = ["left", "right", "middle", "none"];
    for (const button of buttons) {
      const event: MouseEventInfo = {
        eventType: "click",
        button: button,
        column: 0,
        row: 0,
        ctrl: false,
        alt: false,
        shift: false,
      };
      expect(event.button).toBe(button);
    }
  });

  test("MouseEventInfo should support modifier combinations", () => {
    const event: MouseEventInfo = {
      eventType: "click",
      button: "left",
      column: 10,
      row: 5,
      ctrl: true,
      alt: true,
      shift: true,
    };
    expect(event.ctrl).toBe(true);
    expect(event.alt).toBe(true);
    expect(event.shift).toBe(true);
  });
});

// ============================================
// Mouse Integration Tests
// ============================================
describe("Mouse Integration Tests", () => {
  test("enable and disable mouse should work together", () => {
    expect(() => {
      enableMouse();
      disableMouse();
    }).not.toThrow();
  });

  test("MouseEventListener can be created and stopped", async () => {
    const listener = new MouseEventListener();
    // Stop immediately without listening
    await listener.stop();
    const running = await listener.isRunning();
    expect(running).toBe(false);
  });
});
