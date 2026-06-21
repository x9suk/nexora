# Nexora

**Simple as Python. Powerful as JavaScript. Fast as Go.**

A modern programming language with a clean syntax, powerful features, and a complete toolchain — built in Rust.

```
nexora> print("Hello, World!")
Hello, World!
```

---

## Install

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/x9suk/nexora/main/install.ps1 | iex
```

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/x9suk/nexora/main/install.sh | sh
```

### Build from Source

```bash
git clone https://github.com/x9suk/nexora.git
cd nexora
cargo build --release
```

The binaries will be in `target/release/`.

---

## Quick Start

```bash
# Start REPL
nexora

# Run a file
nexora run hello.nx

# Create a project
nxm init

# Install a package
nxm install lodash-nx

# Format code
nexora fmt file.nx

# Lint code
nexora lint file.nx
```

---

## Language Features

### Variables & Constants

```nx
let name = "Nexora"
const PI = 3.14159

let age: int = 25
let items: Array<int> = [1, 2, 3]
```

### Functions

```nx
func add(a, b) {
    return a + b
}

// Arrow lambdas
let double = x => x * 2
let sum = (a, b) => a + b

// Default parameters
func greet(name, greeting = "Hello") {
    print("${greeting}, ${name}!")
}
```

### Classes & Inheritance

```nx
class Animal {
    func init(name) {
        this.name = name
    }
    func speak() {
        return "${this.name} makes a sound"
    }
}

class Dog extends Animal {
    func init(name) {
        super(name)
    }
    func bark() {
        return "${this.name} barks!"
    }
}

let dog = new Dog("Rex")
print(dog.bark())
```

### Pattern Matching

```nx
match command {
    "quit" => exit(),
    "help" => show_help(),
    _ => print("Unknown command"),
}
```

### Error Handling

```nx
try {
    let data = read_file("config.json")
    let config = json_parse(data)
} catch (e) {
    print("Error: ${e}")
} finally {
    cleanup()
}
```

### Modules

```nx
import { sqrt, pow } from "lib/math.nx"
import math from "lib/math.nx"
import "lib/string.nx"
```

### Closures

```nx
func counter() {
    let count = 0
    return () => {
        count = count + 1
        return count
    }
}

let c = counter()
print(c())  // 1
print(c())  // 2
```

### Web Server

```nx
func handler(method, path) {
    if path == "/api/hello" {
        return '{"message": "Hello from Nexora!"}'
    }
    return "<h1>Welcome to Nexora</h1>"
}

serve(8080, handler)
print("Server running on port 8080")
```

---

## Standard Library

| Module | Description |
|--------|-------------|
| `lib/math.nx` | Math functions: sqrt, pow, sin, cos, random |
| `lib/string.nx` | String operations: upper, lower, split, join |
| `lib/collection.nx` | Collections: map, filter, reduce, sort |

---

## Package Manager (nxm)

```bash
nxm init                  # Create nexora.json
nxm install               # Install all dependencies
nxm install lodash-nx     # Install a package
nxm remove lodash-nx      # Remove a package
nxm list                  # List installed packages
nxm modules               # List available packages
```

---

## CLI Commands

| Command | Description |
|---------|-------------|
| `nexora` | Start REPL |
| `nexora run <file>` | Run a .nx file |
| `nexora fmt <file>` | Format source code |
| `nexora lint <file>` | Lint source code |
| `nexora help` | Show help |

---

## Project Structure

```
nexora/
├── src/                    # Language interpreter (Rust)
│   ├── main.rs            # CLI entry point
│   ├── lexer.rs           # Tokenizer
│   ├── parser.rs          # Parser
│   ├── ast.rs             # Abstract Syntax Tree
│   ├── interpreter.rs     # Tree-walking interpreter
│   ├── value.rs           # Runtime values
│   └── error.rs           # Error types
├── lib/                    # Standard library (.nx)
│   ├── math.nx
│   ├── string.nx
│   └── collection.nx
├── nxm/                    # Package manager (Rust)
├── tools/                  # Developer tools
│   ├── fmt/               # Code formatter
│   └── lint/              # Linter
├── language-server/        # LSP server
├── extensions/             # Editor extensions
│   ├── vscode/
│   ├── sublime-text/
│   ├── vim/
│   └── ...
├── website/                # nexora.dev
├── registry/               # Package registry
└── examples/               # Example programs
```

---

## Editor Support

- **VS Code** — Syntax highlighting, snippets, autocomplete
- **Sublime Text** — Syntax highlighting
- **Vim/Neovim** — Syntax highlighting
- **Emacs** — Syntax highlighting
- **JetBrains** — Syntax highlighting
- **Notepad++** — Syntax highlighting
- **Zed** — Syntax highlighting

---

## Documentation

- [Language Specification](BLUEPRINT.md)
- [Getting Started](docs/)
- [Standard Library](lib/)
- [Package Registry](registry/)

---

## Contributing

See [CONTRIBUTING.md](opennexora/CONTRIBUTING.md) for guidelines.

---

## License

MIT License

---

<p align="center">
  <b>Built with ❤️ by the Nexora community</b>
</p>
