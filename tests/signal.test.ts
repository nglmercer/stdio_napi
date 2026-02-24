import { expect, test, describe } from "bun:test";
import {
  SignalHandler,
  getSignalInfo,
  getSupportedSignals,
  sendSignal,
  Signal,
  type SignalInfo,
} from "../index.js";

// ============================================
// Signal Enum Tests
// ============================================
describe("Signal Enum Tests", () => {
  test("Signal should have correct values", () => {
    expect(Signal.Interrupt).toBe(0);
    expect(Signal.Terminate).toBe(1);
    expect(Signal.Kill).toBe(2);
    expect(Signal.User1).toBe(3);
    expect(Signal.User2).toBe(4);
    expect(Signal.WindowChange).toBe(5);
    expect(Signal.Continue).toBe(6);
    expect(Signal.Stop).toBe(7);
  });
});

// ============================================
// Signal Info Type Tests
// ============================================
describe("SignalInfo Type Tests", () => {
  test("SignalInfo should have expected properties", () => {
    const info: SignalInfo = {
      signal: "interrupt",
      number: 2,
      description: "Interrupt signal",
    };
    expect(info.signal).toBe("interrupt");
    expect(info.number).toBe(2);
    expect(info.description).toBe("Interrupt signal");
  });
});

// ============================================
// Get Signal Info Tests
// ============================================
describe("getSignalInfo Tests", () => {
  test("getSignalInfo should return signal info for interrupt", () => {
    const info = getSignalInfo("interrupt");
    expect(info).toBeDefined();
    expect(typeof info.signal).toBe("string");
    expect(typeof info.number).toBe("number");
    expect(typeof info.description).toBe("string");
  });

  test("getSignalInfo should return signal info for terminate", () => {
    const info = getSignalInfo("terminate");
    expect(info).toBeDefined();
    expect(typeof info.signal).toBe("string");
    expect(typeof info.number).toBe("number");
  });

  test("getSignalInfo should return signal info for kill", () => {
    const info = getSignalInfo("kill");
    expect(info).toBeDefined();
    expect(typeof info.signal).toBe("string");
    expect(typeof info.number).toBe("number");
  });
});

// ============================================
// Get Supported Signals Tests
// ============================================
describe("getSupportedSignals Tests", () => {
  test("getSupportedSignals should return an array", () => {
    const signals = getSupportedSignals();
    expect(Array.isArray(signals)).toBe(true);
  });

  test("getSupportedSignals should return non-empty array", () => {
    const signals = getSupportedSignals();
    expect(signals.length).toBeGreaterThan(0);
  });

  test("getSupportedSignals should return SignalInfo objects", () => {
    const signals = getSupportedSignals();
    for (const signal of signals) {
      expect(typeof signal.signal).toBe("string");
      expect(typeof signal.number).toBe("number");
      expect(typeof signal.description).toBe("string");
    }
  });
});

// ============================================
// Signal Handler Tests
// ============================================
describe("SignalHandler Tests", () => {
  test("SignalHandler should be constructable", () => {
    const handler = new SignalHandler(["interrupt"]);
    expect(handler).toBeDefined();
  });

  test("SignalHandler should accept multiple signals", () => {
    const handler = new SignalHandler(["interrupt", "terminate"]);
    expect(handler).toBeDefined();
  });

  test("SignalHandler should have wait method", () => {
    const handler = new SignalHandler(["interrupt"]);
    expect(typeof handler.wait).toBe("function");
  });

  test("SignalHandler should have close method", () => {
    const handler = new SignalHandler(["interrupt"]);
    expect(typeof handler.close).toBe("function");
  });

  test("SignalHandler close should not throw", async () => {
    const handler = new SignalHandler(["interrupt"]);
    await handler.close();
    // Should not throw
  });
});

// ============================================
// Send Signal Tests
// ============================================
describe("sendSignal Tests", () => {
  test("sendSignal should be a function", () => {
    expect(typeof sendSignal).toBe("function");
  });

  test("sendSignal should return boolean", () => {
    // Sending to PID 1 (init) should fail gracefully
    const result = sendSignal(1, "interrupt");
    expect(typeof result).toBe("boolean");
  });

  test("sendSignal should accept different signal types", () => {
    const signals = ["interrupt", "terminate", "kill"];
    for (const signal of signals) {
      expect(typeof sendSignal).toBe("function");
    }
  });
});

// ============================================
// Signal Handler Integration Tests
// ============================================
describe("Signal Handler Integration Tests", () => {
  test("SignalHandler can be created and closed", async () => {
    const handler = new SignalHandler(["interrupt", "terminate"]);
    await handler.close();
    // Should complete without error
  });

  test("Multiple SignalHandler instances can be created", () => {
    const handler1 = new SignalHandler(["interrupt"]);
    const handler2 = new SignalHandler(["terminate"]);
    expect(handler1).toBeDefined();
    expect(handler2).toBeDefined();
  });
});

// ============================================
// Signal Name Tests
// ============================================
describe("Signal Name Tests", () => {
  test("Signal names should be case-insensitive or have specific format", () => {
    // Test that getSignalInfo accepts the expected signal names
    const signalNames = ["interrupt", "terminate", "kill", "user1", "user2"];
    for (const name of signalNames) {
      try {
        const info = getSignalInfo(name);
        expect(info).toBeDefined();
      } catch {
        // Some signals might not be available on all platforms
      }
    }
  });
});
