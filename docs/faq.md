# Frequently Asked Questions

## Table of Contents

- [General](#general)
- [Installation](#installation)
- [Syntax](#syntax)
- [Functions](#functions)
- [Arrays](#arrays)
- [Objects](#objects)
- [Classes](#classes)
- [Error Handling](#error-handling)
- [Performance](#performance)
- [Tooling](#tooling)

## General

### What is Nexora?

Nexora is a simple, clean, AI-native programming language built in Rust. It features a clean syntax, pattern matching, string interpolation, classes with inheritance, and a built-in standard library.

### What file extension do Nexora files use?

Nexora files use the `.nx` extension.

### Is Nexora compiled or interpreted?

Nexora uses an interpreter (tree-walk interpreter). Future versions may include a bytecode compiler and VM for better performance.

### Can Nexora be used for web development?

Yes! Nexora includes a built-in HTTP server via the `serve()` function and HTML generation helpers.

### Is Nexora statically typed?

Nexora is dynamically typed with optional type annotations for documentation. Type annotations don't enforce type safety at runtime.

## Installation

### How do I install Nexora?

1. Download the binary from the [releases page](https://github.com/nexora-lang/nexora/releases)
2. Add it to your PATH
3. Run `nexora version` to verify

Or build from source:

```bash
git clone https://github.com/nexora-lang/nexora.git
cd nexora
cargo build --release
```

### What platforms are supported?

- Windows (x86_64)
- macOS (x86_64, ARM64)
- Linux (x86_64)

### How do I update Nexora?

Download the latest binary from the releases page and replace the existing one.

## Syntax

### Does Nexora use semicolons?

No, semicolons are optional. Nexora uses newlines as statement separators.

### Does Nexora use indentation for blocks?

No, Nexora uses curly braces `{}` for blocks, similar to JavaScript and Rust.

### How do I write multi-line strings?

Use triple quotes:

```nexora
let text = """
This is a
multi-line string
"""
```

### What comment styles are supported?

```nexora
// Single-line comment

/*
  Multi-line comment
*/
```

## Functions

### How do I define a function?

```nexora
func greet(name) {
    return "Hello, " + name + "!"
}
```

### Can functions have default parameters?

Yes:

```nexora
func greet(name, title = "Mr.") {
    print "Hello, " + title + " " + name
}

greet("Smith")          // Hello, Mr. Smith
greet("Smith", "Dr.")   // Hello, Dr. Smith
```

### What are arrow functions?

Arrow functions provide concise syntax:

```nexora
let double = x => x * 2
let add = (a, b) => a + b
```

### Are functions first-class values?

Yes! Functions can be stored in variables, passed as arguments, and returned from other functions:

```nexora
func apply(fn, value) {
    return fn(value)
}

let result = apply(x => x * 2, 5)  // 10
```

### What are closures?

Closures capture variables from their enclosing scope:

```nexora
let multiplier = 3
let multiply = x => x * multiplier

print multiply(5)  // 15
```

## Arrays

### How do I create an array?

```nexora
let arr = [1, 2, 3, 4, 5]
```

### How do I access elements?

Zero-indexed:

```nexora
let arr = [1, 2, 3]
print arr[0]  // 1
```

### How do I add elements?

Use `push()`:

```nexora
let arr = [1, 2]
let extended = push(arr, 3)  // [1, 2, 3]
```

### How do I iterate over an array?

```nexora
let arr = [1, 2, 3]
for item in arr {
    print item
}
```

### How do I map/filter/reduce?

```nexora
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)
let evens = filter(numbers, x => x % 2 == 0)
let sum = reduce(numbers, (acc, x) => acc + x, 0)
```

## Objects

### How do I create an object?

```nexora
let person = {
    name: "Nexora",
    age: 1
}
```

### How do I access properties?

Dot notation or bracket notation:

```nexora
print person.name
print person["name"]
```

### How do I add properties?

```nexora
person.version = 4
```

### How do I iterate over an object?

```nexora
for key in keys(person) {
    print key + ": " + str(person[key])
}
```

## Classes

### How do I define a class?

```nexora
class Dog {
    init(name) {
        this.name = name
    }

    bark() {
        return this.name + " barks!"
    }
}
```

### How do I create an instance?

```nexora
let rex = new Dog("Rex")
print rex.bark()
```

### How do I use inheritance?

```nexora
class Animal {
    init(name) {
        this.name = name
    }
}

class Dog extends Animal {
    init(name) {
        super(name)
    }
}
```

### What is `this`?

`this` refers to the current instance inside methods:

```nexora
class Person {
    init(name) {
        this.name = name
    }

    greet() {
        return "Hello, I'm " + this.name
    }
}
```

## Error Handling

### How do I handle errors?

```nexora
try {
    let result = riskyOperation()
} catch (error) {
    print "Error: " + error
} finally {
    print "Cleanup"
}
```

### How do I throw errors?

```nexora
func divide(a, b) {
    if b == 0 {
        throw "Division by zero"
    }
    return a / b
}
```

### Can I nest try-catch blocks?

Yes:

```nexora
try {
    try {
        throw "inner error"
    } catch (e) {
        throw "outer error"
    }
} catch (e) {
    print e  // "outer error"
}
```

## Performance

### Is Nexora fast?

Nexora is interpreted, so it's slower than compiled languages like Rust or C++. However, it's designed for simplicity and ease of use.

### Can I use Nexora for production?

Nexora is suitable for scripting, tooling, web development, and prototyping. For performance-critical applications, consider using a compiled language.

## Tooling

### Is there a VS Code extension?

Yes! A VS Code extension is available with syntax highlighting and basic language support.

### Is there a REPL?

Yes! Start it with:

```bash
nexora repl
```

### Is there a package manager?

Yes! Use `nxm` (Nexora Package Manager):

```bash
nxm init
nxm add http
nxm install
```

### How do I format code?

```bash
nexora fmt file.nx
```

> **Note:** Formatter is not yet fully implemented.

### How do I run tests?

```bash
nexora test
```

> **Note:** Test runner is not yet fully implemented.

### How do I check my project for issues?

```bash
nexora doctor .
```
