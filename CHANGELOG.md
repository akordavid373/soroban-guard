# Changelog

All notable changes to Soroban Security Guard will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Soroban Security Guard
- Static analysis engine for Soroban smart contracts
- CLI interface with multiple output formats
- Built-in security rules for common vulnerabilities
- Custom rule support with regex pattern matching
- Comprehensive documentation and examples

### Security Rules
- Access Control: Admin and owner function protection
- Arithmetic: Integer overflow/underflow detection
- Reentrancy: External call pattern analysis
- Token Safety: ERC-20/721 implementation checks
- State Management: Race condition detection

### Output Formats
- Console: Colored terminal output
- JSON: Machine-readable format
- HTML: Interactive web reports
- SARIF: Standard security findings format

### Configuration
- TOML-based configuration files
- Severity threshold filtering
- File pattern inclusion/exclusion
- Rule enable/disable controls

## [0.1.0] - 2024-01-XX

### Added
- Core scanning engine
- AST parser for Rust/Soroban contracts
- Rule execution framework
- CLI interface
- Multi-format reporting
- Configuration management
- Example contracts
- Comprehensive test suite
- Performance benchmarks
- CI/CD pipeline
- Documentation

### Features
- Parallel file scanning
- Memory-efficient parsing
- Extensible rule system
- Custom pattern matching
- Detailed vulnerability reports
- Remediation suggestions
- Confidence scoring

### Security
- Safe execution environment
- No external code transmission
- Local-only analysis
- Configurable data retention

### Documentation
- User guide
- Architecture documentation
- API documentation
- Example contracts
- Best practices guide

### Development
- Unit and integration tests
- Performance benchmarks
- Code quality checks
- Security auditing
- Dependency management

---

## Security Vulnerability Disclosure

If you discover a security vulnerability in Soroban Security Guard, please report it privately to:

- Email: security@soroban-security-guard.dev
- GitHub Security Advisory: https://github.com/soroban-security-guard/soroban-security-guard/security/advisories

Please do not disclose security vulnerabilities publicly until they have been addressed.
