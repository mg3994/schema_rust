const test = require("node:test");
const assert = require("node:assert");
const { schemaOrgOutput } = require("../dist/index");

test("schemaOrgOutput generates schema, hydrates, and validates", async () => {
  const output = schemaOrgOutput("Person");
  assert.strictEqual(output.typeName, "Person");

  const raw = { name: "Jane Doe" };
  const hydrated = await output.hydrate(raw);

  assert.strictEqual(hydrated["@type"], "Person");
  assert.strictEqual(hydrated["@context"], "https://schema.org");
  assert.strictEqual(hydrated.name, "Jane Doe");

  const validation = await output.validate(hydrated);
  assert.strictEqual(validation.isValid, true);
});
