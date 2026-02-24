import { expect, test, describe } from "bun:test";
import {
  AsyncLineIterator,
  BufferedReader,
  lineIterator,
} from "../index.js";

// ============================================
// AsyncLineIterator Tests
// ============================================
describe("AsyncLineIterator Tests", () => {
  test("AsyncLineIterator should be constructable", () => {
    const iter = new AsyncLineIterator();
    expect(iter).toBeDefined();
  });

  test("AsyncLineIterator should have addLine method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.addLine).toBe("function");
  });

  test("AsyncLineIterator should have addLines method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.addLines).toBe("function");
  });

  test("AsyncLineIterator should have next method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.next).toBe("function");
  });

  test("AsyncLineIterator should have isExhausted method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.isExhausted).toBe("function");
  });

  test("AsyncLineIterator should have collect method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.collect).toBe("function");
  });

  test("AsyncLineIterator should have take method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.take).toBe("function");
  });

  test("AsyncLineIterator should have skip method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.skip).toBe("function");
  });

  test("AsyncLineIterator should have filter method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.filter).toBe("function");
  });

  test("AsyncLineIterator should have reset method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.reset).toBe("function");
  });

  test("AsyncLineIterator should have len method", () => {
    const iter = new AsyncLineIterator();
    expect(typeof iter.len).toBe("function");
  });
});

// ============================================
// AsyncLineIterator Functional Tests
// ============================================
describe("AsyncLineIterator Functional Tests", () => {
  test("AsyncLineIterator should add and retrieve lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLine("Hello");
    const line = await iter.next();
    expect(line).toBe("Hello");
  });

  test("AsyncLineIterator should add multiple lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["Line1", "Line2", "Line3"]);
    const lines = await iter.collect();
    expect(lines).toHaveLength(3);
    expect(lines[0]).toBe("Line1");
    expect(lines[1]).toBe("Line2");
    expect(lines[2]).toBe("Line3");
  });

  test("AsyncLineIterator isExhausted should return false initially", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLine("test");
    const exhausted = await iter.isExhausted();
    expect(exhausted).toBe(false);
  });

  test("AsyncLineIterator take should return n lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["A", "B", "C", "D", "E"]);
    const lines = await iter.take(3);
    expect(lines).toHaveLength(3);
    expect(lines).toEqual(["A", "B", "C"]);
  });

  test("AsyncLineIterator len should return count", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["A", "B", "C"]);
    const len = await iter.len();
    expect(len).toBe(3);
  });

  test("AsyncLineIterator reset should be callable", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["A", "B", "C"]);
    // Reset should be callable
    expect(async () => await iter.reset()).not.toThrow();
  });

  test("AsyncLineIterator filter should filter lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["apple", "banana", "apricot", "cherry"]);
    const filtered = await iter.filter("ap");
    expect(filtered.length).toBe(2);
    expect(filtered).toContain("apple");
    expect(filtered).toContain("apricot");
  });

  test("AsyncLineIterator skip should skip lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["A", "B", "C", "D"]);
    await iter.skip(2);
    const line = await iter.next();
    expect(line).toBe("C");
  });
});

// ============================================
// BufferedReader Tests
// ============================================
describe("BufferedReader Tests", () => {
  test("BufferedReader should be constructable", () => {
    const reader = new BufferedReader();
    expect(reader).toBeDefined();
  });

  test("BufferedReader should have readLine method", () => {
    const reader = new BufferedReader();
    expect(typeof reader.readLine).toBe("function");
  });

  test("BufferedReader should have readUntil method", () => {
    const reader = new BufferedReader();
    expect(typeof reader.readUntil).toBe("function");
  });

  test("BufferedReader should have next method", () => {
    const reader = new BufferedReader();
    expect(typeof reader.next).toBe("function");
  });

  test("BufferedReader should have read method", () => {
    const reader = new BufferedReader();
    expect(typeof reader.read).toBe("function");
  });
});

// ============================================
// lineIterator Function Tests
// ============================================
describe("lineIterator Function Tests", () => {
  test("lineIterator should create an AsyncLineIterator", () => {
    const iter = lineIterator();
    expect(iter).toBeDefined();
    expect(typeof iter.addLine).toBe("function");
    expect(typeof iter.next).toBe("function");
  });

  test("lineIterator should return a working iterator", async () => {
    const iter = lineIterator();
    await iter.addLine("test line");
    const line = await iter.next();
    expect(line).toBe("test line");
  });
});

// ============================================
// AsyncLineIterator Edge Cases Tests
// ============================================
describe("AsyncLineIterator Edge Cases Tests", () => {
  test("AsyncLineIterator should handle empty lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLine("");
    const line = await iter.next();
    expect(line).toBe("");
  });

  test("AsyncLineIterator should handle unicode lines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLine("Hello  World");
    const line = await iter.next();
    expect(line).toBe("Hello  World");
  });

  test("AsyncLineIterator should handle special characters", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLine("Line\twith\ttabs\nand newline");
    const line = await iter.next();
    expect(line).toContain("\t");
  });

  test("AsyncLineIterator should handle large number of lines", async () => {
    const iter = new AsyncLineIterator();
    const lines = Array.from({ length: 1000 }, (_, i) => `Line ${i}`);
    await iter.addLines(lines);
    const len = await iter.len();
    expect(len).toBe(1000);
  });

  test("AsyncLineIterator should handle take with more than available", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["A", "B"]);
    const lines = await iter.take(5);
    expect(lines).toHaveLength(2);
  });

  test("AsyncLineIterator should handle filter with no matches", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["apple", "banana", "cherry"]);
    const filtered = await iter.filter("xyz");
    expect(filtered).toHaveLength(0);
  });

  test("AsyncLineIterator should handle filter with maxLines", async () => {
    const iter = new AsyncLineIterator();
    await iter.addLines(["ap1", "ap2", "ap3", "ap4"]);
    const filtered = await iter.filter("ap", 2);
    expect(filtered).toHaveLength(2);
  });
});
