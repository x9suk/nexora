# nexora-test

Testing framework for the Nexora runtime. Provides describe/it blocks, expectations, and mocks.

## Installation

```bash
nxm install nexora-test
```

## Quick Start

```nx
import { describe, it, expect, run } from "nexora-test";

describe("Math", () => {
  it("adds numbers", () => {
    expect(1 + 1).toBe(2);
  });
});

await run();
```

## Running Tests

```bash
nxm test
```

## `describe` / `it`

```nx
describe("User model", () => {
  it("creates a user", () => {
    const user = { name: "Alice", age: 30 };
    expect(user.name).toBe("Alice");
  });

  it("validates age", () => {
    expect(() => createUser({ age: -1 })).toThrow("Invalid age");
  });
});
```

## Expectations

### Equality

```nx
expect(1 + 1).toBe(2);
expect({ a: 1 }).toEqual({ a: 1 });
```

### Truthiness

```nx
expect(true).toBeTruthy();
expect(false).toBeFalsy();
expect(undefined).toBeUndefined();
expect(null).toBeNull();
```

### Types

```nx
expect([]).toBeInstanceOf(Array);
```

### Numbers

```nx
expect(5).toBeGreaterThan(3);
expect(3).toBeLessThan(5);
```

### Strings and Arrays

```nx
expect("hello world").toContain("world");
expect([1, 2, 3]).toContain(2);
expect("hello").toHaveLength(5);
```

### Objects

```nx
expect({ a: 1, b: 2 }).toHaveProperty("a");
expect({ a: 1, b: 2 }).toHaveProperty("a", 1);
```

### Regex

```nx
expect("hello world").toMatch(/world/);
```

### Functions

```nx
expect(() => { throw new Error("oops"); }).toThrow();
expect(() => { throw new Error("oops"); }).toThrow("oops");
```

### Negation

```nx
expect(1).not.toBe(2);
expect([]).not.toBeNull();
```

## Before/After Hooks

```nx
let db;

beforeEach(() => {
  db = createDatabase();
});

afterEach(() => {
  db.close();
});

describe("Users", () => {
  it("inserts a user", () => {
    db.insert("users", { name: "Alice" });
    expect(db.count("users")).toBe(1);
  });
});
```

### Before/After All

```nx
before(() => {
  // runs once before all tests
});

after(() => {
  // runs once after all tests
});
```

## Mocks

### `mock(fn)`

Creates a mock function.

```nx
const fn = mock((x) => x * 2);
fn(5);
expect(fn).toHaveBeenCalled();
expect(fn).toHaveBeenCalledWith(5);
expect(fn).toHaveBeenCalledTimes(1);
expect(fn).toHaveReturnedWith(10);
```

### Mock Implementation

```nx
const fn = mock();
fn.mockReturnValue(42);
fn(); // 42

fn.mockImplementation((x) => x + 1);
fn(5); // 6
```

### Spy

```nx
const obj = { greet: (name) => `Hello ${name}` };
const spy = spy(obj, "greet");

obj.greet("Alice");
expect(spy).toHaveBeenCalledWith("Alice");

spy.mockRestore();
```

## Assertions

```nx
import { assert } from "nexora-test";

assert(1 + 1 === 2, "Math works");
assert.equal(1 + 1, 2);
assert.deepEqual({ a: 1 }, { a: 1 });
assert.ok(true);
assert.throws(() => { throw new Error("fail"); });
```

## Full Example

```nx
import { describe, it, expect, mock, beforeEach, run } from "nexora-test";

function createUserService(repo) {
  return {
    create: (name) => {
      if (!name) throw new Error("Name required");
      return repo.save({ name, createdAt: Date.now() });
    },
    list: () => repo.findAll(),
  };
}

describe("UserService", () => {
  let repo;
  let service;

  beforeEach(() => {
    repo = {
      save: mock((data) => ({ id: 1, ...data })),
      findAll: mock(() => [{ id: 1, name: "Alice" }]),
    };
    service = createUserService(repo);
  });

  it("creates a user", () => {
    const user = service.create("Bob");
    expect(user.name).toBe("Bob");
    expect(repo.save).toHaveBeenCalled();
  });

  it("throws on empty name", () => {
    expect(() => service.create("")).toThrow("Name required");
  });

  it("lists users", () => {
    const users = service.list();
    expect(users).toHaveLength(1);
  });
});

await run();
```

## License

MIT
