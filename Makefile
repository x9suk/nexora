.PHONY: build test clean fmt lint install

# Default target
all: build

# Build all components
build:
	cargo build --release

# Build debug
debug:
	cargo build

# Run all tests
test:
	cargo test --all

# Format code
fmt:
	cargo fmt --all

# Lint code
lint:
	cargo clippy --all

# Clean build artifacts
clean:
	cargo clean

# Install binaries
install:
	cargo install --path cli
	cargo install --path package-manager
	cargo install --path formatter
	cargo install --path linter
	cargo install --path language-server

# Run CLI
run:
	cargo run --package nexora-cli -- repl

# Run specific file
run-file:
	cargo run --package nexora-cli -- run $(FILE)

# Build AI Engine
ai-engine:
	cd ai-engine && pip install -r requirements.txt

# Build VS Code Extension
vscode:
	cd vscode-extension && npm install && npm run compile

# Create new project
new:
	cargo run --package nexora-cli -- new $(NAME)

# Format all files
format-all:
	cargo run --package nxfmt -- format-all .

# Lint all files
lint-all:
	cargo run --package nxlint -- lint-all .

# Help
help:
	@echo "Nexora Build System"
	@echo "=================="
	@echo ""
	@echo "Targets:"
	@echo "  all        - Build all components (default)"
	@echo "  build      - Build release binaries"
	@echo "  debug      - Build debug binaries"
	@echo "  test       - Run all tests"
	@echo "  fmt        - Format Rust code"
	@echo "  lint       - Lint Rust code"
	@echo "  clean      - Clean build artifacts"
	@echo "  install    - Install all binaries"
	@echo "  run        - Start Nexora REPL"
	@echo "  run-file   - Run a specific file (FILE=filename)"
	@echo "  ai-engine  - Build AI Engine"
	@echo "  vscode     - Build VS Code Extension"
	@echo "  new        - Create new project (NAME=projectname)"
	@echo "  format-all - Format all .nx files"
	@echo "  lint-all   - Lint all .nx files"
	@echo "  help       - Show this help message"
