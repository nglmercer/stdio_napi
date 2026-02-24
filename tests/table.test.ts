import { expect, test, describe } from "bun:test";
import {
  createTable,
  renderTable,
  printTable,
  renderCsvTable,
  renderKeyValueTable,
  BorderStyle,
  ColumnAlign,
  type TableConfig,
  type TableColumn,
  type TableRow,
} from "../index.js";

// ============================================
// Border Style Tests
// ============================================
describe("BorderStyle Enum Tests", () => {
  test("BorderStyle should have correct values", () => {
    expect(BorderStyle.None).toBe(0);
    expect(BorderStyle.Single).toBe(1);
    expect(BorderStyle.Double).toBe(2);
    expect(BorderStyle.Rounded).toBe(3);
    expect(BorderStyle.Markdown).toBe(4);
    expect(BorderStyle.Ascii).toBe(5);
  });
});

// ============================================
// Column Align Tests
// ============================================
describe("ColumnAlign Enum Tests", () => {
  test("ColumnAlign should have correct values", () => {
    expect(ColumnAlign.Left).toBe(0);
    expect(ColumnAlign.Center).toBe(1);
    expect(ColumnAlign.Right).toBe(2);
  });
});

// ============================================
// Table Column Type Tests
// ============================================
describe("TableColumn Type Tests", () => {
  test("TableColumn should support basic config", () => {
    const column: TableColumn = {
      header: "Name",
    };
    expect(column.header).toBe("Name");
  });

  test("TableColumn should support width", () => {
    const column: TableColumn = {
      header: "Name",
      width: 20,
    };
    expect(column.width).toBe(20);
  });

  test("TableColumn should support alignment", () => {
    const column: TableColumn = {
      header: "Value",
      align: "right",
    };
    expect(column.align).toBe("right");
  });

  test("TableColumn should support wrap option", () => {
    const column: TableColumn = {
      header: "Description",
      wrap: true,
    };
    expect(column.wrap).toBe(true);
  });

  test("TableColumn should support colors", () => {
    const column: TableColumn = {
      header: "Status",
      headerColor: "green",
      cellColor: "white",
    };
    expect(column.headerColor).toBe("green");
    expect(column.cellColor).toBe("white");
  });
});

// ============================================
// Table Row Type Tests
// ============================================
describe("TableRow Type Tests", () => {
  test("TableRow should support cells", () => {
    const row: TableRow = {
      cells: ["Alice", "30", "NYC"],
    };
    expect(row.cells).toHaveLength(3);
    expect(row.cells[0]).toBe("Alice");
  });

  test("TableRow should support color", () => {
    const row: TableRow = {
      cells: ["Error", "Failed"],
      color: "red",
    };
    expect(row.color).toBe("red");
  });

  test("TableRow should support isHeader flag", () => {
    const row: TableRow = {
      cells: ["Name", "Age", "City"],
      isHeader: true,
    };
    expect(row.isHeader).toBe(true);
  });
});

// ============================================
// Table Config Type Tests
// ============================================
describe("TableConfig Type Tests", () => {
  test("TableConfig should support columns", () => {
    const config: TableConfig = {
      columns: [
        { header: "Name" },
        { header: "Age" },
      ],
    };
    expect(config.columns).toHaveLength(2);
  });

  test("TableConfig should support border style", () => {
    const config: TableConfig = {
      columns: [{ header: "Test" }],
      borderStyle: "rounded",
    };
    expect(config.borderStyle).toBe("rounded");
  });

  test("TableConfig should support showHeader", () => {
    const config: TableConfig = {
      columns: [{ header: "Test" }],
      showHeader: true,
    };
    expect(config.showHeader).toBe(true);
  });

  test("TableConfig should support padding", () => {
    const config: TableConfig = {
      columns: [{ header: "Test" }],
      padding: 2,
    };
    expect(config.padding).toBe(2);
  });

  test("TableConfig should support maxWidth", () => {
    const config: TableConfig = {
      columns: [{ header: "Test" }],
      maxWidth: 80,
    };
    expect(config.maxWidth).toBe(80);
  });

  test("TableConfig should support compact mode", () => {
    const config: TableConfig = {
      columns: [{ header: "Test" }],
      compact: true,
    };
    expect(config.compact).toBe(true);
  });
});

// ============================================
// Create Table Tests
// ============================================
describe("Create Table Tests", () => {
  test("createTable should create a simple table", () => {
    const table = createTable(
      ["Name", "Age"],
      [["Alice", "30"], ["Bob", "25"]]
    );
    expect(typeof table).toBe("string");
    expect(table.length).toBeGreaterThan(0);
  });

  test("createTable should handle empty rows", () => {
    const table = createTable(["Name", "Age"], []);
    expect(typeof table).toBe("string");
  });

  test("createTable should handle single row", () => {
    const table = createTable(
      ["Name", "Age"],
      [["Alice", "30"]]
    );
    expect(typeof table).toBe("string");
  });

  test("createTable should handle border style", () => {
    const table = createTable(
      ["Name", "Age"],
      [["Alice", "30"]],
      "rounded"
    );
    expect(typeof table).toBe("string");
  });

  test("createTable should handle different border styles", () => {
    const styles = ["none", "single", "double", "rounded", "markdown", "ascii"];
    for (const style of styles) {
      const table = createTable(
        ["Name"],
        [["Test"]],
        style
      );
      expect(typeof table).toBe("string");
    }
  });

  test("createTable should handle unicode content", () => {
    const table = createTable(
      ["Name", "City"],
      [["Alice", "Tokyo "], ["Bob", "São Paulo"]]
    );
    expect(typeof table).toBe("string");
  });
});

// ============================================
// Render Table Tests
// ============================================
describe("Render Table Tests", () => {
  test("renderTable should render a table", () => {
    const table = renderTable(
      [{ cells: ["Name", "Age"], isHeader: true }, { cells: ["Alice", "30"] }],
      { columns: [{ header: "Name" }, { header: "Age" }] }
    );
    expect(typeof table).toBe("string");
  });

  test("renderTable should handle empty rows", () => {
    const table = renderTable([], { columns: [] });
    expect(typeof table).toBe("string");
  });

  test("renderTable should handle complex config", () => {
    const table = renderTable(
      [
        { cells: ["Name", "Age", "City"], isHeader: true },
        { cells: ["Alice", "30", "NYC"] },
        { cells: ["Bob", "25", "LA"] },
      ],
      {
        columns: [
          { header: "Name", align: "left" },
          { header: "Age", align: "center" },
          { header: "City", align: "right" },
        ],
        borderStyle: "rounded",
        showHeader: true,
        padding: 1,
      }
    );
    expect(typeof table).toBe("string");
  });
});

// ============================================
// Print Table Tests
// ============================================
describe("Print Table Tests", () => {
  test("printTable should not throw", () => {
    expect(() =>
      printTable(
        [{ cells: ["Name", "Age"] }, { cells: ["Alice", "30"] }],
        { columns: [{ header: "Name" }, { header: "Age" }] }
      )
    ).not.toThrow();
  });

  test("printTable should handle empty rows", () => {
    expect(() => printTable([], { columns: [] })).not.toThrow();
  });
});

// ============================================
// CSV Table Tests
// ============================================
describe("CSV Table Tests", () => {
  test("renderCsvTable should render CSV data", () => {
    const csv = "Name,Age,City\nAlice,30,NYC\nBob,25,LA";
    const table = renderCsvTable(csv, ",", true);
    expect(typeof table).toBe("string");
  });

  test("renderCsvTable should handle different delimiters", () => {
    const csv = "Name;Age;City\nAlice;30;NYC";
    const table = renderCsvTable(csv, ";", true);
    expect(typeof table).toBe("string");
  });

  test("renderCsvTable should handle no header", () => {
    const csv = "Alice,30,NYC\nBob,25,LA";
    const table = renderCsvTable(csv, ",", false);
    expect(typeof table).toBe("string");
  });

  test("renderCsvTable should handle empty CSV", () => {
    const table = renderCsvTable("", ",", true);
    expect(typeof table).toBe("string");
  });
});

// ============================================
// Key Value Table Tests
// ============================================
describe("Key Value Table Tests", () => {
  test("renderKeyValueTable should render key-value pairs", () => {
    const table = renderKeyValueTable({
      Name: "Alice",
      Age: "30",
      City: "NYC",
    });
    expect(typeof table).toBe("string");
  });

  test("renderKeyValueTable should handle empty object", () => {
    const table = renderKeyValueTable({});
    expect(typeof table).toBe("string");
  });

  test("renderKeyValueTable should handle border style", () => {
    const table = renderKeyValueTable(
      { Name: "Alice", Age: "30" },
      "rounded"
    );
    expect(typeof table).toBe("string");
  });

  test("renderKeyValueTable should handle unicode values", () => {
    const table = renderKeyValueTable({
      Name: "José",
      City: "São Paulo",
      Emoji: "",
    });
    expect(typeof table).toBe("string");
  });
});

// ============================================
// Table Edge Cases Tests
// ============================================
describe("Table Edge Cases Tests", () => {
  test("createTable should handle many columns", () => {
    const headers = Array.from({ length: 20 }, (_, i) => `Col${i}`);
    const rows = [Array.from({ length: 20 }, (_, i) => `Val${i}`)];
    const table = createTable(headers, rows);
    expect(typeof table).toBe("string");
  });

  test("createTable should handle many rows", () => {
    const headers = ["Name", "Age"];
    const rows = Array.from({ length: 100 }, (_, i) => [`User${i}`, `${i + 20}`]);
    const table = createTable(headers, rows);
    expect(typeof table).toBe("string");
  });

  test("createTable should handle long cell content", () => {
    const longText = "a".repeat(100);
    const table = createTable(["Content"], [[longText]]);
    expect(typeof table).toBe("string");
  });

  test("createTable should handle special characters", () => {
    const table = createTable(
      ["Special"],
      [["<>&\"'"]]
    );
    expect(typeof table).toBe("string");
  });
});
