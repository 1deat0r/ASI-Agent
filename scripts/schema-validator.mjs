import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

function deepEqual(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

function typeMatches(expected, value) {
  switch (expected) {
    case "null":
      return value === null;
    case "array":
      return Array.isArray(value);
    case "object":
      return value !== null && typeof value === "object" && !Array.isArray(value);
    case "integer":
      return Number.isInteger(value);
    case "number":
      return typeof value === "number" && Number.isFinite(value);
    default:
      return typeof value === expected;
  }
}

function loadSchema(path, cache) {
  const absolute = resolve(path);
  if (!cache.has(absolute)) {
    cache.set(absolute, JSON.parse(readFileSync(absolute, "utf8")));
  }
  return { schema: cache.get(absolute), path: absolute };
}

function validateNode(schema, value, location, schemaPath, cache, errors) {
  if (typeof schema === "boolean") {
    if (!schema) errors.push(`${location}: rejected by false schema`);
    return;
  }
  if (schema.$ref) {
    const [referencePath, fragment] = schema.$ref.split("#", 2);
    if (fragment) {
      errors.push(`${location}: schema fragments are not supported by the local validator`);
      return;
    }
    const loaded = loadSchema(resolve(dirname(schemaPath), referencePath), cache);
    validateNode(loaded.schema, value, location, loaded.path, cache, errors);
    return;
  }

  if (schema.type !== undefined) {
    const expected = Array.isArray(schema.type) ? schema.type : [schema.type];
    if (!expected.some((candidate) => typeMatches(candidate, value))) {
      errors.push(`${location}: expected ${expected.join(" or ")}`);
      return;
    }
  }
  if (schema.const !== undefined && !deepEqual(schema.const, value)) {
    errors.push(`${location}: value does not match const`);
  }
  if (schema.enum && !schema.enum.some((candidate) => deepEqual(candidate, value))) {
    errors.push(`${location}: value is outside enum`);
  }

  if (typeof value === "string") {
    if (schema.minLength !== undefined && value.length < schema.minLength) {
      errors.push(`${location}: string is shorter than ${schema.minLength}`);
    }
    if (schema.pattern && !new RegExp(schema.pattern, "u").test(value)) {
      errors.push(`${location}: string does not match ${schema.pattern}`);
    }
    if (
      schema.format === "date-time" &&
      (!/^\d{4}-\d{2}-\d{2}T/.test(value) || Number.isNaN(Date.parse(value)))
    ) {
      errors.push(`${location}: value is not an RFC 3339 date-time`);
    }
  }

  if (typeof value === "number" && schema.minimum !== undefined && value < schema.minimum) {
    errors.push(`${location}: number is below ${schema.minimum}`);
  }

  if (Array.isArray(value)) {
    if (schema.minItems !== undefined && value.length < schema.minItems) {
      errors.push(`${location}: array has fewer than ${schema.minItems} items`);
    }
    if (schema.maxItems !== undefined && value.length > schema.maxItems) {
      errors.push(`${location}: array has more than ${schema.maxItems} items`);
    }
    if (schema.uniqueItems) {
      const encoded = value.map((item) => JSON.stringify(item));
      if (new Set(encoded).size !== encoded.length) errors.push(`${location}: array items are not unique`);
    }
    if (schema.items) {
      value.forEach((item, index) => {
        validateNode(schema.items, item, `${location}[${index}]`, schemaPath, cache, errors);
      });
    }
  }

  if (value !== null && typeof value === "object" && !Array.isArray(value)) {
    const properties = schema.properties ?? {};
    for (const required of schema.required ?? []) {
      if (!Object.hasOwn(value, required)) errors.push(`${location}: missing required property ${required}`);
    }
    if (schema.additionalProperties === false) {
      for (const key of Object.keys(value)) {
        if (!Object.hasOwn(properties, key)) errors.push(`${location}: unexpected property ${key}`);
      }
    }
    for (const [key, childSchema] of Object.entries(properties)) {
      if (Object.hasOwn(value, key)) {
        validateNode(childSchema, value[key], `${location}.${key}`, schemaPath, cache, errors);
      }
    }
  }
}

export function validateAgainstSchema(schemaPath, document, label = "document") {
  const cache = new Map();
  const loaded = loadSchema(schemaPath, cache);
  const errors = [];
  validateNode(loaded.schema, document, "$", loaded.path, cache, errors);
  if (errors.length > 0) {
    throw new Error(`${label} failed ${loaded.path}:\n${errors.slice(0, 25).join("\n")}`);
  }
}
