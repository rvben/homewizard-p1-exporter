.PHONY: help build run test lint fmt clean docker-build docker-run release check gh-secrets

# Default target
help:
	@echo "Available targets:"
	@echo "  make build        - Build the binary in debug mode"
	@echo "  make release      - Build the binary in release mode"
	@echo "  make run          - Run the exporter (requires HOMEWIZARD_HOST)"
	@echo "  make test         - Run tests"
	@echo "  make lint         - Run clippy linter"
	@echo "  make fmt          - Format code"
	@echo "  make check        - Run format check and linter"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-run   - Run Docker container (requires HOMEWIZARD_HOST)"
	@echo "  make gh-secrets   - Set GitHub Actions secrets from .env file"

# Build debug binary
build:
	cargo build

# Build release binary
release:
	cargo build --release

# Run the exporter
run:
	@if [ -z "$$HOMEWIZARD_HOST" ]; then \
		echo "Error: HOMEWIZARD_HOST environment variable is required"; \
		echo "Usage: HOMEWIZARD_HOST=192.168.1.152 make run"; \
		exit 1; \
	fi
	cargo run

# Run tests
test:
	cargo test --verbose

# Run linter
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting and run linter
check:
	cargo fmt -- --check
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Build Docker image
docker-build:
	docker build -t homewizard-p1-exporter:latest .

# Run Docker container
docker-run:
	@if [ -z "$$HOMEWIZARD_HOST" ]; then \
		echo "Error: HOMEWIZARD_HOST environment variable is required"; \
		echo "Usage: HOMEWIZARD_HOST=192.168.1.152 make docker-run"; \
		exit 1; \
	fi
	docker run -d --rm \
		--name homewizard-p1-exporter \
		-p 9898:9898 \
		-e HOMEWIZARD_HOST=$$HOMEWIZARD_HOST \
		homewizard-p1-exporter:latest

# Set GitHub Actions secrets from .env file
gh-secrets:
	@if [ ! -f .env ]; then \
		echo "Error: .env file not found"; \
		echo "Copy .env.example to .env and fill in your values"; \
		exit 1; \
	fi
	@echo "Setting GitHub Actions secrets from .env file..."
	@export $$(cat .env | grep -v '^#' | xargs) && \
		gh secret set DOCKER_USERNAME --body "$$DOCKER_USERNAME" && \
		gh secret set DOCKER_PASSWORD --body "$$DOCKER_PASSWORD" && \
		gh secret set CRATES_IO_TOKEN --body "$$CRATES_IO_TOKEN"
	@echo "GitHub Actions secrets have been set successfully!"