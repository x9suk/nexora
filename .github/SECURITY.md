# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within Nexora, please send an email to security@nexora.dev. All security vulnerabilities will be promptly addressed.

**Please do NOT report security vulnerabilities through public GitHub issues.**

### What to Include

When reporting a vulnerability, please include:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 48 hours of report
- **Initial Assessment**: Within 1 week
- **Fix Release**: Depends on severity, typically within 2 weeks
- **Public Disclosure**: After fix is released

## Security Best Practices

When using Nexora, please follow these security best practices:

### Permission Model

Nexora uses a permission-based security model. Always run with minimal permissions:

```bash
# Instead of:
nx run app.ts

# Use specific permissions:
nx run --allow-net --allow-read app.ts
```

### Dependencies

- Regularly audit your dependencies with `nxm audit`
- Keep dependencies updated
- Review dependency source code when possible

### Environment Variables

- Never commit sensitive data to version control
- Use `.env` files for local development only
- Use secure secret management in production

### Network Security

- Use HTTPS for all network requests
- Validate all input data
- Implement proper CORS policies
- Use rate limiting in production

## Security Features

Nexora includes several built-in security features:

- **Permission System**: Fine-grained control over what code can do
- **Sandboxing**: Isolate untrusted code execution
- **Input Validation**: Built-in validation for common attack vectors
- **Secure Defaults**: Safe configuration out of the box

## Contact

For security-related inquiries, contact:

- **Email**: security@nexora.dev
- **PGP Key**: Available on request
- **Discord**: #security channel (for general security discussions only)

## Acknowledgments

We感谢安全研究人员 responsibly disclose vulnerabilities. Contributors will be credited in our security advisories unless they prefer to remain anonymous.
