# Package Manager (nxm)

## Table of Contents

- [Overview](#overview)
- [Commands](#commands)
- [Package Configuration](#package-configuration)
- [Installing Packages](#installing-packages)
- [Managing Dependencies](#managing-dependencies)
- [Publishing Packages](#publishing-packages)

## Overview

Nexora includes a package manager called **nxm** for managing dependencies and publishing packages.

## Commands

| Command | Description |
|---------|-------------|
| `nxm init` | Initialize a new package |
| `nxm install` | Install all dependencies |
| `nxm install <pkg>` | Install a specific package |
| `nxm add <pkg>` | Add a dependency |
| `nxm add <pkg> --dev` | Add a dev dependency |
| `nxm remove <pkg>` | Remove a dependency |
| `nxm update` | Update all packages |
| `nxm update <pkg>` | Update a specific package |
| `nxm list` | List installed packages |
| `nxm search <query>` | Search for packages |
| `nxm info <pkg>` | Show package info |
| `nxm publish` | Publish your package |
| `nxm clean` | Clean package cache |

## Package Configuration

### nexora.json

Each package has a `nexora.json` configuration file:

```json
{
  "name": "my-package",
  "version": "1.0.0",
  "description": "My awesome Nexora package",
  "main": "src/main.nx",
  "author": "Your Name",
  "license": "MIT",
  "dependencies": {
    "http": "^1.0.0",
    "json": "^1.0.0"
  },
  "devDependencies": {
    "test": "^1.0.0"
  },
  "repository": "https://github.com/user/repo",
  "keywords": ["nexora", "utility"]
}
```

### Fields

| Field | Description |
|-------|-------------|
| `name` | Package name (required) |
| `version` | Semantic version (required) |
| `description` | Package description |
| `main` | Entry point file |
| `author` | Package author |
| `license` | License type |
| `dependencies` | Production dependencies |
| `devDependencies` | Development dependencies |
| `repository` | Source repository URL |
| `keywords` | Search keywords |

## Installing Packages

### Initialize a New Package

```bash
nxm init
```

Creates a `nexora.json` with default values.

### Install Dependencies

```bash
# Install all dependencies from nexora.json
nxm install

# Install a specific package
nxm install http

# Install a specific version
nxm install http@1.2.0
```

### Add Dependencies

```bash
# Add production dependency
nxm add http

# Add specific version
nxm add http@1.2.0

# Add dev dependency
nxm add test --dev
```

### Remove Dependencies

```bash
nxm remove http
```

### Update Packages

```bash
# Update all packages
nxm update

# Update specific package
nxm update http
```

## Managing Dependencies

### List Installed Packages

```bash
nxm list
```

Output:
```
http@1.0.0
json@1.0.0
test@1.0.0 (dev)
```

### Search for Packages

```bash
nxm search http
```

### Show Package Info

```bash
nxm info http
```

Output:
```
Name: http
Version: 1.0.0
Description: HTTP client for Nexora
Author: Nexora Team
License: MIT
```

## Publishing Packages

### Prepare for Publishing

1. Ensure `nexora.json` has all required fields
2. Include a README.md
3. Test your package

### Publish

```bash
nxm publish
```

This uploads your package to the Nexora package registry.

### Versioning

Use semantic versioning:

- **Major** (1.0.0 → 2.0.0): Breaking changes
- **Minor** (1.0.0 → 1.1.0): New features
- **Patch** (1.0.0 → 1.0.1): Bug fixes

## Project Structure

```
my-package/
├── nexora.json
├── README.md
├── src/
│   └── main.nx
└── tests/
    └── test_main.nx
```

## Example: Creating a Utility Library

```bash
# Create project
mkdir my-utils
cd my-utils
nxm init
```

### src/main.nx

```nexora
export func clamp(value, min, max) {
    if value < min { return min }
    if value > max { return max }
    return value
}

export func lerp(a, b, t) {
    return a + (b - a) * t
}

export func mapRange(value, inMin, inMax, outMin, outMax) {
    return outMin + (value - inMin) * (outMax - outMin) / (inMax - inMin)
}
```

### nexora.json

```json
{
  "name": "my-utils",
  "version": "1.0.0",
  "description": "Math utility functions for Nexora",
  "main": "src/main.nx",
  "author": "Your Name",
  "license": "MIT"
}
```

### Publish

```bash
nxm publish
```

### Use in Another Project

```bash
nxm add my-utils
```

```nexora
import { clamp, lerp } from "my-utils"

print clamp(15, 0, 10)     // 10
print lerp(0, 100, 0.5)   // 50
```
