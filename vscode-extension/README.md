# Nexora Language

Complete Nexora language support for Visual Studio Code.

## Features

- **Syntax Highlighting** - Full syntax highlighting for `.nx` files with support for:
  - Keywords (let, func, class, if, else, while, for, etc.)
  - Built-in functions (print, sqrt, pow, map, filter, etc.)
  - Strings with interpolation
  - Comments (single-line and block)
  - Numbers and booleans
  - Operators and delimiters

- **Code Snippets** - 35+ snippets for common patterns:
  - `func` - Create a function
  - `class` - Create a class
  - `if` / `ifelse` - Create if statements
  - `for` / `foreach` - Create loops
  - `import` / `importfrom` - Import modules
  - `async` - Create async functions
  - `try` - Create try-catch blocks
  - `describe` / `it` - Create tests
  - And many more...

- **File Icons** - Custom icons for Nexora files

- **Language Configuration** - Auto-closing brackets, comments, and more

## Installation

1. Open VS Code
2. Press `Ctrl+Shift+X` to open Extensions
3. Search for "Nexora Language"
4. Click Install

## Usage

Open any `.nx` file and start coding! The extension will automatically activate.

## Snippets

| Snippet | Description |
|---------|-------------|
| `func` | Create a function |
| `arrow` | Create an arrow function |
| `class` | Create a class |
| `classext` | Create a class with inheritance |
| `let` | Declare a variable |
| `const` | Declare a constant |
| `if` | Create an if statement |
| `ifelse` | Create an if-else statement |
| `for` | Create a for loop |
| `foreach` | Create a for-each loop |
| `while` | Create a while loop |
| `print` | Print to console |
| `import` | Import a module |
| `importfrom` | Named import |
| `async` | Create an async function |
| `try` | Create a try-catch block |
| `lambda` | Create a lambda |
| `match` | Create a match statement |
| `describe` | Create a test describe block |
| `it` | Create a test case |

## Language Syntax

```nexora
// Variables
let name = "Nexora"
const PI = 3.14

// Functions
func add(a, b) {
    return a + b
}

// Arrow functions
let multiply = (a, b) => a * b

// Classes
class Animal {
    init(name) {
        this.name = name
    }
    
    speak() {
        return this.name + " makes a sound"
    }
}

// Inheritance
class Dog extends Animal {
    speak() {
        return this.name + " barks"
    }
}

// Pattern matching
match (value) {
    1 => "one",
    2 => "two",
    _ => "other"
}

// Async/Await
async func fetchData() {
    let response = await http.get("https://api.example.com")
    return json.parse(response.body)
}

// Import modules
import { sqrt, pow } from "math"
import http from "http"

// Control flow
if (condition) {
    // do something
} else {
    // do something else
}

for (let item of collection) {
    // process item
}
```

## Standard Library

Nexora comes with a powerful standard library:

- **math** - sqrt, pow, abs, min, max, floor, ceil, round
- **string** - split, join, contains, upper, lower, trim, replace
- **collection** - map, filter, reduce, sort, find
- **http** - get, post, put, delete
- **fs** - read, write, append, exists, mkdir
- **json** - parse, stringify
- **os** - env, args, platform, sleep, exec
- **time** - now, sleep, timestamp
- **test** - describe, it, expect

## Package Manager

Use `nxm` to install packages:

```bash
nxm init
nxm install lodash-nx
nxm install discord-nx
nxm install minecraft-nx
```

## Community

- [GitHub](https://github.com/nexora-lang)
- [Discord](https://discord.gg/nexora)
- [Documentation](https://nexora.dev/docs)

## License

MIT
