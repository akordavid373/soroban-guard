# Soroban Security Guard Architecture

## Overview

Soroban Security Guard is a static analysis tool designed to identify security vulnerabilities and invariant violations in Soroban smart contracts. The architecture follows a modular design that allows for extensibility and maintainability.

## Core Components

### 1. Configuration Layer (`src/config.rs`)

**Purpose**: Manage scanner configuration and rule settings.

**Key Features**:
- TOML-based configuration files
- Severity threshold filtering
- File pattern inclusion/exclusion
- Rule enable/disable controls
- Custom rule definitions

**Key Types**:
- `ScannerConfig`: Main configuration structure
- `RulesConfig`: Rule-specific configurations
- `Severity`: Issue severity levels (Low, Medium, High, Critical)

### 2. AST Parser (`src/ast.rs`)

**Purpose**: Parse Rust source code into an Abstract Syntax Tree for analysis.

**Key Features**:
- Function signature extraction
- Struct and enum parsing
- Import and dependency tracking
- Attribute and macro detection
- Type analysis

**Key Types**:
- `ContractAst`: Complete contract representation
- `Function`: Function metadata and body
- `Struct`, `Enum`: Data structure definitions
- `Type`: Type system representation

### 3. Rules Engine (`src/rules.rs`)

**Purpose**: Implement security checks and vulnerability detection rules.

**Key Features**:
- Pluggable rule system
- Built-in security rules
- Custom pattern matching
- Confidence scoring
- Context-aware analysis

**Built-in Rules**:
- Access Control: Admin/owner function protection
- Arithmetic: Overflow/underflow detection
- Reentrancy: External call pattern analysis
- Token Safety: ERC-20/721 implementation checks
- State Management: Race condition detection

### 4. Scanner Core (`src/scanner.rs`)

**Purpose**: Coordinate the scanning process and orchestrate rule execution.

**Key Features**:
- Multi-file scanning
- Parallel processing
- Error handling and recovery
- Progress tracking
- Result aggregation

**Key Types**:
- `SecurityScanner`: Main scanning engine
- `RuleContext`: Analysis context for rules

### 5. Reporting Layer (`src/report.rs`)

**Purpose**: Generate comprehensive security reports in multiple formats.

**Key Features**:
- Multiple output formats (Console, JSON, HTML, SARIF)
- Severity-based filtering
- Statistical summaries
- Vulnerability trending
- Remediation suggestions

**Output Formats**:
- **Console**: Colored terminal output
- **JSON**: Machine-readable format
- **HTML**: Interactive web report
- **SARIF**: Standard security findings format

### 6. CLI Interface (`src/cli.rs`)

**Purpose**: Provide command-line interface for user interaction.

**Key Features**:
- Subcommand structure
- Argument validation
- Help system
- Configuration integration

**Commands**:
- `scan`: Analyze contracts for vulnerabilities
- `list-rules`: Display available security rules
- `init-config`: Generate configuration files
- `validate-config`: Verify configuration syntax
- `version`: Show version information

## Data Flow

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   Config    │───▶│   Scanner    │───▶│   Report    │
└─────────────┘    └──────────────┘    └─────────────┘
                        │
                        ▼
                ┌──────────────┐
                │ AST Parser   │
                └──────────────┘
                        │
                        ▼
                ┌──────────────┐
                │ Rules Engine │
                └──────────────┘
```

## Security Rule Categories

### 1. Access Control
- **Admin Functions**: Functions that should be restricted to administrators
- **Owner Functions**: Functions that should be restricted to contract owners
- **Authorization**: Missing or improper access controls

### 2. Arithmetic Safety
- **Integer Overflow**: Unchecked arithmetic operations
- **Integer Underflow**: Subtraction without bounds checking
- **Safe Math**: Usage of safe arithmetic libraries

### 3. Reentrancy Protection
- **External Calls**: Analysis of external contract interactions
- **State Changes**: Order of operations vulnerability
- **Checks-Effects-Interactions**: Pattern compliance

### 4. Token Implementation
- **ERC-20**: Standard token interface compliance
- **ERC-721**: NFT implementation security
- **Approve Patterns**: Race condition vulnerabilities

### 5. State Management
- **Race Conditions**: Concurrent access issues
- **Atomic Operations**: Transaction safety
- **Initialization**: Proper state setup

## Extensibility

### Custom Rules
Users can define custom security rules using:
- Regular expressions for pattern matching
- Severity levels and confidence scores
- Descriptive messages and suggestions

### Plugin Architecture
The modular design allows for:
- New rule implementations
- Custom output formats
- Additional analysis passes
- Third-party integrations

## Performance Considerations

### Parallel Processing
- Multi-file scanning in parallel
- Rule execution optimization
- Memory-efficient parsing

### Caching
- AST parsing results
- Rule execution caching
- Configuration validation

### Scalability
- Large codebase support
- Incremental scanning
- Memory usage optimization

## Security Considerations

### Safe Execution
- Sandboxed rule execution
- Resource usage limits
- Error isolation

### Data Privacy
- Local-only analysis
- No code transmission
- Configurable data retention

## Testing Strategy

### Unit Tests
- Individual rule testing
- Configuration validation
- AST parsing accuracy

### Integration Tests
- End-to-end scanning
- CLI functionality
- Report generation

### Benchmark Tests
- Performance measurement
- Memory usage analysis
- Scalability validation

## Future Enhancements

### Advanced Analysis
- Data flow analysis
- Control flow analysis
- Symbolic execution

### Machine Learning
- Pattern recognition
- Anomaly detection
- Automated rule generation

### Integration
- CI/CD pipeline support
- IDE plugins
- Cloud-based scanning
