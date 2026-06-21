<div align="center">

# OpenNexora

**A next-generation, high-performance JavaScript/TypeScript runtime built in Rust.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub Sponsors](https://img.shields.io/badge/Sponsor-opennexora-pink)](https://github.com/sponsors/opennexora)

</div>

---

## What is Nexora?

Nexora is a modern JavaScript/TypeScript runtime that combines the best of Node.js and Deno with cutting-edge performance, built-in tooling, and a seamless developer experience. Powered by Rust and async-first architecture, Nexora delivers lightning-fast execution while maintaining full compatibility with the npm ecosystem.

## Features

- **Lightning Fast** - Built on Rust for maximum performance
- **TypeScript First** - Native TypeScript support without configuration
- **npm Compatible** - Full compatibility with npm packages via `nxm`
- **Secure by Default** - Permission-based security model
- **Built-in Tooling** - Linter, formatter, test runner, and bundler included
- **Web Standards** - Fetch, Web Streams, Web Crypto APIs built-in
- **ESM Native** - ECMAScript modules by default

## Organization Repos

| Repository | Description |
|:-----------|:------------|
| [nexora-runtime](https://github.com/opennexora/nexora-runtime) | The core Nexora runtime |
| [nxm](https://github.com/opennexora/nxm) | Nexora package manager |
| [nexora.dev](https://github.com/opennexora/nexora.dev) | Official documentation website |
| [nexora-vscode](https://github.com/opennexora/nexora-vscode) | VS Code extension for Nexora |
| [lodash-nx](https://github.com/opennexora/lodash-nx) | Lodash ported for Nexora |
| [express-nx](https://github.com/opennexora/express-nx) | Express.js ported for Nexora |
| [nexora-http](https://github.com/opennexora/nexora-http) | Built-in HTTP server module |
| [nexora-test](https://github.com/opennexora/nexora-test) | Built-in test runner |

## Quick Start

```bash
# Install Nexora
curl -fsSL https://get.nexora.dev | sh

# Create a new project
nxm init my-project
cd my-project

# Run your app
nx run main.ts
```

## Community

- **Discord**: [discord.gg/opennexora](https://discord.gg/opennexora)
- **Twitter**: [@opennexora](https://twitter.com/opennexora)
- **Blog**: [nexora.dev/blog](https://nexora.dev/blog)

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) to get started.

## License

MIT License - see [LICENSE](LICENSE) for details.
