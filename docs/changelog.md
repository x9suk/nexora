# Changelog

## Table of Contents

- [v0.1.0](#v010)
- [v0.2.0](#v020)
- [v0.3.0](#v030)
- [v0.4.0](#v040)
- [Upcoming](#upcoming)

## v0.1.0

**Initial Release**

### Features

- Variables with `let` and `const`
- Data types: integer, float, string, boolean, null
- Arithmetic, comparison, and logical operators
- String concatenation
- `if`/`elif`/`else` conditionals
- `while` loops
- `for...in` loops
- `break` and `continue` statements
- Functions with `func` keyword
- Default parameters
- Recursive functions
- Arrays with index access
- Objects with property access
- Nested arrays and objects
- `print` function
- `typeof` function
- `len` function
- Type conversion: `str`, `num`
- String operations: `split`, `join`
- Array operations: `push`, `pop`
- Math functions: `Math.PI`, `Math.E`, `Math.abs`, `Math.max`, `Math.min`, `Math.sqrt`, `Math.pow`
- Web server with `serve()`
- HTML generation helpers
- HTTP functions: `http_get`, `http_post`
- JSON functions: `json_parse`, `json_stringify`
- File I/O: `read_file`, `write_file`, `append_file`
- Import system with `import`
- Ternary operator
- Comments (`//` and `/* */`)

## v0.2.0

**Classes & Error Handling**

### New Features

- Class declarations with `class` keyword
- Constructor with `init` method
- Instance methods
- `this` keyword for instance access
- Method chaining
- `try`/`catch`/`finally` error handling
- `throw` statement for custom errors
- `assert` function for testing
- `test` blocks for test definitions
- Date/Time functions: `now()`, `timestamp()`
- Collection functions: `sort`, `reverse`, `unique`, `flatten`, `range`, `zip`
- Higher-order functions: `map`, `filter`, `reduce`
- Object functions: `keys`, `values`, `entries`
- Anonymous functions with `func` keyword
- String methods: `toUpperCase`, `toLowerCase`, `trim`, `includes`, `startsWith`, `endsWith`, `indexOf`, `charAt`
- Array methods: `push`, `pop`, `indexOf`, `slice`

## v0.3.0

**Inheritance, Lambdas & Pattern Matching**

### New Features

- String interpolation with `${expression}` syntax
- Class inheritance with `extends` keyword
- `super()` to call parent constructor
- `super.method()` to call parent methods
- Lambda expressions with `=>` syntax
- Single-parameter lambdas: `x => expression`
- Multi-parameter lambdas: `(a, b) => expression`
- Match expressions with `match` keyword
- Literal patterns
- Wildcard pattern with `_`
- Variable patterns

### Improvements

- Better error messages
- Improved parser performance

## v0.4.0

**Type Annotations & Standard Library**

### New Features

- Type annotations: `let x: int = 5`
- Supported types: `int`, `float`, `string`, `bool`
- `parseInt` and `parseFloat` functions
- Advanced math library: `sin`, `cos`, `tan`, `log`, `ln`
- String library: `pad_left`, `pad_right`, `center`, `count`, `is_alpha`, `is_digit`
- Collection library: `flat_map`, `group_by`, `count_by`, `partition`, `shuffle`, `sample`, `sum`, `product`, `min`, `max`, `average`
- `async`/`await` support for async functions
- `every`, `some`, `find`, `find_index` array methods
- `concat`, `flat_map` array operations
- Improved closure support

### Improvements

- Better variable scoping
- Improved closure environment handling
- Enhanced error handling in REPL

## Upcoming

### v0.5.0 (Planned)

- Improved import system with file resolution
- Module system with proper exports
- Enhanced pattern matching with guards
- Bytecode compiler for better performance
- Package manager improvements

### v1.0.0 (Planned)

- Complete type system with inference
- Bytecode compiler and VM
- Full standard library
- Complete CLI tooling
- VS Code extension improvements
- Language server protocol

### v2.0.0 (Planned)

- JIT compiler
- Optimizing passes
- Concurrent execution
- WebAssembly target

---

## Version History Summary

| Version | Key Features |
|---------|--------------|
| v0.1.0 | Core language, variables, functions, arrays, objects |
| v0.2.0 | Classes, error handling, collections, testing |
| v0.3.0 | Inheritance, lambdas, pattern matching, string interpolation |
| v0.4.0 | Type annotations, advanced stdlib, async/await |
