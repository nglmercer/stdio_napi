import { expect, test, describe } from "bun:test";
import {
  TerminalMultiplexer,
  createMultiplexer,
  isMultiplexingSupported,
  LayoutDirection,
  type Session,
  type SessionOptions,
  type Window,
  type WindowOptions,
  type Pane,
  type PaneOptions,
} from "../index.js";

// ============================================
// Layout Direction Tests
// ============================================
describe("LayoutDirection Enum Tests", () => {
  test("LayoutDirection should have correct values", () => {
    expect(LayoutDirection.Horizontal).toBe(0);
    expect(LayoutDirection.Vertical).toBe(1);
  });
});

// ============================================
// Multiplexing Support Tests
// ============================================
describe("Multiplexing Support Tests", () => {
  test("isMultiplexingSupported should return a boolean", () => {
    const result = isMultiplexingSupported();
    expect(typeof result).toBe("boolean");
  });
});

// ============================================
// Session Options Type Tests
// ============================================
describe("SessionOptions Type Tests", () => {
  test("SessionOptions should support required name", () => {
    const options: SessionOptions = {
      name: "test-session",
    };
    expect(options.name).toBe("test-session");
  });

  test("SessionOptions should support optional cwd", () => {
    const options: SessionOptions = {
      name: "test",
      cwd: "/home/user",
    };
    expect(options.cwd).toBe("/home/user");
  });

  test("SessionOptions should support windowName", () => {
    const options: SessionOptions = {
      name: "test",
      windowName: "main",
    };
    expect(options.windowName).toBe("main");
  });

  test("SessionOptions should support dimensions", () => {
    const options: SessionOptions = {
      name: "test",
      width: 120,
      height: 40,
    };
    expect(options.width).toBe(120);
    expect(options.height).toBe(40);
  });

  test("SessionOptions should support shell", () => {
    const options: SessionOptions = {
      name: "test",
      shell: "/bin/bash",
    };
    expect(options.shell).toBe("/bin/bash");
  });
});

// ============================================
// Window Options Type Tests
// ============================================
describe("WindowOptions Type Tests", () => {
  test("WindowOptions should support name", () => {
    const options: WindowOptions = {
      name: "editor",
    };
    expect(options.name).toBe("editor");
  });

  test("WindowOptions should support cwd", () => {
    const options: WindowOptions = {
      cwd: "/project",
    };
    expect(options.cwd).toBe("/project");
  });

  test("WindowOptions should support command", () => {
    const options: WindowOptions = {
      command: "vim",
    };
    expect(options.command).toBe("vim");
  });

  test("WindowOptions should support dimensions", () => {
    const options: WindowOptions = {
      width: 80,
      height: 24,
    };
    expect(options.width).toBe(80);
    expect(options.height).toBe(24);
  });
});

// ============================================
// Pane Options Type Tests
// ============================================
describe("PaneOptions Type Tests", () => {
  test("PaneOptions should support cwd", () => {
    const options: PaneOptions = {
      cwd: "/home",
    };
    expect(options.cwd).toBe("/home");
  });

  test("PaneOptions should support command", () => {
    const options: PaneOptions = {
      command: "htop",
    };
    expect(options.command).toBe("htop");
  });

  test("PaneOptions should support split direction", () => {
    const options: PaneOptions = {
      split: "vertical",
    };
    expect(options.split).toBe("vertical");
  });

  test("PaneOptions should support percentage", () => {
    const options: PaneOptions = {
      percentage: 30,
    };
    expect(options.percentage).toBe(30);
  });
});

// ============================================
// Session Type Tests
// ============================================
describe("Session Type Tests", () => {
  test("Session should have expected properties", () => {
    const session: Session = {
      id: "session-1",
      name: "main",
      createdAt: Date.now(),
      lastActivity: Date.now(),
      windows: ["window-1"],
      width: 120,
      height: 40,
      attached: true,
    };
    expect(session.id).toBe("session-1");
    expect(session.name).toBe("main");
    expect(session.windows).toHaveLength(1);
    expect(session.attached).toBe(true);
  });

  test("Session should support optional activeWindow", () => {
    const session: Session = {
      id: "session-1",
      name: "main",
      createdAt: Date.now(),
      lastActivity: Date.now(),
      windows: ["window-1"],
      width: 120,
      height: 40,
      attached: true,
      activeWindow: "window-1",
    };
    expect(session.activeWindow).toBe("window-1");
  });
});

// ============================================
// Window Type Tests
// ============================================
describe("Window Type Tests", () => {
  test("Window should have expected properties", () => {
    const window: Window = {
      id: "window-1",
      sessionId: "session-1",
      index: 0,
      width: 120,
      height: 40,
      panes: ["pane-1"],
    };
    expect(window.id).toBe("window-1");
    expect(window.sessionId).toBe("session-1");
    expect(window.index).toBe(0);
    expect(window.panes).toHaveLength(1);
  });

  test("Window should support optional properties", () => {
    const window: Window = {
      id: "window-1",
      sessionId: "session-1",
      index: 0,
      name: "editor",
      width: 120,
      height: 40,
      panes: ["pane-1"],
      activePane: "pane-1",
    };
    expect(window.name).toBe("editor");
    expect(window.activePane).toBe("pane-1");
  });
});

// ============================================
// Pane Type Tests
// ============================================
describe("Pane Type Tests", () => {
  test("Pane should have expected properties", () => {
    const pane: Pane = {
      id: "pane-1",
      windowId: "window-1",
      index: 0,
      active: true,
      width: 60,
      height: 40,
      x: 0,
      y: 0,
    };
    expect(pane.id).toBe("pane-1");
    expect(pane.windowId).toBe("window-1");
    expect(pane.active).toBe(true);
    expect(pane.width).toBe(60);
    expect(pane.height).toBe(40);
  });

  test("Pane should support optional properties", () => {
    const pane: Pane = {
      id: "pane-1",
      windowId: "window-1",
      index: 0,
      cwd: "/home/user",
      title: "bash",
      active: true,
      width: 60,
      height: 40,
      x: 0,
      y: 0,
      pid: 12345,
    };
    expect(pane.cwd).toBe("/home/user");
    expect(pane.title).toBe("bash");
    expect(pane.pid).toBe(12345);
  });
});

// ============================================
// TerminalMultiplexer Tests
// ============================================
describe("TerminalMultiplexer Tests", () => {
  test("TerminalMultiplexer should be constructable", () => {
    const mux = new TerminalMultiplexer();
    expect(mux).toBeDefined();
  });

  test("createMultiplexer should create a TerminalMultiplexer", () => {
    const mux = createMultiplexer();
    expect(mux).toBeDefined();
    expect(mux instanceof TerminalMultiplexer).toBe(true);
  });

  test("TerminalMultiplexer should have session methods", () => {
    const mux = new TerminalMultiplexer();
    expect(typeof mux.createSession).toBe("function");
    expect(typeof mux.listSessions).toBe("function");
    expect(typeof mux.getSession).toBe("function");
    expect(typeof mux.attachSession).toBe("function");
    expect(typeof mux.detachSession).toBe("function");
    expect(typeof mux.killSession).toBe("function");
  });

  test("TerminalMultiplexer should have window methods", () => {
    const mux = new TerminalMultiplexer();
    expect(typeof mux.createWindow).toBe("function");
    expect(typeof mux.listWindows).toBe("function");
    expect(typeof mux.getWindow).toBe("function");
    expect(typeof mux.killWindow).toBe("function");
    expect(typeof mux.selectWindow).toBe("function");
  });

  test("TerminalMultiplexer should have pane methods", () => {
    const mux = new TerminalMultiplexer();
    expect(typeof mux.splitPane).toBe("function");
    expect(typeof mux.listPanes).toBe("function");
    expect(typeof mux.getPane).toBe("function");
    expect(typeof mux.killPane).toBe("function");
    expect(typeof mux.selectPane).toBe("function");
    expect(typeof mux.resizePane).toBe("function");
    expect(typeof mux.sendKeys).toBe("function");
  });

  test("TerminalMultiplexer should have utility methods", () => {
    const mux = new TerminalMultiplexer();
    expect(typeof mux.getActiveSession).toBe("function");
    expect(typeof mux.setActiveSession).toBe("function");
    expect(typeof mux.renameSession).toBe("function");
    expect(typeof mux.renameWindow).toBe("function");
    expect(typeof mux.sessionCount).toBe("function");
    expect(typeof mux.windowCount).toBe("function");
    expect(typeof mux.paneCount).toBe("function");
  });
});

// ============================================
// TerminalMultiplexer Functional Tests
// ============================================
describe("TerminalMultiplexer Functional Tests", () => {
  test("listSessions should return an array", async () => {
    const mux = new TerminalMultiplexer();
    const sessions = await mux.listSessions();
    expect(Array.isArray(sessions)).toBe(true);
  });

  test("sessionCount should return a number", async () => {
    const mux = new TerminalMultiplexer();
    const count = await mux.sessionCount();
    expect(typeof count).toBe("number");
    expect(count).toBeGreaterThanOrEqual(0);
  });

  test("getSession should return null for non-existent session", async () => {
    const mux = new TerminalMultiplexer();
    const session = await mux.getSession("non-existent-id");
    expect(session).toBeNull();
  });

  test("getActiveSession should return session or null", async () => {
    const mux = new TerminalMultiplexer();
    const session = await mux.getActiveSession();
    expect(session === null || typeof session === "object").toBe(true);
  });
});
