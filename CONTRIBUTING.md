# Contributing to Soroban Security Guard

Thank you for your interest in contributing to Soroban Security Guard! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Contributing Guidelines](#contributing-guidelines)
- [Security Rules](#security-rules)
- [Testing](#testing)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic knowledge of Rust and smart contract security

### Setup

1. Fork the repository
2. Clone your fork locally
3. Set up the development environment

```bash
git clone https://github.com/your-username/soroban-security-guard.git
cd soroban-security-guard
make dev-setup
```

### Building and Testing

```bash
# Build the project
make build

# Run tests
make test

# Run linting
make lint

# Run benchmarks
make benchmark
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-fix-name
```

### 2. Make Changes

- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Changes

Use conventional commit messages:

```
feat: add new security rule for X
fix: resolve issue with Y
docs: update user guide for Z
test: add tests for new functionality
refactor: improve code organization
```

### 4. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Create a pull request with:
- Clear description of changes
- Related issue numbers
- Test results
- Documentation updates

## Contributing Guidelines

### Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust idioms and best practices
- Add comments for complex logic
- Use meaningful variable and function names

### Security Rules

When contributing new security rules:

1. **Rule Implementation**
   - Implement the `Rule` trait
   - Provide clear rule name and description
   - Set appropriate severity level
   - Include confidence scoring

2. **Rule Categories**
   - Access Control
   - Arithmetic Safety
   - Reentrancy Protection
   - Token Implementation
   - State Management
   - Custom Patterns

3. **Testing**
   - Add unit tests for the rule
   - Test with vulnerable code examples
   - Test with safe code examples
   - Test edge cases and false positives

### Example Rule Implementation

```rust
pub struct YourCustomRule;

impl Rule for YourCustomRule {
    fn name(&self) -> &str {
        "your_custom_rule"
    }

    fn description(&self) -> &str {
        "Description of what this rule detects"
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        // Implementation of rule logic
        vec![]
    }

    fn is_enabled(&self, config: &ScannerConfig) -> bool {
        config.rules.your_category.enabled
    }
}
```

## Testing

### Unit Tests

- Test individual components
- Test rule implementations
- Test configuration parsing
- Test AST parsing

### Integration Tests

- Test end-to-end scanning
- Test CLI functionality
- Test report generation
- Test with example contracts

### Benchmark Tests

- Test performance of new features
- Compare with baseline performance
- Test memory usage
- Test scalability

### Running Tests

```bash
# Run all tests
make test

# Run specific test
cargo test test_name

# Run tests with coverage
make coverage

# Run benchmarks
make benchmark
```

## Documentation

### Types of Documentation

1. **Code Documentation**
   - Rustdoc comments for public APIs
   - Inline comments for complex logic
   - Example code in documentation

2. **User Documentation**
   - User guide updates
   - New feature documentation
   - Configuration examples

3. **Developer Documentation**
   - Architecture documentation
   - API reference
   - Contributing guidelines

### Documentation Standards

- Use clear, concise language
- Provide examples
- Include troubleshooting information
- Keep documentation up to date

### Writing Documentation

```rust
/// Represents a security rule that can be applied to Soroban contracts.
/// 
/// # Examples
/// 
/// ```
/// use soroban_security_guard::rules::Rule;
/// 
/// let rule = YourCustomRule;
/// assert_eq!(rule.name(), "your_custom_rule");
/// ```
pub trait Rule {
    /// Returns the name of the rule.
    fn name(&self) -> &str;
    
    // ... other methods
}
```

## Security Considerations

### Safe Rule Implementation

- Validate all inputs
- Handle errors gracefully
- Avoid panics in rule execution
- Use safe Rust practices

### Privacy and Data

- No external data transmission
- Local-only analysis
- Respect user privacy
- Handle sensitive data carefully

### Vulnerability Disclosure

If you discover security vulnerabilities:

1. Do not disclose publicly
2. Report privately to maintainers
3. Allow time for fixes
4. Follow responsible disclosure

## Release Process

### Version Management

- Follow semantic versioning
- Update CHANGELOG.md
- Tag releases appropriately
- Update version numbers

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] Performance benchmarks run
- [ ] Security audit completed
- [ ] Release notes prepared

### Publishing

```bash
# Run full test suite
make ci

# Update version
cargo version patch  # or minor/major

# Create release tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# Publish to crates.io
cargo publish
```

## Community

### Getting Help

- GitHub Issues: Report bugs and request features
- GitHub Discussions: Ask questions and share ideas
- Documentation: Reference guides and examples

### Communication Channels

- Issues: Bug reports and feature requests
- Discussions: General questions and ideas
- Pull Requests: Code contributions and reviews

## Recognition

Contributors are recognized in:

- README.md contributors section
- Release notes
- GitHub contributors list
- Annual project reports

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (MIT License).

## Questions?

If you have questions about contributing:

1. Check existing documentation
2. Search existing issues and discussions
3. Create a new discussion or issue
4. Contact maintainers directly

Thank you for contributing to Soroban Security Guard! 🛡️
