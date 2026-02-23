
# Default target
.PHONY: all build build-release docs format
all: build

build:
	@echo "Building Rust project..."
	@cargo build 

build-release:
	@echo "Building Rust project in release mode..."
	@cargo build --release

docs:
	@echo "Opening generated docs"
	@cargo doc
	@cargo doc --open

format:
	@echo "🎨 Formatting..."
	@cargo fmt