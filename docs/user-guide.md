# Soroban Security Guard User Guide

## Installation

### Prerequisites
- Rust 1.70+ installed
- Soroban SDK (optional, for contract development)

### Install from Crates.io
```bash
cargo install soroban-security-guard
```

### Install from Source
```bash
git clone https://github.com/soroban-security-guard/soroban-security-guard.git
cd soroban-security-guard
cargo install --path .
```

## Quick Start

### Basic Scan
```bash
soroban-security-guard scan ./my_contract.rs
```

### Scan with Custom Configuration
```bash
soroban-security-guard scan ./contracts/ --config my-config.toml --severity low
```

## Configuration

### Generate Default Configuration
```bash
soroban-security-guard init-config
```

### Generate Strict Configuration
```bash
soroban-security-guard init-config --strict
```

### Configuration File Structure
```toml
[scanner]
severity_threshold = "medium"
output_format = "console"
max_depth = 10

exclude_patterns = ["test_*", "mock_*"]
include_patterns = ["*.rs"]

[rules.access_control]
enabled = true
strict_mode = false
check_admin_functions = true
check_owner_functions = true

[rules.arithmetic]
enabled = true
check_overflow = true
check_underflow = true
safe_math_required = false

[rules.reentrancy]
enabled = true
check_external_calls = true
check_state_changes = true
require_checks_effect = true

[rules.token_safety]
enabled = true
check_erc20 = true
check_erc721 = true
check_approve_patterns = true

[rules.state_management]
enabled = true
check_uninitialized_state = true
check_race_conditions = true
check_atomic_operations = true
```

## Commands

### Scan Command
```bash
soroban-security-guard scan [OPTIONS] <PATH>
```

**Options:**
- `--exclude <PATTERNS>`: Exclude files matching patterns
- `--include <PATTERNS>`: Include only files matching patterns
- `--max-depth <DEPTH>`: Maximum directory depth to scan
- `--output <FORMAT>`: Output format (console, json, html, sarif)
- `--output-file <FILE>`: Save output to file
- `--severity <LEVEL>`: Minimum severity level (low, medium, high, critical)
- `--config <FILE>`: Configuration file path
- `--verbose`: Enable verbose output

**Examples:**
```bash
# Basic scan
soroban-security-guard scan ./src/

# Scan with high severity threshold
soroban-security-guard scan ./contracts/ --severity high

# Generate HTML report
soroban-security-guard scan ./src/ --output html --output-file report.html

# Exclude test files
soroban-security-guard scan ./ --exclude "test_*" --exclude "*_test.rs"

# Scan single file with JSON output
soroban-security-guard scan ./contract.rs --output json --output-file results.json
```

### List Rules Command
```bash
soroban-security-guard list-rules [OPTIONS]
```

**Options:**
- `--severity <LEVEL>`: Filter rules by severity
- `--show-disabled`: Include disabled rules

**Examples:**
```bash
# List all enabled rules
soroban-security-guard list-rules

# List only high severity rules
soroban-security-guard list-rules --severity high

# List all rules including disabled
soroban-security-guard list-rules --show-disabled
```

### Init Config Command
```bash
soroban-security-guard init-config [OPTIONS]
```

**Options:**
- `--output <FILE>`: Output configuration file path
- `--strict`: Generate strict configuration

**Examples:**
```bash
# Generate default config
soroban-security-guard init-config

# Generate strict config
soroban-security-guard init-config --strict

# Custom output location
soroban-security-guard init-config --output ./config/my-config.toml
```

### Validate Config Command
```bash
soroban-security-guard validate-config <CONFIG_FILE>
```

**Examples:**
```bash
soroban-security-guard validate-config ./my-config.toml
```

### Version Command
```bash
soroban-security-guard version [OPTIONS]
```

**Options:**
- `--detailed`: Show detailed version information

**Examples:**
```bash
# Basic version
soroban-security-guard version

# Detailed version
soroban-security-guard version --detailed
```

## Output Formats

### Console Output
Default colored terminal output with:
- Severity-based color coding
- Issue locations and descriptions
- Remediation suggestions
- Summary statistics

### JSON Output
Machine-readable format for:
- CI/CD integration
- Automated processing
- Data analysis

**Example:**
```json
{
  "scan_metadata": {
    "scanner_version": "0.1.0",
    "scan_timestamp": "2024-01-01T12:00:00Z",
    "config_used": "default",
    "rules_enabled": ["admin_only_function", "potential_overflow"]
  },
  "scanned_files": [
    {
      "path": "contract.rs",
      "functions_count": 5,
      "structs_count": 2,
      "enums_count": 1,
      "scan_duration": "0.05s",
      "issues_count": 2
    }
  ],
  "issues": [
    {
      "rule_name": "admin_only_function",
      "severity": "High",
      "message": "Function 'set_admin' appears to be admin-only but lacks access control",
      "location": {
        "file_path": "contract.rs",
        "line": 15,
        "column": 0,
        "function": "set_admin"
      },
      "suggestion": "Add access control check at the beginning of the function",
      "confidence": 1.0
    }
  ],
  "summary": {
    "issues_by_severity": {
      "High": 1,
      "Medium": 1
    },
    "issues_by_file": {
      "contract.rs": 2
    },
    "issues_by_rule": {
      "admin_only_function": 1,
      "potential_overflow": 1
    }
  }
}
```

### HTML Output
Interactive web report with:
- Responsive design
- Filterable issue lists
- Severity-based highlighting
- Export functionality

### SARIF Output
Standard Static Analysis Results Interchange Format for:
- GitHub Security tab integration
- Azure DevOps integration
- Other security tools

## Security Rules

### Access Control Rules

#### Admin Only Function
**Severity**: High
**Description**: Detects functions that should be admin-only but lack access control.

**Pattern Matching**:
- Function names containing "admin_", "set_admin", "change_admin"
- Function names containing "upgrade", "migrate", "emergency_"
- Missing access control checks (require, assert, panic!)

**Example Vulnerable Code:**
```rust
pub fn set_admin(env: &Env, new_admin: Address) {
    // No access control - vulnerable!
    env.storage().instance().set(&DataKey::Admin, &new_admin);
}
```

**Example Safe Code:**
```rust
pub fn set_admin(env: &Env, new_admin: Address) {
    let current_admin = Self::get_admin(env);
    require!(current_admin == env.current_contract_address(), "Admin only");
    env.storage().instance().set(&DataKey::Admin, &new_admin);
}
```

#### Owner Only Function
**Severity**: High
**Description**: Detects functions that should be owner-only but lack access control.

**Pattern Matching**:
- Function names containing "owner_", "transfer_ownership", "renounce_ownership"
- Missing ownership verification

### Arithmetic Rules

#### Potential Overflow
**Severity**: Medium
**Description**: Detects potential integer overflow in arithmetic operations.

**Pattern Matching**:
- Addition, multiplication operations without overflow checks
- Missing safe math usage
- Large integer operations

**Example Vulnerable Code:**
```rust
pub unsafe fn add(a: u64, b: u64) -> u64 {
    a + b // No overflow check
}
```

**Example Safe Code:**
```rust
pub fn add(a: u64, b: u64) -> Result<u64, &'static str> {
    a.checked_add(b).ok_or("Overflow detected")
}
```

### Reentrancy Rules

#### Reentrancy Pattern
**Severity**: High
**Description**: Detects potential reentrancy vulnerabilities.

**Pattern Matching**:
- External calls followed by state changes
- Violation of checks-effects-interactions pattern
- State changes after external contract calls

**Example Vulnerable Code:**
```rust
pub fn withdraw(env: &Env, amount: u64, recipient: Address) {
    let balance = Self::get_balance(env, caller);
    env.storage().instance().set(&DataKey::Balance, &(balance - amount));
    
    // External call
    Self::external_transfer(env, recipient, amount);
    
    // State change after external call - vulnerable!
    env.storage().instance().set(&DataKey::Counter, &1);
}
```

**Example Safe Code:**
```rust
pub fn withdraw(env: &Env, amount: u64, recipient: Address) {
    let balance = Self::get_balance(env, caller);
    require!(balance >= amount, "Insufficient balance");
    
    // All state changes before external calls
    env.storage().instance().set(&DataKey::Balance, &(balance - amount));
    env.storage().instance().set(&DataKey::Counter, &1);
    
    // External call at the end
    Self::external_transfer(env, recipient, amount);
}
```

## Custom Rules

### Defining Custom Rules
Add custom rules to your configuration file:

```toml
[[rules.custom_rules]]
name = "hardcoded_address"
pattern = "0x[a-fA-F0-9]{40}"
severity = "medium"
description = "Potential hardcoded address detected"
enabled = true

[[rules.custom_rules]]
name = "debug_statement"
pattern = "println!|dbg!|eprintln!"
severity = "low"
description = "Debug statement found in production code"
enabled = true
```

### Pattern Matching
Custom rules use regular expressions for pattern matching:
- Standard regex syntax
- Case-sensitive by default
- Can match across multiple lines with proper flags

## Best Practices

### Configuration
- Start with default configuration
- Adjust severity threshold based on project requirements
- Enable strict mode for production contracts
- Regularly update custom rules

### Integration
- Include in CI/CD pipelines
- Set up automated reporting
- Configure failure thresholds
- Monitor security trends

### Development
- Scan contracts during development
- Address high-severity issues immediately
- Use custom rules for project-specific patterns
- Document security decisions

## Troubleshooting

### Common Issues

#### Build Errors
**Problem**: Missing Visual Studio C++ build tools on Windows
**Solution**: Install Visual Studio Build Tools with C++ support

#### Configuration Errors
**Problem**: Invalid TOML syntax
**Solution**: Use `validate-config` command to check syntax

#### Performance Issues
**Problem**: Slow scanning on large codebases
**Solution**: 
- Use file filtering patterns
- Increase `max_depth` limit
- Consider parallel processing

#### False Positives
**Problem**: Incorrect rule triggering
**Solution**:
- Disable specific rules in configuration
- Create custom rules with better patterns
- Report false positives to improve tool

### Getting Help
- Check GitHub Issues for known problems
- Review documentation for configuration options
- Use `--verbose` flag for detailed error information
- Validate configuration files before use

## Contributing

### Reporting Issues
- Use GitHub issue tracker
- Provide minimal reproduction examples
- Include configuration and output
- Specify environment details

### Contributing Rules
- Follow existing rule patterns
- Include comprehensive tests
- Document rule behavior
- Update documentation

### Development Setup
```bash
git clone https://github.com/soroban-security-guard/soroban-security-guard.git
cd soroban-security-guard
cargo test
cargo build --release
```
