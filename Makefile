# Soroban Security Guard Makefile

.PHONY: help build test clean install lint benchmark docs

# Default target
help:
	@echo "Soroban Security Guard - Available targets:"
	@echo ""
	@echo "  build     - Build the project in release mode"
	@echo "  test      - Run all tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install the binary locally"
	@echo "  lint      - Run code linting"
	@echo "  benchmark - Run performance benchmarks"
	@echo "  docs      - Generate documentation"
	@echo "  examples  - Test with example contracts"
	@echo ""

# Build the project
build:
	@echo "🔧 Building Soroban Security Guard..."
	cargo build --release
	@echo "✅ Build completed successfully!"

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test --all
	@echo "✅ All tests passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean completed!"

# Install locally
install: build
	@echo "📦 Installing Soroban Security Guard..."
	cargo install --path .
	@echo "✅ Installation completed!"

# Run linting
lint:
	@echo "🔍 Running code linting..."
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --check
	@echo "✅ Linting completed!"

# Run benchmarks
benchmark:
	@echo "📊 Running performance benchmarks..."
	cargo bench
	@echo "✅ Benchmarks completed!"

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open
	@echo "✅ Documentation generated!"

# Test with example contracts
examples: build
	@echo "🧪 Testing with example contracts..."
	./target/release/soroban-security-guard scan examples/vulnerable_contract.rs --output json --output-file vulnerable_report.json
	./target/release/soroban-security-guard scan examples/safe_contract.rs --output json --output-file safe_report.json
	@echo "✅ Example tests completed!"
	@echo "📁 Reports generated: vulnerable_report.json, safe_report.json"

# Development setup
dev-setup:
	@echo "🛠️  Setting up development environment..."
	rustup component add clippy rustfmt
	cargo fetch
	@echo "✅ Development setup completed!"

# Continuous integration target
ci: lint test benchmark
	@echo "✅ CI pipeline completed successfully!"

# Release preparation
release-prep: clean lint test docs
	@echo "✅ Release preparation completed!"

# Check for security vulnerabilities in dependencies
audit:
	@echo "🔒 Auditing dependencies for security vulnerabilities..."
	cargo audit
	@echo "✅ Security audit completed!"

# Update dependencies
update:
	@echo "⬆️  Updating dependencies..."
	cargo update
	@echo "✅ Dependencies updated!"

# Check outdated dependencies
outdated:
	@echo "📋 Checking for outdated dependencies..."
	cargo outdated
	@echo "✅ Dependency check completed!"

# Generate coverage report
coverage:
	@echo "📈 Generating code coverage report..."
	cargo tarpaulin --out Html
	@echo "✅ Coverage report generated in tarpaulin-report.html!"

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

# Check formatting
check-format:
	@echo "🔍 Checking code formatting..."
	cargo fmt --check
	@echo "✅ Code formatting check completed!"

# Full development workflow
dev: format check-format lint test
	@echo "✅ Full development workflow completed!"
