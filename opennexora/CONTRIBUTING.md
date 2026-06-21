# Contributing to OpenNexora

Thank you for your interest in contributing to the OpenNexora Foundation! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project adheres to the [OpenNexora Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [conduct@opennexora.org](mailto:conduct@opennexora.org).

## Getting Started

1. **Familiarize Yourself**: Read the [OpenNexora Foundation README](README.md) to understand our mission and projects.
2. **Join the Community**: Join our [Discord server](https://discord.gg/opennexora) to connect with other contributors.
3. **Find an Issue**: Look for issues labeled "good first issue" or "help wanted" in our GitHub repositories.
4. **Set Up Your Environment**: Follow the [Development Setup](#development-setup) instructions below.

## How to Contribute

### Code Contributions

1. **Find or Create an Issue**: Before starting work, ensure there's an issue tracking your proposed change. If not, create one.
2. **Fork the Repository**: Fork the relevant repository to your GitHub account.
3. **Create a Branch**: Create a descriptive branch name (e.g., `feature/add-http2-support`).
4. **Make Your Changes**: Follow our [Coding Standards](#coding-standards).
5. **Write Tests**: Ensure your changes are covered by tests.
6. **Update Documentation**: If your change affects user-facing behavior, update the documentation.
7. **Submit a Pull Request**: Follow our [Pull Request Process](#pull-request-process).

### Documentation Contributions

Documentation is just as important as code! You can contribute by:
- Fixing typos or grammatical errors
- Adding examples or tutorials
- Improving existing documentation
- Translating documentation to other languages

### Community Contributions

- **Answer Questions**: Help others in Discord or GitHub issues
- **Report Bugs**: File detailed bug reports with reproduction steps
- **Suggest Features**: Propose new features with use cases
- **Review Pull Requests**: Help review code from other contributors
- **Organize Events**: Help organize meetups, workshops, or conferences

## Development Setup

### Prerequisites

- [Nexora Runtime](https://nexora.dev/download) v0.4 or later
- [nxm Package Manager](https://nexora.dev/nxm) (included with Nexora)
- Git
- A code editor (we recommend VS Code with the [Nexora extension](https://marketplace.visualstudio.com/items?itemName=OpenNexora.nexora))

### Setting Up a Repository

```bash
# Clone the repository
git clone https://github.com/opennexora/repository-name.git

# Navigate to the repository
cd repository-name

# Install dependencies
npm install

# Run tests to ensure everything works
npm test
```

### Project Structure

Most OpenNexora projects follow this structure:

```
repository-name/
├── src/                 # Source code
│   ├── core/           # Core functionality
│   ├── utils/          # Utility functions
│   └── index.nx        # Main entry point
├── tests/              # Test files
├── docs/               # Documentation
├── examples/           # Usage examples
├── package.nx          # Project configuration
└── README.md           # Project documentation
```

## Pull Request Process

### Before Submitting

1. **Ensure Quality**:
   - All tests pass
   - Code follows our [Coding Standards](#coding-standards)
   - Documentation is updated
   - No new warnings or errors

2. **Prepare Your PR**:
   - Write a clear title and description
   - Reference related issues
   - Include screenshots for UI changes
   - Add changelog entry if applicable

### PR Template

```markdown
## Description

Brief description of the changes.

## Related Issues

Fixes #123

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist

- [ ] My code follows the project's coding standards
- [ ] I have updated the documentation accordingly
- [ ] I have added tests to cover my changes
- [ ] All new and existing tests pass
- [ ] I have performed a self-review of my code
```

### Review Process

1. **Automated Checks**: CI/CD will run tests and checks automatically
2. **Code Review**: At least two maintainers must approve your PR
3. **Address Feedback**: Respond to review comments and make necessary changes
4. **Merge**: Once approved, a maintainer will merge your PR

## Coding Standards

### General Principles

- **Readability**: Code should be easy to read and understand
- **Simplicity**: Prefer simple solutions over complex ones
- **Consistency**: Follow existing patterns in the codebase
- **Documentation**: Write clear comments and documentation

### Nexora-Specific Guidelines

```nx
// Use descriptive variable names
let userCount = 0;

// Write functions that do one thing well
fn calculateTotalPrice(items: Array<Item>) -> Number {
    return items.reduce((sum, item) => sum + item.price, 0);
}

// Use meaningful comments for complex logic
// This algorithm implements the quicksort pattern
// for efficient array sorting in O(n log n) time
fn quicksort(arr: Array<Number>) -> Array<Number> {
    // Implementation
}

// Handle errors explicitly
try {
    let result = await fetchData();
} catch (error) {
    console.error("Failed to fetch data:", error);
    throw error;
}
```

### Testing Standards

- Write tests for all new functionality
- Aim for high test coverage (80%+)
- Use descriptive test names
- Test both success and error cases

```nx
// Example test structure
describe("UserService", () => {
    it("should create a new user with valid data", async () => {
        // Test implementation
    });

    it("should throw error for invalid email", async () => {
        // Test implementation
    });
});
```

## Documentation

### Writing Documentation

- Use clear, concise language
- Include code examples
- Explain why, not just what
- Keep documentation up to date

### Documentation Structure

- **README.md**: Project overview and quick start
- **API Reference**: Detailed API documentation
- **Guides**: Step-by-step tutorials
- **Examples**: Real-world usage examples

## Community

### Getting Help

- **Discord**: Join our [Discord server](https://discord.gg/opennexora) for real-time help
- **GitHub Discussions**: Use GitHub Discussions for longer conversations
- **Stack Overflow**: Tag questions with `opennexora` for community support

### Staying Connected

- **Newsletter**: Subscribe to our [monthly newsletter](https://nexora.dev/newsletter)
- **Blog**: Follow our [blog](https://nexora.dev/blog) for updates
- **Social Media**: Follow us on [Twitter](https://twitter.com/OpenNexora)

## Recognition

We value all contributions! Contributors will be recognized in:
- Our [contributors page](https://nexora.dev/contributors)
- Release notes for significant contributions
- Annual contributor appreciation events

## Questions?

If you have questions about contributing, feel free to ask in our [Discord server](https://discord.gg/opennexora) or open a [GitHub Discussion](https://github.com/opennexora/.github/discussions).

Thank you for contributing to OpenNexora! 🚀