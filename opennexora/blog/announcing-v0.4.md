# Announcing Nexora v0.4

*June 15, 2026*

*By Dr. Elena Rodriguez, TSC Chair*

---

We're thrilled to announce the release of Nexora v0.4, a major update that brings significant performance improvements, new language features, and an enhanced developer experience. This release represents months of hard work from the core team and community contributors.

## What's New in v0.4

### Performance Improvements

Nexora v0.4 includes a completely rewritten garbage collector that delivers up to 40% faster execution times for typical workloads. The new generational GC reduces pause times and improves memory efficiency.

```nx
// Benchmarks show significant improvements
// Before (v0.3): 1000ms for 1M operations
// After (v0.4): 600ms for 1M operations

let startTime = Date.now();
for (let i = 0; i < 1000000; i++) {
  // Perform operation
}
let duration = Date.now() - startTime;
console.log(`Completed in ${duration}ms`);
```

### Pattern Matching

One of the most requested features is now available. Pattern matching provides a clean, expressive way to handle complex conditional logic.

```nx
// Pattern matching syntax
fn describe(value: Any) -> String {
  return match value {
    is Number when value > 0 => "Positive number",
    is Number when value < 0 => "Negative number",
    is Number => "Zero",
    is String when value.length > 0 => "Non-empty string",
    is String => "Empty string",
    is Array when value.length > 0 => "Non-empty array",
    is Array => "Empty array",
    is null => "Null value",
    _ => "Unknown type"
  };
}

console.log(describe(42));    // "Positive number"
console.log(describe(""));    // "Empty string"
console.log(describe([]));    // "Empty array"
```

### Enhanced Type Inference

The type system has been significantly improved with better inference capabilities, reducing the need for explicit type annotations.

```nx
// Before (v0.3) - explicit types required
let numbers: Array<Number> = [1, 2, 3];
let result: Array<Number> = numbers.map((n: Number) => n * 2);

// After (v0.4) - types inferred automatically
let numbers = [1, 2, 3];
let result = numbers.map((n) => n * 2);
// TypeScript knows result is Array<Number>
```

### Async Iterators

Native support for async iterators enables more elegant handling of asynchronous data streams.

```nx
// Async iterator for database queries
async fn* fetchUsers() {
  let page = 1;
  let hasMore = true;
  
  while (hasMore) {
    let response = await fetch(`/api/users?page=${page}`);
    let data = await response.json();
    
    for (let user of data.users) {
      yield user;
    }
    
    hasMore = data.hasMore;
    page++;
  }
}

// Usage
for await (let user of fetchUsers()) {
  console.log(user.name);
}
```

### New Standard Library Functions

Nexora v0.4 adds several new utility functions to the standard library:

```nx
import { chunk, flatten, groupBy, sortBy } from 'nexora:stdlib';

// Chunk array into groups
let chunks = chunk([1, 2, 3, 4, 5, 6], 3);
// Result: [[1, 2, 3], [4, 5, 6]]

// Flatten nested arrays
let flat = flatten([[1, 2], [3, [4, 5]], 6]);
// Result: [1, 2, 3, 4, 5, 6]

// Group objects by property
let users = [
  { name: 'Alice', department: 'Engineering' },
  { name: 'Bob', department: 'Marketing' },
  { name: 'Charlie', department: 'Engineering' }
];
let grouped = groupBy(users, (u) => u.department);
// Result: { Engineering: [...], Marketing: [...] }

// Sort by multiple criteria
let sorted = sortBy(products, [(p) => p.category, (p) => -p.price]);
```

### Improved Error Messages

Error messages are now more descriptive and include helpful suggestions for fixing common issues.

```nx
// Before (v0.3)
// Error: Type mismatch

// After (v0.4)
// Error: Type mismatch in function 'add'
//   Expected: Number
//   Received: String
//   
//   Hint: Convert the string to a number using:
//     Number.parseInt(value) or Number.parseFloat(value)
//   
//   Example:
//     let result = add(Number.parseInt(value), 10);
```

### Developer Tools Improvements

The VS Code extension has been significantly enhanced with better IntelliSense, debugging support, and refactoring tools.

- **Improved Autocomplete**: Faster, more accurate suggestions
- **Better Debugger**: Step through async code seamlessly
- **Refactoring Tools**: Rename variables, extract functions, and more
- **Inline Diagnostics**: See errors and warnings directly in the editor

## Breaking Changes

While we've worked to minimize breaking changes, some updates may require code modifications:

### 1. Removed Deprecated APIs

```nx
// Before (v0.3)
let result = util.inherits(Child, Parent);

// After (v0.4)
// Use class inheritance instead
class Child extends Parent {
  // ...
}
```

### 2. Updated Package Resolution

Package resolution now follows stricter rules. Ensure your `package.nx` files are properly formatted:

```nx
// package.nx
{
  "name": "my-package",
  "version": "1.0.0",
  "dependencies": {
    "nexora": "^0.4.0"
  }
}
```

### 3. Stricter Type Checking

Some previously allowed type coercions are now errors:

```nx
// Before (v0.3) - allowed with warning
let num: Number = "123";

// After (v0.4) - error
// Error: Cannot assign String to Number
// Fix: let num: Number = Number.parseInt("123");
```

## Migration Guide

To upgrade from v0.3 to v0.4:

```bash
# Update Nexora
nxm update nexora@0.4

# Check for breaking changes
nxm audit

# Run your test suite
npx nexora-test

# Update dependencies
nxm update
```

## Performance Benchmarks

We've conducted extensive benchmarks comparing v0.4 to v0.3:

| Benchmark | v0.3 | v0.4 | Improvement |
|-----------|------|------|-------------|
| Startup time | 150ms | 90ms | 40% faster |
| Memory usage | 120MB | 85MB | 29% less |
| HTTP requests/sec | 10,000 | 15,000 | 50% more |
| File I/O ops/sec | 50,000 | 75,000 | 50% more |

## Community Contributions

We'd like to thank all the community members who contributed to this release:

- **@alex-dev** - Pattern matching implementation
- **@sarah-codes** - Async iterator support
- **@mike-types** - Type inference improvements
- **@lisa-perf** - Performance optimizations
- **@john-docs** - Documentation improvements

And many others who reported bugs, suggested features, and helped test the release.

## What's Next

### v0.4.1 (July 2026)
- Bug fixes and stability improvements
- Additional standard library functions
- Enhanced IDE support

### v0.5 (September 2026)
- WebAssembly compilation target
- Improved module system
- Package signing and verification

### v1.0 (December 2026)
- Stable API guarantee
- Long-term support (LTS)
- Enterprise features

## Get Involved

Join the Nexora community and help shape the future of the language:

- **GitHub**: [github.com/opennexora](https://github.com/opennexora)
- **Discord**: [discord.gg/nexora](https://discord.gg/nexora)
- **Twitter**: [@OpenNexora](https://twitter.com/OpenNexora)
- **Documentation**: [nexora.dev](https://nexora.dev)

## Support the Project

If you find Nexora valuable, consider supporting the project:

- **Star us on GitHub**: Help others discover Nexora
- **Contribute**: Submit bug reports, documentation, or code
- **Spread the Word**: Share Nexora with your network
- **Sponsor**: Support the foundation financially

---

*Thank you for being part of the Nexora community. We're excited to see what you'll build with v0.4!*

*— The OpenNexora Foundation Team*