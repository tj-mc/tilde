# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Tails, please report it responsibly:

### How to Report

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Send an email to the maintainer with details about the vulnerability
3. Include as much information as possible:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if you have them)

### What to Expect

- **Response Time**: We aim to acknowledge security reports within 48 hours
- **Investigation**: We will investigate and validate the reported vulnerability
- **Fix Timeline**: Critical vulnerabilities will be addressed as quickly as possible
- **Disclosure**: We will coordinate responsible disclosure once a fix is available

### Security Considerations

Tails is a scripting language interpreter that can:
- Execute system commands through shell integration
- Make HTTP requests to external services
- Read and write files through script execution

When using Tails:
- Only run trusted scripts from known sources
- Be cautious with scripts that make network requests
- Review scripts that interact with the file system
- Consider running untrusted scripts in sandboxed environments

### Safe Usage Guidelines

1. **Script Review**: Always review scripts before execution
2. **Trusted Sources**: Only download scripts from trusted repositories
3. **Network Awareness**: Be mindful of scripts that make external requests
4. **File Permissions**: Understand what file operations a script performs
5. **Environment Isolation**: Use containers or VMs for untrusted code

Thank you for helping keep Tails and its users safe!