import { expect, test, describe } from "bun:test";
import { getVersion } from "../index.js";

// ============================================
// Version Tests
// ============================================
describe("Version Tests", () => {
  test("getVersion should return a valid semver string", () => {
    const version = getVersion();
    expect(typeof version).toBe("string");
    expect(version).toMatch(/^\d+\.\d+\.\d+/);
  });

  test("getVersion should return non-empty string", () => {
    const version = getVersion();
    expect(version.length).toBeGreaterThan(0);
  });

  test("getVersion should be consistent across calls", () => {
    const version1 = getVersion();
    const version2 = getVersion();
    expect(version1).toBe(version2);
  });
});
