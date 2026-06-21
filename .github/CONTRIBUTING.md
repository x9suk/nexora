# Contributing to Nexora

Thank you for your interest in contributing to Nexora! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Style Guidelines](#style-guidelines)
- [Community](#community)

## Code of Conduct

By participating in this project, you agree to maintain a welcoming, inclusive, and harassment-free environment. Please read our [Code of Conduct](https://nexora.dev/code-of-conduct) before contributing.

## Getting Started

### Finding Something to Work On

- Check out [good first issue](https://github.com/opennexora/nexora-runtime/labels/good%20first%20issue) labels
- Look at [help wanted](https://github.com/opennexora/nexora-runtime/labels/help%20wanted) issues
- Browse the [roadmap](https://nexora.dev/roadmap) for planned features
- Join [Discord](https://discord.gg/opennexora) to discuss ideas

### Types of Contributions

- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: Improvements, translations, examples
- **Testing**: Test coverage, edge cases, integration tests
- **Design**: UI/UX improvements, branding, icons
- **Community**: Helping others, triaging issues, reviewing PRs

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- Node.js 20+ (for documentation website)
- npm or yarn

### Clone the Repository

```bash
git clone https://github.com/opennexora/nexora-runtime.git
cd nexora-runtime
```

### Build Nexora

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Run Benchmarks

```bash
cargo bench
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feat/add-new-api` - New features
- `fix/memory-leak` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/improve-error-handling` - Refactoring
- `test/add-http-tests` - Adding tests

### Commit Messages

Follow conventional commits:

```
feat: add new HTTP/2 support
fix: resolve memory leak in parser
docs: update API documentation
refactor: simplify error handling
test: add integration tests for nxm
chore: update dependencies
```

### Code Style

- Use `rustfmt` for formatting
- Follow Rust naming conventions
- Write meaningful variable and function names
- Add comments for complex logic
- Keep functions small and focused

### Documentation

- Update documentation for new features
- Add examples for public APIs
- Include code samples where helpful
- Fix typos and grammar

## Testing

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/http_test.rs
use nexora_runtime::http;

#[test]
fn test_http_get() {
    // Test code here
}
```

### Writing Good Tests

- Test one thing per test
- Use descriptive test names
- Include edge cases
- Test error conditions
- Avoid testing implementation details

## Submitting Changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write or update tests
5. Update documentation if needed
6. Run `cargo test` and `cargo fmt`
7. Commit with clear message
8. Push to your fork
9. Open a pull request

### PR Description

Include:

- **Summary**: What this PR does
- **Motivation**: Why this change is needed
- **Implementation**: How you implemented it
- **Testing**: What tests you added/ran
- **Documentation**: Any docs updates
- **Breaking Changes**: If applicable

### Review Process

- All PRs require at least one approval
- Address review feedback
- Keep PRs focused and small when possible
- Ensure CI passes before merging

## Style Guidelines

### Rust Code

- Use `rustfmt` default configuration
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable names
- Keep functions under 50 lines when possible
- Prefer `Result` over `panic!`

### Documentation

- Use clear, concise language
- Include code examples
- Explain "why" not just "what"
- Keep paragraphs short
- Use proper grammar and spelling

### Comments

- Explain complex algorithms
- Document safety invariants
- Note performance considerations
- Don't comment obvious code

## Community

### Getting Help

- **Discord**: [discord.gg/opennexora](https://discord.gg/opennexora)
- **GitHub Discussions**: Ask questions and share ideas
- **Stack Overflow**: Use the `nexora` tag

### Stay Updated

- Follow [@opennexora](https://twitter.com/opennexora) on Twitter
- Subscribe to the [blog](https://nexora.dev/blog)
- Watch the repository for updates

## Recognition

Contributors are recognized in:

- The Contributors page on our website
- Release notes
- Annual contributor appreciation posts

Thank you for contributing to Nexora! Your help makes this project better for everyone.
