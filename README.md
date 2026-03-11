# Soroban Security Guard

An automated invariant scanner for Soroban smart contracts that identifies potential security vulnerabilities and invariant violations.

## Features

- **Static Analysis**: Analyzes Soroban contract source code for security issues
- **Invariant Detection**: Identifies potential violations of common invariants
- **Pattern Matching**: Detects known vulnerability patterns
- **Custom Rules**: Support for custom security rules and invariants
- **Detailed Reports**: Generates comprehensive security reports
- **CLI Interface**: Easy-to-use command-line interface

## Installation

```bash
cargo install soroban-security-guard
```

## Usage

### Basic Scan

```bash
soroban-security-guard scan /path/to/contract
```

### Advanced Options

```bash
soroban-security-guard scan /path/to/contract \
  --severity medium \
  --output json \
  --config custom_rules.toml
```

## Security Checks

The scanner includes checks for:

- **Access Control Issues**: Unauthorized function access
- **Integer Overflow/Underflow**: Arithmetic safety
- **Reentrancy**: Potential reentrancy vulnerabilities
- **Logic Errors**: Contract logic inconsistencies
- **Token Safety**: ERC-20/721 implementation issues
- **State Management**: Unsafe state modifications

## Configuration

Create a `soroban-security-guard.toml` file to customize scanning rules:

```toml
[scanner]
severity_threshold = "medium"
exclude_patterns = ["test_*", "mock_*"]

[rules.access_control]
enabled = true
strict_mode = false

[rules.arithmetic]
enabled = true
check_overflow = true
```

## Output Formats

- **Console**: Colored terminal output
- **JSON**: Machine-readable format
- **HTML**: Detailed web report
- **SARIF**: Standard security findings format

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new security checks
4. Submit a pull request

## License

MIT License - see LICENSE file for details.
