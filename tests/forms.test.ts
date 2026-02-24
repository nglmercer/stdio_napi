import { expect, test, describe } from "bun:test";
import {
  confirm,
  prompt,
  selectMenu,
  collectForm,
  displayForm,
  formTextInput,
  formPasswordInput,
  formConfirmInput,
  formSelectInput,
  formMultiSelectInput,
  formNumberInput,
  FormFieldType,
  type FormFieldConfig,
  type FormFieldResult,
  type FormResult,
} from "../index.js";

// ============================================
// Form Field Type Tests
// ============================================
describe("FormFieldType Enum Tests", () => {
  test("FormFieldType should have correct values", () => {
    expect(FormFieldType.Text).toBe(0);
    expect(FormFieldType.Password).toBe(1);
    expect(FormFieldType.Confirm).toBe(2);
    expect(FormFieldType.Select).toBe(3);
    expect(FormFieldType.MultiSelect).toBe(4);
    expect(FormFieldType.Number).toBe(5);
  });
});

// ============================================
// Form Field Config Type Tests
// ============================================
describe("FormFieldConfig Type Tests", () => {
  test("FormFieldConfig should support text field", () => {
    const config: FormFieldConfig = {
      name: "username",
      fieldType: "text",
      required: true,
      minLength: 3,
      maxLength: 20,
    };
    expect(config.name).toBe("username");
    expect(config.fieldType).toBe("text");
    expect(config.required).toBe(true);
  });

  test("FormFieldConfig should support password field", () => {
    const config: FormFieldConfig = {
      name: "password",
      fieldType: "password",
      required: true,
      minLength: 8,
    };
    expect(config.fieldType).toBe("password");
    expect(config.minLength).toBe(8);
  });

  test("FormFieldConfig should support select field", () => {
    const config: FormFieldConfig = {
      name: "choice",
      fieldType: "select",
      options: ["Option 1", "Option 2", "Option 3"],
    };
    expect(config.options).toHaveLength(3);
  });

  test("FormFieldConfig should support default value", () => {
    const config: FormFieldConfig = {
      name: "name",
      fieldType: "text",
      default: "John Doe",
    };
    expect(config.default).toBe("John Doe");
  });

  test("FormFieldConfig should support placeholder", () => {
    const config: FormFieldConfig = {
      name: "email",
      fieldType: "text",
      placeholder: "Enter your email",
    };
    expect(config.placeholder).toBe("Enter your email");
  });

  test("FormFieldConfig should support validation pattern", () => {
    const config: FormFieldConfig = {
      name: "email",
      fieldType: "text",
      pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
    };
    expect(config.pattern).toBeDefined();
  });

  test("FormFieldConfig should support help text", () => {
    const config: FormFieldConfig = {
      name: "name",
      fieldType: "text",
      help: "Enter your full name",
    };
    expect(config.help).toBe("Enter your full name");
  });
});

// ============================================
// Form Field Result Type Tests
// ============================================
describe("FormFieldResult Type Tests", () => {
  test("FormFieldResult should have expected properties", () => {
    const result: FormFieldResult = {
      name: "username",
      value: "johndoe",
      valid: true,
    };
    expect(result.name).toBe("username");
    expect(result.value).toBe("johndoe");
    expect(result.valid).toBe(true);
  });

  test("FormFieldResult should support error message", () => {
    const result: FormFieldResult = {
      name: "email",
      value: "invalid",
      valid: false,
      error: "Invalid email format",
    };
    expect(result.valid).toBe(false);
    expect(result.error).toBe("Invalid email format");
  });
});

// ============================================
// Form Result Type Tests
// ============================================
describe("FormResult Type Tests", () => {
  test("FormResult should have expected properties", () => {
    const result: FormResult = {
      fields: [
        { name: "name", value: "John", valid: true },
        { name: "email", value: "john@example.com", valid: true },
      ],
      valid: true,
      fieldCount: 2,
    };
    expect(result.fields).toHaveLength(2);
    expect(result.valid).toBe(true);
    expect(result.fieldCount).toBe(2);
  });

  test("FormResult should handle invalid fields", () => {
    const result: FormResult = {
      fields: [
        { name: "name", value: "", valid: false, error: "Required" },
      ],
      valid: false,
      fieldCount: 1,
    };
    expect(result.valid).toBe(false);
    expect(result.fields[0].error).toBe("Required");
  });
});

// ============================================
// Form Functions Tests
// ============================================
describe("Form Functions Tests", () => {
  test("confirm should be a function", () => {
    expect(typeof confirm).toBe("function");
  });

  test("prompt should be a function", () => {
    expect(typeof prompt).toBe("function");
  });

  test("selectMenu should be a function", () => {
    expect(typeof selectMenu).toBe("function");
  });

  test("collectForm should be a function", () => {
    expect(typeof collectForm).toBe("function");
  });

  test("displayForm should be a function", () => {
    expect(typeof displayForm).toBe("function");
  });

  test("formTextInput should be a function", () => {
    expect(typeof formTextInput).toBe("function");
  });

  test("formPasswordInput should be a function", () => {
    expect(typeof formPasswordInput).toBe("function");
  });

  test("formConfirmInput should be a function", () => {
    expect(typeof formConfirmInput).toBe("function");
  });

  test("formSelectInput should be a function", () => {
    expect(typeof formSelectInput).toBe("function");
  });

  test("formMultiSelectInput should be a function", () => {
    expect(typeof formMultiSelectInput).toBe("function");
  });

  test("formNumberInput should be a function", () => {
    expect(typeof formNumberInput).toBe("function");
  });
});

// ============================================
// Form Field Config Validation Tests
// ============================================
describe("FormFieldConfig Validation Tests", () => {
  test("required field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "test",
      fieldType: "text",
      required: true,
    };
    expect(config.required).toBe(true);
  });

  test("minLength and maxLength should be numbers", () => {
    const config: FormFieldConfig = {
      name: "test",
      fieldType: "text",
      minLength: 1,
      maxLength: 100,
    };
    expect(typeof config.minLength).toBe("number");
    expect(typeof config.maxLength).toBe("number");
  });

  test("options should be string array", () => {
    const config: FormFieldConfig = {
      name: "test",
      fieldType: "select",
      options: ["a", "b", "c"],
    };
    expect(Array.isArray(config.options)).toBe(true);
    expect(config.options?.every(o => typeof o === "string")).toBe(true);
  });
});

// ============================================
// Form Field Types Tests
// ============================================
describe("Form Field Types Tests", () => {
  test("text field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "username",
      fieldType: "text",
      placeholder: "Enter username",
      required: true,
    };
    expect(config.fieldType).toBe("text");
  });

  test("password field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "password",
      fieldType: "password",
      minLength: 8,
      required: true,
    };
    expect(config.fieldType).toBe("password");
  });

  test("confirm field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "agree",
      fieldType: "confirm",
      default: "false",
    };
    expect(config.fieldType).toBe("confirm");
  });

  test("select field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "color",
      fieldType: "select",
      options: ["red", "green", "blue"],
    };
    expect(config.fieldType).toBe("select");
    expect(config.options).toHaveLength(3);
  });

  test("multiSelect field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "features",
      fieldType: "multiSelect",
      options: ["feature1", "feature2", "feature3"],
    };
    expect(config.fieldType).toBe("multiSelect");
  });

  test("number field config should be valid", () => {
    const config: FormFieldConfig = {
      name: "age",
      fieldType: "number",
      minLength: 0,
      maxLength: 150,
    };
    expect(config.fieldType).toBe("number");
  });
});
