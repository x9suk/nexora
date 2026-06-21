# CLI Reference

## Table of Contents

- [Command Overview](#command-overview)
- [nexora run](#nexora-run)
- [nexora repl](#nexora-repl)
- [nexora new](#nexora-new)
- [nexora version](#nexora-version)
- [nexora fmt](#nexora-fmt)
- [nexora lint](#nexora-lint)
- [nexora test](#nexora-test)
- [nexora explain](#nexora-explain)
- [nexora ai](#nexora-ai)
- [nexora doctor](#nexora-doctor)

## Command Overview

```
nexora <command> [options]

Commands:
  run <file>        Run a Nexora file
  repl              Start the interactive REPL
  new <name>        Create a new project
  version           Show version information
  fmt <path>        Format code
  lint <path>       Lint code
  test [path]       Run tests
  explain <file>    Explain code in plain language
  ai <prompt>       AI-generate code from prompt
  doctor <path>     Check project for issues
  help              Show help message
```

## nexora run

Run a Nexora file:

```bash
nexora run <file.nx>
```

### Examples

```bash
nexora run hello.nx
nexora run src/main.nx
nexora run ../project/app.nx
```

### Options

| Option | Description |
|--------|-------------|
| `<file>` | Path to the `.nx` file to run (required) |

### Exit Codes

- `0` — Successful execution
- `1` — Runtime error

## nexora repl

Start the interactive REPL (Read-Eval-Print Loop):

```bash
nexora repl
```

### REPL Interface

```
Nexora REPL v0.1.0
Type 'exit' or 'quit' to exit, 'help' for help

nexora> print "Hello!"
Hello!
nexora> 2 + 3
5
nexora> let x = 10
nexora> x * 2
20
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

### Features

- Command history (persists between sessions)
- Auto-print expressions
- Multi-line input support
- Error recovery

## nexora new

Create a new Nexora project:

```bash
nexora new <project-name>
```

### Examples

```bash
nexora new my-app
nexora new my-library
```

### Generated Structure

```
my-app/
├── src/
│   └── main.nx
├── tests/
├── nexora.json
└── README.md
```

### Generated Files

**src/main.nx:**
```nexora
print "Hello from Nexora!"
print "Welcome to "
```

**nexora.json:**
```json
{
  "name": "my-app",
  "version": "0.1.0",
  "description": "",
  "main": "src/main.nx",
  "ai": {
    "autocomplete": true,
    "autofix": true,
    "explain": true,
    "optimize": true
  }
}
```

## nexora version

Show version information:

```bash
nexora version
```

Output:
```
Nexora v0.1.0
AI-Native Programming Language
Copyright (c) 2026 Nexora Team
```

## nexora fmt

Format Nexora code:

```bash
nexora fmt <path>
```

### Examples

```bash
nexora fmt hello.nx
nexora fmt src/
```

### Options

| Option | Description |
|--------|-------------|
| `<path>` | File or directory to format |

> **Note:** Formatter is not yet implemented.

## nexora lint

Lint Nexora code:

```bash
nexora lint <path>
```

### Examples

```bash
nexora lint hello.nx
nexora lint src/
```

### Options

| Option | Description |
|--------|-------------|
| `<path>` | File or directory to lint |

> **Note:** Linter is not yet implemented.

## nexora test

Run tests:

```bash
nexora test [path]
```

### Examples

```bash
nexora test
nexora test tests/
nexora test tests/test_main.nx
```

### Options

| Option | Description |
|--------|-------------|
| `[path]` | Test file or directory (default: current directory) |

> **Note:** Test runner is not yet implemented.

## nexora explain

Explain code in plain language:

```bash
nexora explain <file.nx>
```

### Examples

```bash
nexora explain src/main.nx
nexora explain utils.nx
```

### Output

```
AI: Explaining src/main.nx...

This file contains 3 statement(s):

  1. Variable declaration: 'name'
  2. Function 'greet' with parameters: [name]
  3. Expression statement
```

## nexora ai

AI-generate code from a prompt:

```bash
nexora ai "<prompt>"
```

### Examples

```bash
nexora ai "create a hello world function"
nexora ai "write a fibonacci function"
nexora ai "make a web server"
```

> **Note:** AI code generation will be available soon with the AI engine.

## nexora doctor

Check a project for issues:

```bash
nexora doctor <path>
```

### Examples

```bash
nexora doctor .
nexora doctor ../my-project
```

### Output

```
Doctor: Scanning project at ....

  OK: Found nexora.json
  OK: Found src/ directory
  OK: Found 3 .nx file(s)
    OK: main.nx - OK
    OK: utils.nx - OK
    OK: test.nx - OK

Summary:
  Issues: 0
  Warnings: 0

Success: Your project looks healthy!
```

### Checks Performed

- Verifies `nexora.json` exists
- Checks for `src/` directory
- Scans `.nx` files for syntax errors
- Reports issues and warnings

## Global Options

| Option | Description |
|--------|-------------|
| `--help` | Show help message |
| `--version` | Show version |
