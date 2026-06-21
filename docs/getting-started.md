# Getting Started with Nexora

## Table of Contents

- [Installation](#installation)
- [Your First Program](#your-first-program)
- [Running Nexora Code](#running-nexora-code)
- [REPL](#repl)
- [Creating a Project](#creating-a-project)

## Installation

### Option 1: Download Pre-built Binary

1. Download `nexora.exe` (or `nexora` on macOS/Linux) from the [releases page](https://github.com/nexora-lang/nexora/releases)
2. Place it in any folder (e.g., `C:\nexora` or `/usr/local/bin`)
3. Add that folder to your `PATH`
4. Verify installation:

```bash
nexora version
```

### Option 2: Build from Source

```bash
# Install Rust (one-time setup)
# Visit https://rustup.rs

# Clone and build
git clone https://github.com/nexora-lang/nexora.git
cd nexora
cargo build --release

# Install (Windows)
.\install.bat

# Install (macOS/Linux)
chmod +x install.sh
./install.sh
```

## Your First Program

Create a file called `hello.nx`:

```nexora
print "Hello, World!"
print "Welcome to Nexora!"
```

Run it:

```bash
nexora run hello.nx
```

Output:

```
Hello, World!
Welcome to Nexora!
```

## Running Nexora Code

### Run a file

```bash
nexora run filename.nx
```

### Start the REPL

```bash
nexora repl
```

The interactive REPL allows you to type Nexora code and see results immediately:

```
Nexora REPL v0.1.0
nexora> print "Hello!"
Hello!
nexora> 2 + 3
5
nexora> exit
Goodbye!
```

### REPL Commands

| Command | Description |
|---------|-------------|
| `exit` or `quit` | Exit the REPL |
| `help` | Show help message |
| `clear` | Clear the screen |
| `history` | Show command history |

## Creating a Project

Use the `new` command to scaffold a new Nexora project:

```bash
nexora new my-project
```

This creates:

```
my-project/
├── src/
│   └── main.nx
├── tests/
├── nexora.json
└── README.md
```

Run your project:

```bash
cd my-project
nexora run src/main.nx
```

## Next Steps

- [Syntax Reference](syntax.md) — Learn the language syntax
- [Variables & Types](variables.md) — Declare variables and work with types
- [Functions](functions.md) — Write reusable functions
