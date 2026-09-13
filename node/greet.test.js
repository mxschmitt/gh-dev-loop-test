const assert = require("node:assert/strict");
const { test } = require("node:test");
const { greet } = require("./greet.js");

test("greet", () => {
  assert.equal(greet("merlin"), "hello, merlin!");
});
