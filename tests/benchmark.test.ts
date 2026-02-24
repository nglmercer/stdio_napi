import { expect, test, describe } from "bun:test";
import {
  benchmarkPrint,
  benchmarkProgressBar,
  benchmarkShellEscape,
  benchmarkSpinner,
  benchmarkStringAllocation,
  benchmarkStringFormat,
  benchmarkTableRender,
  benchmarkTerminalSize,
  benchmarkVectorOperations,
  benchmarkHashmapOperations,
  benchmarkKeyParsing,
  runBenchmarkSuite,
  type BenchmarkResult,
  type BenchmarkSuiteResult,
} from "../index.js";

// ============================================
// Benchmark Result Type Tests
// ============================================
describe("BenchmarkResult Type Tests", () => {
  test("BenchmarkResult should have expected properties", () => {
    const result: BenchmarkResult = {
      name: "test",
      iterations: 1000,
      totalTimeMs: 100,
      avgTimeUs: 100,
      minTimeUs: 50,
      maxTimeUs: 200,
      opsPerSecond: 10000,
    };
    expect(result.name).toBe("test");
    expect(result.iterations).toBe(1000);
    expect(result.totalTimeMs).toBe(100);
    expect(result.avgTimeUs).toBe(100);
    expect(result.minTimeUs).toBe(50);
    expect(result.maxTimeUs).toBe(200);
    expect(result.opsPerSecond).toBe(10000);
  });
});

// ============================================
// Benchmark Suite Result Type Tests
// ============================================
describe("BenchmarkSuiteResult Type Tests", () => {
  test("BenchmarkSuiteResult should have expected properties", () => {
    const result: BenchmarkSuiteResult = {
      name: "suite",
      results: [],
      totalTimeMs: 1000,
    };
    expect(result.name).toBe("suite");
    expect(Array.isArray(result.results)).toBe(true);
    expect(result.totalTimeMs).toBe(1000);
  });
});

// ============================================
// Benchmark Print Tests
// ============================================
describe("benchmarkPrint Tests", () => {
  test("benchmarkPrint should return BenchmarkResult", () => {
    const result = benchmarkPrint(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(typeof result.iterations).toBe("number");
    expect(typeof result.totalTimeMs).toBe("number");
    expect(typeof result.avgTimeUs).toBe("number");
    expect(typeof result.minTimeUs).toBe("number");
    expect(typeof result.maxTimeUs).toBe("number");
    expect(typeof result.opsPerSecond).toBe("number");
  });

  test("benchmarkPrint should handle default iterations", () => {
    const result = benchmarkPrint();
    expect(result).toBeDefined();
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkPrint should handle custom iterations", () => {
    const result = benchmarkPrint(50);
    expect(result.iterations).toBe(50);
  });
});

// ============================================
// Benchmark Progress Bar Tests
// ============================================
describe("benchmarkProgressBar Tests", () => {
  test("benchmarkProgressBar should return BenchmarkResult", () => {
    const result = benchmarkProgressBar(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkProgressBar should handle default iterations", () => {
    const result = benchmarkProgressBar();
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Shell Escape Tests
// ============================================
describe("benchmarkShellEscape Tests", () => {
  test("benchmarkShellEscape should return BenchmarkResult", () => {
    const result = benchmarkShellEscape(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkShellEscape should handle default iterations", () => {
    const result = benchmarkShellEscape();
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Spinner Tests
// ============================================
describe("benchmarkSpinner Tests", () => {
  test("benchmarkSpinner should return BenchmarkResult", () => {
    const result = benchmarkSpinner(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkSpinner should handle default iterations", () => {
    const result = benchmarkSpinner();
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark String Allocation Tests
// ============================================
describe("benchmarkStringAllocation Tests", () => {
  test("benchmarkStringAllocation should return BenchmarkResult", () => {
    const result = benchmarkStringAllocation(100, 50);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkStringAllocation should handle default parameters", () => {
    const result = benchmarkStringAllocation();
    expect(result).toBeDefined();
  });

  test("benchmarkStringAllocation should handle custom string size", () => {
    const result = benchmarkStringAllocation(100, 1000);
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark String Format Tests
// ============================================
describe("benchmarkStringFormat Tests", () => {
  test("benchmarkStringFormat should return BenchmarkResult", () => {
    const result = benchmarkStringFormat(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkStringFormat should handle default iterations", () => {
    const result = benchmarkStringFormat();
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Table Render Tests
// ============================================
describe("benchmarkTableRender Tests", () => {
  test("benchmarkTableRender should return BenchmarkResult", () => {
    const result = benchmarkTableRender(10, 5, 3);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkTableRender should handle default parameters", () => {
    const result = benchmarkTableRender();
    expect(result).toBeDefined();
  });

  test("benchmarkTableRender should handle large tables", () => {
    const result = benchmarkTableRender(10, 100, 10);
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Terminal Size Tests
// ============================================
describe("benchmarkTerminalSize Tests", () => {
  test("benchmarkTerminalSize should return BenchmarkResult", () => {
    const result = benchmarkTerminalSize(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkTerminalSize should handle default iterations", () => {
    const result = benchmarkTerminalSize();
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Vector Operations Tests
// ============================================
describe("benchmarkVectorOperations Tests", () => {
  test("benchmarkVectorOperations should return BenchmarkResult", () => {
    const result = benchmarkVectorOperations(100, 50);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkVectorOperations should handle default parameters", () => {
    const result = benchmarkVectorOperations();
    expect(result).toBeDefined();
  });

  test("benchmarkVectorOperations should handle custom vector size", () => {
    const result = benchmarkVectorOperations(100, 1000);
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Hashmap Operations Tests
// ============================================
describe("benchmarkHashmapOperations Tests", () => {
  test("benchmarkHashmapOperations should return BenchmarkResult", () => {
    const result = benchmarkHashmapOperations(100, 50);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkHashmapOperations should handle default parameters", () => {
    const result = benchmarkHashmapOperations();
    expect(result).toBeDefined();
  });

  test("benchmarkHashmapOperations should handle custom map size", () => {
    const result = benchmarkHashmapOperations(100, 1000);
    expect(result).toBeDefined();
  });
});

// ============================================
// Benchmark Key Parsing Tests
// ============================================
describe("benchmarkKeyParsing Tests", () => {
  test("benchmarkKeyParsing should return BenchmarkResult", () => {
    const result = benchmarkKeyParsing(100);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(result.iterations).toBeGreaterThan(0);
  });

  test("benchmarkKeyParsing should handle default iterations", () => {
    const result = benchmarkKeyParsing();
    expect(result).toBeDefined();
  });
});

// ============================================
// Run Benchmark Suite Tests
// ============================================
describe("runBenchmarkSuite Tests", () => {
  test("runBenchmarkSuite should return BenchmarkSuiteResult", () => {
    const result = runBenchmarkSuite(10);
    expect(result).toBeDefined();
    expect(typeof result.name).toBe("string");
    expect(Array.isArray(result.results)).toBe(true);
    expect(typeof result.totalTimeMs).toBe("number");
  });

  test("runBenchmarkSuite should handle default iterations", () => {
    const result = runBenchmarkSuite();
    expect(result).toBeDefined();
    expect(result.results.length).toBeGreaterThan(0);
  });

  test("runBenchmarkSuite should return multiple benchmark results", () => {
    const result = runBenchmarkSuite(10);
    expect(result.results.length).toBeGreaterThan(1);
    for (const benchmark of result.results) {
      expect(typeof benchmark.name).toBe("string");
      expect(typeof benchmark.iterations).toBe("number");
      expect(typeof benchmark.avgTimeUs).toBe("number");
    }
  });
});

// ============================================
// Benchmark Consistency Tests
// ============================================
describe("Benchmark Consistency Tests", () => {
  test("multiple benchmark runs should return consistent structure", () => {
    const result1 = benchmarkPrint(50);
    const result2 = benchmarkPrint(50);
    expect(result1.name).toBe(result2.name);
    expect(result1.iterations).toBe(result2.iterations);
  });

  test("benchmark results should have reasonable values", () => {
    const result = benchmarkPrint(100);
    expect(result.avgTimeUs).toBeGreaterThanOrEqual(0);
    expect(result.minTimeUs).toBeLessThanOrEqual(result.maxTimeUs);
    expect(result.opsPerSecond).toBeGreaterThanOrEqual(0);
  });
});
