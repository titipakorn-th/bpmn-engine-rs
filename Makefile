.PHONY: test test-coverage clean build

# Run all tests
test:
	cargo test

# Run tests with coverage
test-coverage:
	cargo llvm-cov --all-features --lcov --output-path lcov.info --workspace
	@echo "Coverage report generated: lcov.info"

# Check coverage threshold (100%)
check-coverage:
	@coverage=$$(cargo llvm-cov --all-features --summary-only | grep -oP 'Total.*\K\d+\.\d+' | head -1); \
	echo "Coverage: $${coverage}%"; \
	if (( $$(echo "$${coverage} < 100.0" | bc -l) )); then \
		echo "ERROR: Coverage is below 100% threshold"; \
		exit 1; \
	fi

# Build project
build:
	cargo build

# Clean build artifacts
clean:
	cargo clean

# Run linter
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

