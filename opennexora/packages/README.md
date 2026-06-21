# Official Nexora Packages

The OpenNexora Foundation maintains and curates a collection of official packages for the Nexora ecosystem. These packages are maintained by the foundation to ensure quality, security, and long-term support.

## Package Registry

All official packages are available through the [Nexora Package Registry](https://registry.nexora.dev). Install packages using the nxm package manager:

```bash
# Install a package
npm install @opennexora/package-name

# Or using nxm directly
nxm add @opennexora/package-name
```

## Official Packages

### Core Packages

| Package | Description | Version |
|---------|-------------|---------|
| [@opennexora/runtime](./nexora-runtime/README.md) | Nexora runtime environment | 0.4.2 |
| [@opennexora/stdlib](./nexora-stdlib/README.md) | Standard library | 0.4.0 |
| [@opennexora/compiler](./nexora-compiler/README.md) | Ahead-of-time compiler | 0.1.0 |

### Utility Packages

| Package | Description | Version |
|---------|-------------|---------|
| [@opennexora/lodash-nx](./lodash-nx/README.md) | Utility functions for Nexora | 1.0.0 |
| [@opennexora/http](./nexora-http/README.md) | HTTP client library | 1.2.0 |
| [@opennexora/test](./nexora-test/README.md) | Testing framework | 0.4.1 |

### Web Framework Packages

| Package | Description | Version |
|---------|-------------|---------|
| [@opennexora/express-nx](./express-nx/README.md) | Express.js-style web framework | 1.0.0 |
| [@opennexora/graphql](./nexora-graphql/README.md) | GraphQL implementation | 0.3.0 |

### Database Packages

| Package | Description | Version |
|---------|-------------|---------|
| [@opennexora/postgres](./nexora-postgres/README.md) | PostgreSQL client | 0.9.0 |
| [@opennexora/mongo](./nexora-mongo/README.md) | MongoDB client | 0.8.0 |

## Community Packages

The Nexora ecosystem also includes thousands of community packages. Browse them at [registry.nexora.dev/community](https://registry.nexora.dev/community).

### Popular Community Packages

- **nexora-forms** - Form validation and handling
- **nexora-auth** - Authentication and authorization
- **nexora-cache** - Caching solutions
- **nexora-queue** - Message queue implementation
- **nexora-websocket** - WebSocket support

## Package Quality Standards

All official packages must meet these standards:

### Code Quality
- 90%+ test coverage
- TypeScript/Nexora type definitions
- Comprehensive documentation
- Regular security audits

### Maintenance
- Active maintenance by foundation members
- Regular updates and bug fixes
- Timely security patches
- Long-term support (LTS) versions available

### Documentation
- Clear README with examples
- API documentation
- Migration guides for major versions
- Changelog for all changes

## Package Guidelines

### Creating Official Packages

To propose a new official package:
1. Submit an RFC to the [nexora-rfcs repository](https://github.com/opennexora/rfcs)
2. Get approval from the TSC
3. Create the package under the @opennexora scope
4. Follow the [Package Development Guide](https://nexora.dev/docs/package-development)

### Package Structure

Official packages follow this structure:

```
package-name/
├── src/                 # Source code
│   ├── index.nx        # Main entry point
│   └── ...             # Other modules
├── tests/              # Test files
├── docs/               # Documentation
├── examples/           # Usage examples
├── package.nx          # Package configuration
├── README.md           # Package documentation
├── CHANGELOG.md        # Version history
└── LICENSE             # MIT License
```

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- **Major**: Breaking changes
- **Minor**: New features (backward compatible)
- **Patch**: Bug fixes (backward compatible)

### Publishing

Packages are published automatically when a new version is tagged:
```bash
# Update version
nxm version major|minor|patch

# Publish to registry
npm publish --access public
```

## Security

### Reporting Vulnerabilities

If you discover a security vulnerability in an official package:
1. Do NOT open a public issue
2. Email security@opennexora.org
3. Include detailed reproduction steps
4. Allow 90 days for a fix before public disclosure

### Security Audits

All official packages undergo:
- Regular automated security scans
- Annual manual security audits
- Dependency vulnerability checks
- Supply chain security measures

## Support

### Getting Help

- **Documentation**: [nexora.dev/docs](https://nexora.dev/docs)
- **GitHub Issues**: Report bugs or request features
- **Discord**: Join #package-support channel
- **Stack Overflow**: Tag questions with `opennexora`

### Contributing

We welcome contributions to official packages! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

All official packages are licensed under the [MIT License](../LICENSE) unless otherwise specified.

---

*Official packages are maintained by the OpenNexora Foundation. For more information, visit [nexora.dev/packages](https://nexora.dev/packages).*